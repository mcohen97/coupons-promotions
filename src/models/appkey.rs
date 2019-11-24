use crate::schema::appkeys;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Clone)]
#[table_name = "appkeys"]
pub struct AppKey {
    pub promotion_id: i32,
    pub token: String,
    pub organization_id: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppKeyOut {
    pub token: String,
    pub name: String,
    pub organization_id: String,
    pub promotions: Vec<i32>
}