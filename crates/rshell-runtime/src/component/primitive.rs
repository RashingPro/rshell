use crate::component::window_lifetime::WindowLifetime;
use log::warn;
use mlua::prelude::LuaError;
use mlua::{FromLua, IntoLua, Lua, Table, Value};

#[derive(Clone, Default, Debug, strum::Display, strum::EnumString)]
#[non_exhaustive]
pub enum PrimitiveComponent {
    #[default]
    Unknown,
    Window {
        lifetime: WindowLifetime
    }
}

impl PrimitiveComponent {
    pub const fn type_name() -> &'static str {
        "PrimitiveComponent"
    }

    pub fn lua_api(&self, lua: &Lua, table: Table, private: Table) -> mlua::Result<()> {
        match self {
            Self::Window { .. } => {
                let table_clone = table.clone();
                let private = private.clone();
                table.set(
                    "lifetime",
                    lua.create_function(move |lua, value: Table| {
                        let new_value = WindowLifetime::from_lua(Value::Table(value), lua)?;
                        let primitive: Table = private.raw_get("primitive")?;
                        primitive.set("lifetime", new_value.into_lua(lua)?)?;
                        Ok(table_clone.clone())
                    })?
                )?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl IntoLua for PrimitiveComponent {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        let table = lua.create_table()?;

        table.set("kind", lua.create_string(self.to_string())?)?;

        match self {
            Self::Window { lifetime } => {
                table.set("lifetime", lifetime.into_lua(lua)?)?;
            }
            _ => {}
        }
        Ok(Value::Table(table))
    }
}

impl FromLua for PrimitiveComponent {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: Self::type_name().to_owned(),
                message: Some("Expected table".to_owned())
            });
        };

        let kind: String = table.raw_get("kind")?;

        Ok(match kind.as_str() {
            "Window" => {
                let lifetime = WindowLifetime::from_lua(table.raw_get("lifetime")?, lua)?;
                Self::Window { lifetime }
            }
            "Unknown" => Self::Unknown,
            _ => {
                warn!("Unknown primitive component: {}", kind);
                Self::Unknown
            }
        })
    }
}
