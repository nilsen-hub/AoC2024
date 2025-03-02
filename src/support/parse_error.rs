pub enum ParseError {
    NoData,
    InvalidData,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoData => write!(f, "No data"),
            Self::InvalidData => write!(f, "Invalid data"),
        }
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoData => write!(f, "NoData"),
            Self::InvalidData => write!(f, "InvalidData"),
        }
    }
}
