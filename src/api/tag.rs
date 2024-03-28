use poem_openapi::Tags;

#[derive(Tags)]
pub enum ApiTag {
    /// Index page
    Index,
    Login, // define other tags here
}
