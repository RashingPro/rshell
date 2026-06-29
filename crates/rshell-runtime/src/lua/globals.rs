use crate::component::Component;
use log::trace;
use mlua::{FromLuaMulti, IntoLua, IntoLuaMulti, Lua, MaybeSend, Result, Table};
use std::fmt::Display;
use std::time::Duration;

pub fn init_globals(lua: &Lua) {
    register_global_async_function(lua, "sleep", async move |_, amount: u64| {
        trace!(target: "lua_globals", "Sleeping for {} milliseconds", amount);
        tokio::time::sleep(Duration::from_millis(amount)).await;
        Ok(())
    });

    register_global_function(lua, "component", move |_, ()| {
        trace!(target: "lua_globals", "Creating component");
        Ok(Component::default())
    });
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
