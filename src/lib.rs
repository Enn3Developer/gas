#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod data;
mod pages;
mod widgets;

use crate::data::account::Account;
pub use app::NGas;

pub enum NPage {
    Index,
    Login,
    Shop,
}

pub enum NMessage {
    Redirect(NPage),
    Login(Account),
}
