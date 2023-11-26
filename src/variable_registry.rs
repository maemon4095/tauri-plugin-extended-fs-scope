use std::collections::HashMap;

use crate::SEPARATOR_PAT;

pub trait VariableRegistry: 'static + Send + Sync {
    fn resolve(&self, name: &str) -> Option<&str>;
    fn variables(&self) -> Box<dyn '_ + Iterator<Item = (&str, &str)>>;
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

    fn variables(&self) -> Box<dyn '_ + Iterator<Item = (&str, &str)>> {
        Box::new(self.map.iter().map(|(k, v)| (k.as_str(), v.as_str())))
    }
}

#[derive(Debug)]
pub enum Error {
    ConflictVariable(String),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConflictVariable(name) => {
                write!(
                    f,
                    "Variable registry already contains variable have the name: {}",
                    name
                )
            }
        }
    }
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}
