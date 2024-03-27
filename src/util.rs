use crate::database::{AuthorRepo, BookRepo};
use derive_more::{Display, Error};

/// TODO(vinc3nzo): move to `poem::Data`
pub struct Context {
    pub book_repo: BookRepo,
    pub author_repo: AuthorRepo,
}

#[derive(Debug, Error, Display)]
pub struct EmptyError;
