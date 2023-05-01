use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct User {
    pub username: String,
    pub language: String,
}
