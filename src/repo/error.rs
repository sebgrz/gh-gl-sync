use std::fmt::Display;

use git2::Error;

#[derive(Debug)]
pub struct RepositoryError(pub String);

pub trait ToRepositoryError: Display {
    fn to_repository_error(&self) -> RepositoryError {
        RepositoryError(self.to_string())
    }
}

impl ToRepositoryError for Error {}
