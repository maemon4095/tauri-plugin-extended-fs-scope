use crate::{error::Error, VariableRegistry, SEPARATOR_PAT};

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

pub struct PatternEncoder<R: VariableRegistry> {
    registry: R,
    buf: String,
}

impl<R: VariableRegistry> PatternEncoder<R> {
    pub fn new(registry: R) -> Self {
        Self {
            registry,
            buf: String::new(),
        }
    }

    pub fn encode(
        &mut self,
        scope: &tauri::FsScope,
        pat: ScopedPattern,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let buf = &mut self.buf;
        buf.clear();

        match pat {
            ScopedPattern::None(s) => buf.push_str(s),
            ScopedPattern::With(name, s) => {
                let Some(prefix) = self.registry.resolve(name) else {
                    return Err(Error::UnrecognizedVariable(name.to_string()))?;
                };

                buf.push_str(&prefix);
                buf.push(std::path::MAIN_SEPARATOR);
                buf.push_str(s);
            }
        }

        scope.allow_file(&buf)?;
        Ok(())
    }
}
