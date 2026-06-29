use mlua::prelude::LuaError;
use mlua::{FromLua, IntoLua, Lua, Value};
use std::str::FromStr;

#[derive(Clone, Default, Debug, strum::Display, strum::EnumString)]
pub enum BuiltInComponent {
    #[default]
    Unknown
}

impl BuiltInComponent {
    pub const fn type_name() -> &'static str {
        "BuiltInComponent"
    }
}

impl IntoLua for BuiltInComponent {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        Ok(Value::String(lua.create_string(self.to_string())?))
    }
}

impl FromLua for BuiltInComponent {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::String(s) => {
                Ok(BuiltInComponent::from_str(&s.to_string_lossy()).unwrap_or_default())
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: BuiltInComponent::type_name().to_owned(),
                message: Some("Expected string".to_owned())
            })
        }
    }
}
