use crate::models::{Connection, Promotion, NewPromotion};
use crate::server::{ApiResult, ApiError, Pagination, PromotionQueries};
use diesel::prelude::*;
use crate::schema::promotions::dsl::promotions;
use crate::schema::promotions::columns::{code, active, organization_id, name, type_, deleted};
use std::rc::Rc;
use diesel::result::DatabaseErrorInformation;

#[derive(Clone)]
pub struct PromotionRepository {
    conn: Rc<Connection>
}

impl PromotionRepository {
    pub fn new(conn: Rc<Connection>) -> Self {
        PromotionRepository { conn }
    }

    pub fn get(&self, org_id: &str, pag: Pagination, query_params: PromotionQueries) -> ApiResult<Vec<Promotion>> {
        let Pagination { offset, limit } = pag;
        let (name_, code_, promotion_type, active_) = query_params.into_params();
        let mut query = promotions
            .into_boxed()
            .filter(deleted.eq(false))
            .filter(organization_id.eq(org_id))
            .offset(offset)
            .limit(limit);

        if let Some(name_) = name_ {
            query = query.filter(name.like(name_));
        }

        if let Some(code_) = code_ {
            query = query.filter(code.like(code_));
        }

        if let Some(promotion_type) = promotion_type {
            query = query.filter(type_.like(promotion_type));
        }

        if let Some(active_) = active_ {
            query = query.filter(active.eq(active_));
        }

        Ok(query.load(&*self.conn)?)
    }

    pub fn find(&self, id: i32, org_id: &str) -> ApiResult<Promotion> {
        Ok(promotions
            .filter(organization_id.eq(org_id))
            .filter(deleted.eq(false))
            .find(id)
            .first::<Promotion>(&*self.conn)?
        )
    }

    pub fn find_by_code(&self, code_: &str, org_id: &str) -> ApiResult<Promotion> {
        Ok(promotions
            .filter(organization_id.eq(org_id))
            .filter(code.eq(code_))
            .filter(deleted.eq(false))
            .first::<Promotion>(&*self.conn)?
        )
    }

    pub fn create(&self, promo: &NewPromotion) -> ApiResult<Promotion> {
        let res = diesel::insert_into(promotions)
            .values(promo)
            .get_result(&*self.conn);

        match res {
            Ok(val) => Ok(val),
            Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, e)) =>
                Err(ApiError::from(Self::friendly_error(e))),
            Err(e) => Err(ApiError::from(e))
        }
    }

    pub fn update(&self, promo: &Promotion) -> ApiResult<()> {
        let find = promotions.filter(deleted.eq(false)).find(promo.id);
        let res: QueryResult<Promotion> = diesel::update(find)
            .set(promo)
            .get_result(&*self.conn);

        match res {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, e)) =>
                Err(ApiError::from(Self::friendly_error(e))),
            Err(e) => Err(ApiError::from(e))
        }
    }

    pub fn delete(&self, id: i32, org_id: &str) -> ApiResult<bool> {
        let find = promotions.filter(organization_id.eq(org_id)).find(id);
        diesel::update(find)
            .set(deleted.eq(true))
            .execute(&*self.conn)?;
        diesel::update(find)
            .set(active.eq(false))
            .execute(&*self.conn)?;

        Ok(true)
    }

    fn friendly_error(err: Box<dyn DatabaseErrorInformation>) -> String {
        let msg = err.message();
        if msg.contains("promotions_code") {
            "Promotion code has been taken".into()
        }
        else if msg.contains("name") {
            "Name has been taken".into()
        }
        else {
            msg.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::r2d2::ConnectionManager;
    use crate::models;
    use crate::models::{PromotionReturn, PromotionType};
    use chrono::Utc;

    #[test]
    fn crud_test() {
        dotenv::dotenv().ok();
        let url = std::env::var("DATABASE_URL").unwrap();
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool: models::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let conn = Rc::new(pool.get().unwrap());
        let org = "TESTING PROMO";
        let repo = PromotionRepository::new(conn);
        let new_promo = build_promo();

        let promo = repo.create(&new_promo).unwrap();
        assert_ne!(0, promo.id);

        let mut fetched = repo.find(promo.id, "TESTING PROMO").unwrap();
        assert_eq!(promo, fetched);
        assert_eq!(promo.name, fetched.name);

        fetched.name = "Another name".into();
        repo.update(&fetched);
        let fetched = repo.find(promo.id, "TESTING PROMO").unwrap();
        assert_eq!("Another name", fetched.name);

        let deleted_ = repo.delete(promo.id, "TESTING PROMO").unwrap();
        assert!(deleted_);

        let still_exists = repo.find(promo.id, "TESTING PROMO").err().is_none();
        assert!(!still_exists);
    }

    fn build_promo() -> NewPromotion {
        NewPromotion::new(
            "Another name".into(),
            "TestingCode_EWFEWFWEFewgew".into(),
            "if valid_transaction then apply_discount".into(),
            true,
            PromotionReturn::Percentage(10.0),
            PromotionType::Discount,
            "TESTING PROMO".into(),
            Utc::now(),
        )
    }
}