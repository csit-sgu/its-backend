use sqlx::PgPool;

use crate::{
    database::BasicRepositoryExt,
    database::Repository,
    model::entity::{Author, Book},
};

pub struct BookRepo {
    pub pool: PgPool,
}

impl Repository for BookRepo {
    fn get_conn_pool(&self) -> PgPool {
        self.pool.clone()
    }
}

impl BasicRepositoryExt<Book> for BookRepo {}

pub struct AuthorRepo {
    pub pool: PgPool,
}

impl Repository for AuthorRepo {
    fn get_conn_pool(&self) -> PgPool {
        self.pool.clone()
    }
}

impl BasicRepositoryExt<Author> for AuthorRepo {}
