use crate::component::Component;
use crate::lua::runtime::ConfigRuntimeCollector;
use log::trace;
use mlua::{FromLuaMulti, IntoLua, IntoLuaMulti, Lua, MaybeSend, Result, Table};
use parking_lot::RwLock;
use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

pub fn init_globals(lua: &Lua, collector: Arc<RwLock<ConfigRuntimeCollector>>) {
    register_global_async_function(lua, "sleep", async move |_, amount: u64| {
        trace!(target: "lua_globals", "Sleeping for {} milliseconds", amount);
        tokio::time::sleep(Duration::from_millis(amount)).await;
        Ok(())
    });

    register_global_function(lua, "component", move |_, ()| {
        trace!(target: "lua_globals", "Creating component");
        Ok(Component::default())
    });

    register_global_function(lua, "render", move |_, root: Component| {
        trace!(target: "lua_globals", "Collecting component to render");
        collector.write().collect_render(root);
        Ok(())
    })
}

pub fn prepare_render_stage(lua: &Lua) -> Result<()> {
    lua.globals().raw_remove("render")?;

    // Calling twice is intentional. See gc_collect function docs.
    lua.gc_collect()?;
    lua.gc_collect()?;
    Ok(())
}

fn register_global_function<F, A, R>(lua: &Lua, name: impl IntoLua + Display + Clone, function: F)
where
    F: Fn(&Lua, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti
{
    register_global(
        &lua.globals(),
        name.clone(),
        lua.create_function(function)
            .unwrap_or_else(|_| panic!("Failed to create global function \"{}\"", name))
    )
}

fn register_global_async_function<F, A, FR, R>(
    lua: &Lua,
    name: impl IntoLua + Display + Clone,
    function: F
) where
    F: Fn(Lua, A) -> FR + MaybeSend + 'static,
    A: FromLuaMulti,
    FR: Future<Output = Result<R>> + MaybeSend + 'static,
    R: IntoLuaMulti
{
    register_global(
        &lua.globals(),
        name.clone(),
        lua.create_async_function(function)
            .unwrap_or_else(|_| panic!("Failed to create global function \"{}\"", name))
    )
}

fn register_global(globals: &Table, key: impl IntoLua, value: impl IntoLua) {
    globals
        .set(key, value)
        .expect("Failed to set global function");
}
