#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Out of bounds")]
    OutOfBounds,
}
