pub mod builtin;

use crate::component::builtin::BuiltInComponent;
use getset::Getters;
use mlua::prelude::LuaError;
use mlua::{FromLua, IntoLua, Lua, Table, Value};

#[derive(Clone, Default, Debug, Getters)]
pub struct Component {
    children: Vec<Component>,
    #[getset(get = "pub")]
    builtin: BuiltInComponent
}

impl Component {
    pub const fn type_name() -> &'static str {
        "Component"
    }
}

impl IntoLua for Component {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        let table = lua.create_table()?;

        let metatable = lua.create_table()?;

        let public = lua.create_table()?;

        {
            let table = table.clone();
            public.set(
                "child",
                lua.create_function(move |_, child: Component| {
                    table.raw_get::<Table>("children")?.push(child)
                })?
            )?;
        }

        {
            let public = public.clone();
            metatable.set(
                "__index",
                lua.create_function(move |_, (_, key): (Table, Value)| -> mlua::Result<Value> {
                    public.raw_get(key)
                })?
            )?;
        }

        table.set("public", public)?;

        table.set_metatable(Some(metatable))?;

        table.set("children", self.children)?;
        table.set("builtin_component_name", self.builtin.into_lua(lua)?)?;

        Ok(Value::Table(table))
    }
}

impl FromLua for Component {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: Component::type_name().to_owned(),
                message: None
            });
        };

        let children = table
            .raw_get("children")
            .map_err(|_| LuaError::FromLuaConversionError {
                from: Value::Nil.type_name(),
                to: "Vec<Component>".to_owned(),
                message: Some("Expected array of children".to_owned())
            })?;

        Ok(Component {
            children,
            builtin: BuiltInComponent::from_lua(table.raw_get("builtin_component_name")?, lua)?
        })
    }
}
