use crate::component::Component;
use crate::component::primitive::PrimitiveComponent;
use crate::component::window_lifetime::WindowLifetimeCreate;
use crate::error::{Error, Result};
use log::trace;
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
            match component.primitive() {
                PrimitiveComponent::Unknown => {
                    return Err(Error::InvalidComponent {
                        message: "Attempted to render unknown primitive component.".to_owned()
                    });
                }
                PrimitiveComponent::Window { lifetime } => {
                    match lifetime.create {
                        WindowLifetimeCreate::Always => {
                            trace!("Encountered with Always lifetime create policy. Rendering.");
                            // TODO: render window right away
                        }
                        WindowLifetimeCreate::Prepare => {
                            trace!("Encountered with Prepare lifetime create policy. Preparing.");
                            // TODO: prepare window for rendering
                        }
                        WindowLifetimeCreate::Never => {
                            trace!("Encountered with Never lifetime create policy. Skipping.");
                            continue;
                        }
                    }
                }
                _ => {
                    return Err(Error::InvalidComponent {
                        message: format!("Can not render component \"{}\"", component.primitive())
                    });
                }
            }
        }

        Ok(())
    }
}
