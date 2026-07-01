use colored::Colorize;
use mlua::prelude::LuaError;
use mlua::{ObjectLike, Value};

pub fn format(value: &Value) -> String {
    fn fallback(value: &Value) -> mlua::Result<String> {
        value
            .to_string()
            .map_err(|_| LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "String".to_owned(),
                message: None
            })
    }

    match value {
        Value::Nil => "nil".bright_black().to_string(),
        Value::Boolean(val) => val.to_string().yellow().to_string(),
        Value::Integer(val) => val.to_string().blue().to_string(),
        Value::Number(val) => val.to_string().blue().to_string(),
        Value::Vector(vec) => format!(
            "({})",
            [vec.x(), vec.y(), vec.z(), vec.w()]
                .iter()
                .map(|i| i.to_string().blue().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ),
        Value::String(val) => val.to_string_lossy(),
        Value::Table(val) => val.to_string().unwrap(), // TODO: improve table formatting
        Value::Error(val) => val.to_string().red().to_string(),
        _ => fallback(value).unwrap()
    }
}
