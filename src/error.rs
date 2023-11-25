#[derive(Debug)]
pub enum Error {
    UnrecognizedVariable(String),
    ConflictVariable(String),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnrecognizedVariable(name) => {
                write!(f, "Scope pattern contains unrecognized variable: {}", name)
            }
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
