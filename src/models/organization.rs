use crate::schema::organizations;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[table_name = "organizations"]
pub struct Organization {
    pub id: String
}