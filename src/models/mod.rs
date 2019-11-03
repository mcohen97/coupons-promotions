mod demographics;

pub use demographics::Demographics;
use crate::schema::promotions;
use diesel::{r2d2::ConnectionManager, PgConnection};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "promotions"]
pub struct Promotion {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub active: bool,
    pub return_type: i32,
    pub return_value: i32,
    pub type_: String,
    pub organization_id: i32,
    pub invocations: i32,
    pub negative_responses: i32,
    pub average_response_time: f64,
    pub total_spent: f64,
}

impl Promotion {
    pub fn default() -> Self {
        Promotion {
            id: 1,
            name: "".into(),
            code: "".into(),
            active: false,
            return_type: 0,
            return_value: 0,
            type_: "".into(),
            organization_id: 0,
            invocations: 0,
            negative_responses: 0,
            average_response_time: 0.0,
            total_spent: 0.0,
        }
    }
}