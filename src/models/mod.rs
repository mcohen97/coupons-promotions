mod demographics;
mod promotion;
mod organization;
mod expression_parser;
#[macro_use]
mod promotion_repo;
mod organization_repo;
mod coupon_uses_repo;
mod coupons_repo;
mod new_promotion;
mod coupon;
mod coupon_uses;

pub use demographics::*;
pub use promotion::*;
pub use coupon::*;
pub use coupon_uses::*;
pub use coupon_uses_repo::*;
pub use promotion_repo::*;
pub use organization_repo::*;
pub use new_promotion::*;
pub use expression_parser::*;
pub use coupons_repo::*;
pub use coupon_uses::*;

use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use chrono::Utc;

// type alias to use in multiple places
pub type DateTime = chrono::DateTime<Utc>;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

