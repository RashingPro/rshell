use crate::component::Component;
use crate::component::builtin::BuiltInComponent;
use crate::error::{Error, Result};
use mlua::Lua;

#[derive(Default)]
pub struct RenderRuntime {
    lua: Lua,
    render_components: Vec<Component>
}

impl RenderRuntime {
    pub fn new(lua: Lua, components: Vec<Component>) -> Self {
        Self {
            lua,
            render_components: components
        }
    }

    pub async fn run(self) -> Result<()> {
        for component in self.render_components {
            if let BuiltInComponent::Unknown = component.builtin() {
                return Err(Error::InvalidComponent {
                    message: "Attempted to render unknown builtin component".to_owned()
                });
            }
            // TODO: proceed with rendering here
        }

        Ok(())
    }
}
