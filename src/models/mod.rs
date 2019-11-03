mod demographics;
mod promotion;
mod expression_parser;
#[macro_use]
mod create_hashmap;

pub use demographics::*;
pub use promotion::*;

use diesel::{r2d2::ConnectionManager, PgConnection};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

