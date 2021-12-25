#[derive(Clone, Debug)]
pub struct GeneralError<T> {
    pub code: String,
    pub description: Option<T>,
}
