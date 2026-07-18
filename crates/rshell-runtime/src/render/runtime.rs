use crate::component::Component;
use crate::component::primitive::PrimitiveComponent;
use crate::error::{Error, Result};
use mlua::Lua;

#[derive(Default)]
pub struct RenderRuntime {
    lua: Lua,
    registered_components: Vec<Component>
}

impl RenderRuntime {
    pub fn new(lua: Lua, components: Vec<Component>) -> Self {
        Self {
            lua,
            registered_components: components
        }
    }

    pub async fn run(self) -> Result<()> {
        for component in self.registered_components {
            if let PrimitiveComponent::Unknown = component.primitive() {
                return Err(Error::InvalidComponent {
                    message: "Attempted to render unknown primitive component.".to_owned()
                });
            }
            // TODO: proceed with rendering here
        }

        Ok(())
    }
}
