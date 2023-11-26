use std::sync::{Mutex, MutexGuard};

use crate::{VariableRegistry, SEPARATOR_PAT};

pub struct PatternParser;

impl PatternParser {
    pub fn parse<'a>(&self, raw_pat: &'a str) -> ScopedPattern<'a> {
        if !raw_pat.starts_with('$') {
            return ScopedPattern::None(raw_pat);
        }

        let end = raw_pat.find(SEPARATOR_PAT).unwrap_or(raw_pat.len());
        let scope = &raw_pat[1..end];
        let rest = &raw_pat[end..];

        ScopedPattern::With(scope, rest)
    }
}

pub enum ScopedPattern<'a> {
    None(&'a str),
    With(&'a str, &'a str),
}

pub struct PatternResolver {
    registry: Box<dyn VariableRegistry>,
    buf: Mutex<String>,
}

impl PatternResolver {
    pub fn new(registry: Box<dyn VariableRegistry>) -> Self {
        Self {
            registry,
            buf: Mutex::new(String::new()),
        }
    }

    pub fn variables(&self) -> Box<dyn '_ + Iterator<Item = (&str, &str)>> {
        self.registry.variables()
    }

    pub fn resolve<'a>(&'a self, pattern: &str) -> Result<MutexGuard<'a, String>, Error> {
        let mut buf = self.buf.lock().unwrap();
        buf.clear();
        let scoped = PatternParser.parse(pattern);
        match scoped {
            ScopedPattern::None(s) => {
                buf.push_str(s);
            }
            ScopedPattern::With(name, s) => {
                let Some(prefix) = self.registry.resolve(name) else {
                    return Err(Error::UnrecognizedVariable(name.to_string()));
                };

                buf.push_str(&prefix);
                buf.push(std::path::MAIN_SEPARATOR);
                buf.push_str(s);
            }
        };

        Ok(buf)
    }
}

#[derive(Debug)]
pub enum Error {
    UnrecognizedVariable(String),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnrecognizedVariable(name) => {
                write!(f, "Scope pattern contains unrecognized variable: {}", name)
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
