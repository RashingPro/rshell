use mlua::{Error, FromLua, IntoLua, Lua, Value};
use std::str::FromStr;

#[derive(Clone, Default, Debug)]
pub struct WindowLifetime {
    pub create: WindowLifetimeCreate
}

impl WindowLifetime {
    pub const fn type_name() -> &'static str {
        "WindowLifetime"
    }
}

#[derive(Clone, Default, Debug, strum::Display, strum::EnumString)]
pub enum WindowLifetimeCreate {
    #[default]
    Always,
    Prepare,
    Never
}

impl WindowLifetimeCreate {
    pub const fn type_name() -> &'static str {
        "WindowLifetimeCreate"
    }
}

impl IntoLua for WindowLifetime {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        let table = lua.create_table()?;

        table.set("create", self.create.to_string())?;

        Ok(Value::Table(table))
    }
}

impl FromLua for WindowLifetime {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(Error::FromLuaConversionError {
                from: value.type_name(),
                to: Self::type_name().to_owned(),
                message: Some("Expected table".to_owned())
            });
        };

        let create: String = table.get("create")?;

        Ok(Self {
            create: WindowLifetimeCreate::from_str(&create).map_err(|_| {
                Error::FromLuaConversionError {
                    from: "string",
                    to: WindowLifetimeCreate::type_name().to_owned(),
                    message: Some(format!("Unexpected value: {}", create))
                }
            })?
        })
    }
}
