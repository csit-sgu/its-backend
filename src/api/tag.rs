use poem_openapi::Tags;

#[derive(Tags)]
pub enum ApiTag {
    /// Index page
    Index,
    /// Books
    Book,
    /// Authors
    Author,
}
