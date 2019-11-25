use crate::schema::appkeys::dsl::appkeys;
use crate::schema::promotions::dsl::promotions;
use crate::schema::appkeys::columns::*;
use diesel::prelude::*;
use std::rc::Rc;
use crate::models::{Promotion, AppKey, AppKeyOut};
use crate::server::{ApiResult, ApiError, Pagination};

#[derive(Clone)]
pub struct AppKeyRepo {
    conn: Rc<crate::models::Connection>
}

impl AppKeyRepo {
    pub fn new(conn: Rc<crate::models::Connection>) -> Self {
        AppKeyRepo { conn }
    }

    pub fn create(&self, promos: &[i32], org_id: String, name_: String) -> ApiResult<AppKeyOut> {
        let token_ = nanoid::simple();
        self.validate_name_not_taken(&org_id, &name_)?;
        self.validate_promotions(promos, &org_id)?;
        self.conn.transaction(|| {
            promos.into_iter()
                .map(|&p| AppKey { promotion_id: p, token: token_.clone(), organization_id: org_id.clone(), name: name_.clone() })
                .map(|p| self.insert_keys(p))
                .collect::<ApiResult<()>>()
        })?;

        Ok(AppKeyOut { token: token_, organization_id: org_id, name: name_, promotions: promos.to_vec() })
    }

    fn validate_name_not_taken(&self, org_id: &str, name_: &str) -> ApiResult<()> {
        let query: Result<AppKey, diesel::result::Error> = appkeys
            .filter(name.eq(name_))
            .filter(organization_id.eq(org_id))
            .first(&*self.conn);

        if let Err(diesel::NotFound) = query {
            Ok(())
        } else {
            Err(ApiError::from("App key name taken"))
        }
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
        let res = diesel::insert_into(appkeys).values(&appkey).get_result::<AppKey>(&*self.conn);

        match res {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) =>
                Err(ApiError::from("Name taken")),
            Err(e) => Err(ApiError::from(e))
        }
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

    pub fn get_all(&self, org_id: &str, pag: Pagination) -> ApiResult<Vec<AppKeyOut>> {
        let Pagination { offset, limit } = pag;
        let keys: Vec<AppKey> = appkeys
            .filter(organization_id.eq(org_id))
            .offset(offset)
            .limit(limit)
            .load(&*self.conn)?;

        Ok(keys.into_iter()
            .map(|k| self.build(k))
            .collect::<ApiResult<Vec<AppKeyOut>>>()?)
    }

    fn build(&self, key: AppKey) -> ApiResult<AppKeyOut> {
        let promotions_ = self.get_promotions_by_token(&key.token, &key.organization_id)?;
        Ok(AppKeyOut {
            promotions: promotions_,
            name: key.name,
            organization_id: key.organization_id,
            token: key.token,
        })
    }

    pub fn get_name(&self, token_: &str, org_id: &str) -> ApiResult<String> {
        Ok(appkeys
            .select(name)
            .filter(token.eq(token_))
            .filter(organization_id.eq(org_id))
            .first(&*self.conn)?
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

    pub fn delete_token(&self, token_: &str, org: &str) -> ApiResult<()> {
        let source = appkeys
            .filter(token.eq(token_))
            .filter(organization_id.eq(org));
        diesel::delete(source).execute(&*self.conn)?;

        Ok(())
    }
}