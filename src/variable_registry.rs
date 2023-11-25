use std::collections::HashMap;

use crate::{error::Error, SEPARATOR_PAT};

pub trait VariableRegistry: 'static + Send {
    fn resolve(&self, name: &str) -> Option<&str>;
}

pub struct DefaultVariableRegistry {
    map: HashMap<String, String>,
}

impl DefaultVariableRegistry {
    pub fn empty() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn with_defaults() -> Result<DefaultVariableRegistry, std::io::Error> {
        let mut map = HashMap::new();
        {
            let mut exe_path = std::env::current_exe()?;
            exe_path.pop();
            map.insert("SELF".to_string(), exe_path.to_string_lossy().into_owned());
        }

        Ok(Self { map })
    }

    pub fn define(&mut self, name: String, mut value: String) -> Result<(), Error> {
        if self.map.get(&name).is_some() {
            return Err(Error::ConflictVariable(name));
        }

        if value.ends_with(SEPARATOR_PAT) {
            value.pop();
        }

        self.map.insert(name, value);

        Ok(())
    }
}

impl VariableRegistry for DefaultVariableRegistry {
    fn resolve(&self, name: &str) -> Option<&str> {
        self.map.get(name).map(|e| e.as_str())
    }
}
