pub mod index;
pub mod about;
pub mod links;
pub mod dreams;

pub mod error;
pub mod assets;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    index::configure(cfg);
    assets::configure(cfg);
    about::configure(cfg);
    links::configure(cfg);
    dreams::configure(cfg);
}