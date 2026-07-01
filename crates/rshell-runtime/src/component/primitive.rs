use log::warn;
use mlua::prelude::LuaError;
use mlua::{FromLua, IntoLua, Lua, Value};
use std::str::FromStr;

#[derive(Clone, Default, Debug, strum::Display, strum::EnumString)]
#[non_exhaustive]
pub enum PrimitiveComponent {
    #[default]
    Unknown
}

impl PrimitiveComponent {
    pub const fn type_name() -> &'static str {
        "PrimitiveComponent"
    }
}

impl IntoLua for PrimitiveComponent {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        Ok(Value::String(lua.create_string(self.to_string())?))
    }
}

impl FromLua for PrimitiveComponent {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::String(s) => {
                let s = s.to_string_lossy();
                Ok(PrimitiveComponent::from_str(&s).unwrap_or_else(|_| {
                    warn!("Unknown primitive component: {}", s);
                    PrimitiveComponent::Unknown
                }))
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: PrimitiveComponent::type_name().to_owned(),
                message: Some("Expected string".to_owned())
            })
        }
    }
}
