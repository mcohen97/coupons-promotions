use crate::schema::transactions::dsl::transactions;
use diesel::prelude::*;
use crate::server::{ApiResult, ApiError};
use crate::models::{Connection, Transaction};
use std::rc::Rc;

#[derive(Clone)]
pub struct TransactionRepository {
    conn: Rc<Connection>
}

impl TransactionRepository {
    pub fn new(conn: Rc<Connection>) -> Self {
        TransactionRepository { conn }
    }

    pub fn exists(&self, id: i32) -> ApiResult<bool> {
        let query = transactions.find(id).first::<Transaction>(&*self.conn);

        match query {
            Err(diesel::NotFound) => Ok(false),
            Err(err) => Err(ApiError::from(err)),
            Ok(_) => Ok(true)
        }
    }

    pub fn create(&self, transaction: &Transaction) -> ApiResult<Transaction> {
        Ok(diesel::insert_into(transactions)
            .values(transaction)
            .get_result(&*self.conn)?
        )
    }
}

