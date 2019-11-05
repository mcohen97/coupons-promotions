use crate::models::{Connection, Promotion};
use crate::server::{ApiResult, ApiError};
use diesel::prelude::*;
use crate::schema::promotions::dsl::{promotions};

pub struct PromotionRepo<'a> {
    conn: &'a Connection
}

impl<'a> PromotionRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        PromotionRepo { conn }
    }

    pub fn find(&self, id: i32) -> ApiResult<Option<Promotion>> {
        let maybe_promo = promotions.find(id).first::<Promotion>(self.conn);
        if let Err(diesel::NotFound) = maybe_promo {
            return Ok(None)
        }
        Ok(Some(maybe_promo?))
    }

    pub fn create(&self, promo: &Promotion) -> ApiResult<Promotion> {
        Ok(diesel::insert_into(promotions)
            .values(promo)
            .get_result(self.conn)?)
    }

    pub fn delete(&self, id: i32) -> ApiResult<bool> {
        let query = diesel::delete(promotions)
            .filter(crate::schema::promotions::table.id == id)
            .get_result::<Promotion>(self.conn);

        match query {
            Err(diesel::NotFound) => Ok(false),
            Err(err) => Err(ApiError::from(err)),
            Ok(_) => Ok(true)
        }
    }
}