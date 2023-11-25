mod error;
mod patterns;
mod variable_registry;

use patterns::{PatternEncoder, PatternParser};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use variable_registry::{DefaultVariableRegistry, VariableRegistry};

static SEPARATOR_PAT: &'static [char] = &['/', '\\'];

#[derive(serde::Deserialize)]
pub struct Config {
    scope: Vec<String>,
}

pub fn init_with<R: Runtime>(registry: impl VariableRegistry) -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("extended-fs-scope")
        .setup_with_config(|app_handle, config| {
            let scope = app_handle.fs_scope();
            let parser = PatternParser;
            let mut encoder = PatternEncoder::new(registry);

            for raw_pat in config.scope.iter() {
                let scoped = parser.parse(raw_pat);
                encoder.encode(&scope, scoped)?;
            }

            Ok(())
        })
        .build()
}

pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    init_with(DefaultVariableRegistry::with_defaults().unwrap())
}
