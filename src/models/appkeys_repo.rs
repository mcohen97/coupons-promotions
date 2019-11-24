use crate::schema::appkeys::dsl::appkeys;
use crate::schema::promotions::dsl::promotions;
use crate::schema::appkeys::columns::*;
use diesel::prelude::*;
use std::rc::Rc;
use crate::models::{Promotion, AppKey};
use crate::server::{ApiResult, ApiError, Pagination};

#[derive(Clone)]
pub struct AppKeyRepo {
    conn: Rc<crate::models::Connection>
}

impl AppKeyRepo {
    pub fn new(conn: Rc<crate::models::Connection>) -> Self {
        AppKeyRepo { conn }
    }

    pub fn create(&self, promos: &[i32], org_id: String) -> ApiResult<String> {
        let token_ = nanoid::simple();
        self.validate_promotions(promos, &org_id)?;
        self.conn.transaction(|| {
            promos.into_iter()
                .map(|&p| AppKey { promotion_id: p, token: token_.clone(), organization_id: org_id.clone() })
                .map(|p| self.insert_keys(p))
                .collect::<ApiResult<()>>()
        })?;

        Ok(token_)
    }

    fn validate_promotions(&self, promos: &[i32], org_id: &str) -> ApiResult<()> {
        let r_promos: Result<Vec<Promotion>, diesel::result::Error> = promos.into_iter()
            .map(|&p| promotions.find(p).first::<Promotion>(&*self.conn))
            .collect();
        if let Err(diesel::result::Error::DatabaseError(_, _)) = r_promos {
            return Err(ApiError::from("One of the promotion doesnt exists"));
        }
        if let Err(diesel::NotFound) = r_promos {
            return Err(ApiError::from("One of the promotion doesnt exists"));
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

    pub fn get_all(&self, org_id: &str, pag: Pagination) -> ApiResult<Vec<AppKey>> {
        let Pagination { offset, limit } = pag;
        Ok(appkeys
            .filter(organization_id.eq(org_id))
            .offset(offset)
            .limit(limit)
            .load(&*self.conn)?
        )
    }

    pub fn get_promotions_by_token(&self, token_: &str, org_id: &str) -> ApiResult<Vec<i32>> {
        appkeys.filter(token.eq(token_)).first::<AppKey>(&*self.conn)?;

        Ok(appkeys
            .select(promotion_id)
            .filter(organization_id.eq(org_id))
            .filter(token.eq(token_))
            .load(&*self.conn)?
        )
    }

    pub fn find_organization_by_token(&self, token_: &str) -> ApiResult<String> {
        Ok(appkeys
            .select(organization_id)
            .filter(token.eq(token_))
            .first(&*self.conn)?
        )
    }
}