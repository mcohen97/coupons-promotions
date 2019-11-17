use crate::schema::transactions;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[table_name = "transactions"]
pub struct Transaction {
   pub id: i32
}

