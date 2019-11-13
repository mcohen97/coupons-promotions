mod demographics;
mod promotion;
mod organization;
mod expression_parser;
#[macro_use]
mod promotion_repo;
mod organization_repo;
mod new_promotion;

pub use demographics::*;
pub use promotion::*;
pub use promotion_repo::*;
pub use organization_repo::*;
pub use new_promotion::*;
pub use expression_parser::*;

use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

