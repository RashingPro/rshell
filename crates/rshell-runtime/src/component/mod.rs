pub mod primitive;

use crate::component::primitive::PrimitiveComponent;
use getset::Getters;
use mlua::prelude::LuaError;
use mlua::{FromLua, IntoLua, Lua, Table, Value};

#[derive(Clone, Default, Debug, Getters)]
pub struct Component {
    children: Vec<Component>,
    #[getset(get = "pub")]
    primitive: PrimitiveComponent
}

impl Component {
    pub const fn type_name() -> &'static str {
        "Component"
    }
}

impl IntoLua for Component {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        let table = lua.create_table()?;

        let private = lua.create_table()?;

        {
            let private = private.clone();
            table.set(
                "child",
                lua.create_function(move |_, child: Component| {
                    private.get::<Table>("children")?.push(child)
                })?
            )?;
        }

        private.set("children", self.children)?;
        private.set("primitive_component_name", self.primitive.into_lua(lua)?)?;

        // TODO: we might want add warning log when accessing it from lua
        table.set("__internal", private)?;

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

        let private = table.raw_get::<Table>("__internal")?;

        let children =
            private
                .raw_get("children")
                .map_err(|_| LuaError::FromLuaConversionError {
                    from: Value::Nil.type_name(),
                    to: "Vec<Component>".to_owned(),
                    message: Some("Expected array of children".to_owned())
                })?;

        Ok(Component {
            children,
            primitive: PrimitiveComponent::from_lua(
                private.raw_get("primitive_component_name")?,
                lua
            )?
        })
    }
}
