#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod pages;
mod widgets;

pub use app::NGas;

pub enum NPage {
    Index,
    Login,
}

pub enum NMessage {
    Redirect(NPage),
}
