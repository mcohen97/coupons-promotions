use crate::schema::appkeys::dsl::appkeys;
use crate::schema::promotions::dsl::promotions;
use diesel::prelude::*;
use std::rc::Rc;
use crate::models::{Promotion, AppKey};
use crate::server::{ApiResult, ApiError};

#[derive(Clone)]
pub struct AppKeyRepo {
    conn: Rc<crate::models::Connection>
}

impl AppKeyRepo {
    pub fn new(conn: Rc<crate::models::Connection>) -> Self {
        AppKeyRepo { conn }
    }

    pub fn create(&self, promos: &[i32], org_id: String) -> ApiResult<String> {
        let token = nanoid::simple();
        self.validate_promotions(promos, org_id)?;
        self.conn.transaction(|| {
            promos.into_iter()
                .map(|&p| AppKey { promotion_id: p, token: token.clone() })
                .map(|p| self.insert_keys(p))
                .collect::<ApiResult<()>>()
        })?;

        Ok(token)
    }

    fn validate_promotions(&self, promos: &[i32], org_id: String) -> ApiResult<()> {
        let r_promos: Result<Vec<Promotion>, diesel::result::Error> = promos.into_iter()
            .map(|&p| promotions.find(p).first::<Promotion>(&*self.conn))
            .collect();
        if let Err(diesel::result::Error::DatabaseError(_, _)) = r_promos {
            return Err(ApiError::from("One of the promotion doesnt exists"))
        }
        if let Err(diesel::NotFound) = r_promos {
            return Err(ApiError::from("One of the promotion doesnt exists"))
        }
        let promos = r_promos?;
        promos.first().ok_or(ApiError::from("Needs at least 1 promotion"))?;

        let all_have_same_org = promos.iter().all(|p| p.organization_id == org_id);
        if all_have_same_org {
            Ok(())
        } else {
            Err(ApiError::from(format!("All promotions need to be of the organization {}", org_id)))
        }
    }

    fn insert_keys(&self, appkey: AppKey) -> ApiResult<()> {
        diesel::insert_into(appkeys).values(&appkey).get_result::<AppKey>(&*self.conn)?;
        Ok(())
    }

    pub fn validate_token_permits_promotion(&self, promotion: &Promotion, token_: String) -> ApiResult<()> {
        let res = appkeys.find((promotion.id, token_)).first::<AppKey>(&*self.conn);
        if let Err(diesel::NotFound) = res {
            Err(ApiError::from("Invalid app key"))
        } else {
            res?;
            Ok(())
        }
    }
}