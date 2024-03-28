use derive_more::{Display, Error};

#[allow(unused)]
pub struct Context {
    // declare your repositories here
}

#[derive(Debug, Error, Display)]
pub struct EmptyError;
