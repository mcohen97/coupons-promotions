use crate::schema::appkeys;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Clone)]
#[table_name = "appkeys"]
pub struct AppKey {
    pub promotion_id: i32,
    pub token: String,
    pub organization_id: String,
}