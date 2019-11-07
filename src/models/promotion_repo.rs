use crate::models::{Connection, Promotion, NewPromotion};
use crate::server::{ApiResult, ApiError};
use diesel::prelude::*;
use crate::schema::promotions::dsl::promotions;

pub struct PromotionRepo {
    conn: Box<Connection>
}

impl PromotionRepo {
    pub fn new(conn: Box<Connection>) -> Self {
        PromotionRepo { conn }
    }

    pub fn get(&self) -> ApiResult<Vec<Promotion>> {
        Ok(promotions.load(&*self.conn)?)
    }

    pub fn find(&self, id: i32) -> ApiResult<Promotion> {
        Ok(promotions.find(id).first::<Promotion>(&*self.conn)?)
    }

    pub fn create(&self, promo: &NewPromotion) -> ApiResult<Promotion> {
        Ok(diesel::insert_into(promotions)
            .values(promo)
            .get_result(&*self.conn)?)
    }

    pub fn delete(&self, id: i32) -> ApiResult<bool> {
        let find = promotions.find(id);
        let delete = diesel::delete(find).execute(&*self.conn);

        match delete {
            Err(diesel::NotFound) => Ok(false),
            Err(err) => Err(ApiError::from(err)),
            Ok(_) => Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::r2d2::ConnectionManager;
    use crate::models;
    use crate::models::{PromotionReturn, PromotionType};

    #[test]
    fn crud_test() {
        dotenv::dotenv().ok();
        let url = std::env::var("DATABASE_URL").unwrap();
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool: models::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let conn = pool.get().unwrap();
        let repo = PromotionRepo::new(&conn);
        let new_promo = build_promo();

        let promo = repo.create(&new_promo).unwrap();
        assert_ne!(0, promo.id);

        let fetched = repo.find(promo.id).unwrap().unwrap();
        assert_eq!(promo, fetched);
        assert_eq!(promo.name, fetched.name);

        let deleted = repo.delete(promo.id).unwrap();
        !(deleted);

        let still_exists = repo.find(promo.id).unwrap().is_some();
        assert!(!still_exists);
    }

    #[test]
    fn create_many() {
        dotenv::dotenv().ok();
        let url = std::env::var("DATABASE_URL").unwrap();
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool: models::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let conn = pool.get().unwrap();
        let repo = PromotionRepo::new(&conn);
        let new_promo = build_promo();

        let created = vec![repo.create(&new_promo).unwrap(); 10];
        let fetched = repo.get().unwrap();

        let fetched_same_as_created = created.iter().zip(fetched.iter())
            .all(|(f, s)| f == s);
        assert!(fetched_same_as_created);

        let all_deleted = fetched.iter().all(|p| repo.delete(p.id).unwrap());
        assert!(all_deleted);

        let promos_left = repo.get().unwrap();
        assert_eq!(0, promos_left.len());
    }

    fn build_promo() -> NewPromotion<'static> {
        NewPromotion::new(
            "Promo",
            "if valid_transaction then apply_discount",
            true,
            PromotionReturn::Percentage(10.0),
            PromotionType::Discount,
            1,
        )
    }
}