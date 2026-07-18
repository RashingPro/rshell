use crate::component::Component;
use crate::component::primitive::PrimitiveComponent;
use crate::lua::format::format;
use crate::lua::runtime::ConfigRuntimeCollector;
use log::{info, trace};
use mlua::{FromLuaMulti, IntoLua, IntoLuaMulti, Lua, MaybeSend, Result, Table, Value, Variadic};
use parking_lot::RwLock;
use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

pub fn prepare_collecting_stage(
    lua: &Lua,
    collector: Arc<RwLock<ConfigRuntimeCollector>>
) -> Result<()> {
    register_global_async_function(lua, "sleep", async move |_, amount: u64| {
        trace!(target: "lua_globals", "Sleeping for {} milliseconds", amount);
        tokio::time::sleep(Duration::from_millis(amount)).await;
        Ok(())
    })?;

    register_global_function(lua, "print", move |_, args: Variadic<Value>| {
        let s = args.iter().map(format).collect::<Vec<String>>().join(" ");
        info!(target: "config_runtime", "{}", s);
        Ok(())
    })?;

    register_global_function(lua, "component", move |_, ()| {
        trace!(target: "lua_globals", "Creating component");
        Ok(Component::default())
    })?;

    register_global_function(lua, "Window", move |_, ()| {
        trace!(target: "lua_globals", "Creating window");
        Ok(Component::new(PrimitiveComponent::Window {
            lifetime: Default::default()
        }))
    })?;

    register_global_function(lua, "register", move |_, root: Component| {
        trace!(target: "lua_globals", "Registering component");
        collector.write().register_component(root);
        Ok(())
    })?;

    Ok(())
}

pub fn prepare_render_stage(lua: &Lua) -> Result<()> {
    lua.globals().raw_remove("register")?;

    // Calling twice is intentional. See gc_collect function docs.
    lua.gc_collect()?;
    lua.gc_collect()?;
    Ok(())
}

fn register_global_function<F, A, R>(
    lua: &Lua,
    name: impl IntoLua + Display + Clone,
    function: F
) -> Result<()>
where
    F: Fn(&Lua, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti
{
    register_global(&lua.globals(), name.clone(), lua.create_function(function)?)
}

fn register_global_async_function<F, A, FR, R>(
    lua: &Lua,
    name: impl IntoLua + Display + Clone,
    function: F
) -> Result<()>
where
    F: Fn(Lua, A) -> FR + MaybeSend + 'static,
    A: FromLuaMulti,
    FR: Future<Output = Result<R>> + MaybeSend + 'static,
    R: IntoLuaMulti
{
    register_global(
        &lua.globals(),
        name.clone(),
        lua.create_async_function(function)?
    )
}

fn register_global(globals: &Table, key: impl IntoLua, value: impl IntoLua) -> Result<()> {
    globals.set(key, value)
}
