mod pattern_resolver;
mod variable_registry;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use pattern_resolver::{Error as ResolveError, PatternResolver};
pub use variable_registry::{DefaultVariableRegistry, Error as RegistryError, VariableRegistry};

static SEPARATOR_PAT: &'static [char] = &['/', '\\'];

#[derive(serde::Deserialize)]
pub struct Config {
    scope: Vec<String>,
}

pub fn init_with<R: Runtime>(registry: impl VariableRegistry) -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("extended-fs-scope")
        .setup_with_config(|app_handle, config| {
            let scope = app_handle.fs_scope();
            let resolver = PatternResolver::new(Box::new(registry));
            for raw_pat in config.scope.iter() {
                let resolved = resolver.resolve(&raw_pat)?;
                scope.allow_file(resolved.as_str())?;
            }

            app_handle.manage(resolver);

            Ok(())
        })
        .build()
}

pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    init_with(DefaultVariableRegistry::with_defaults().unwrap())
}
