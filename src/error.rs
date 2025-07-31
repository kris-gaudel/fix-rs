#[derive(Debug, thiserror::Error)]
pub enum FixError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Missing field: {0}")]
    MissingField(u32),
    #[error("Invalid tag: {0}")]
    InvalidTag(String),

    #[error("Invalid checksum: expected {expected}, got {actual}")]
    InvalidChecksum { expected: u8, actual: u8 },
    #[error("Body length mismatch: expected {expected}, got {got}")]
    BodyLengthMismatch { expected: usize, got: usize },
    #[error("Unknown message type: {0}")]
    UnknownMsgType(String),
}
