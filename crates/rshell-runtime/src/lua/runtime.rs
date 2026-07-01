use crate::component::Component;
use crate::error::{Error, Result};
use crate::lua::globals::{prepare_collecting_stage, prepare_render_stage};
use mlua::{Lua, LuaOptions, StdLib};
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;

pub struct ConfigRuntime {
    collector: Arc<RwLock<ConfigRuntimeCollector>>,
    lua: Lua,
    config: PathBuf
}

impl ConfigRuntime {
    pub fn new(config: PathBuf) -> Self {
        let lua = Lua::new_with(
            StdLib::MATH | StdLib::TABLE | StdLib::STRING | StdLib::VECTOR,
            LuaOptions::default()
        )
        .expect("Failed to create Lua runtime");

        let collector = Arc::new(RwLock::new(ConfigRuntimeCollector::new()));

        Self {
            collector,
            lua,
            config
        }
    }

    pub async fn run(self) -> Result<(Lua, ConfigRuntimeCollector)> {
        prepare_collecting_stage(&self.lua, self.collector.clone()).map_err(|err| {
            Error::Other {
                message: format!("Error while preparing Lua runtime:\n{}", err)
            }
        })?;

        if let Err(error) = self.lua.load(self.config).exec_async().await {
            return Err(Error::from(error));
        }

        prepare_render_stage(&self.lua).map_err(Error::from)?;

        let Some(arc_inner) = Arc::into_inner(self.collector) else {
            return Err(Error::Other {
                message: "Collector arc contains more than one strong reference. You have saved a \
                          reference to RShell global API somewhere."
                    .to_owned()
            });
        };

        Ok((self.lua, arc_inner.into_inner()))
    }
}

pub struct ConfigRuntimeCollector {
    render_components: Vec<Component>
}

impl ConfigRuntimeCollector {
    pub fn new() -> Self {
        Self {
            render_components: Vec::new()
        }
    }

    pub fn collect_render(&mut self, root: Component) {
        self.render_components.push(root);
    }

    pub fn read(self) -> Vec<Component> {
        self.render_components
    }
}
