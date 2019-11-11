use std::rc::Rc;
use crate::models::Connection;
use crate::server::{ApiResult, ApiError};
use crate::schema::organizations::dsl::organizations;
use diesel::{RunQueryDsl, QueryDsl};
use crate::models::organization::Organization;

pub struct OrganizationRepo {
    conn: Rc<Connection>
}

impl OrganizationRepo {
    pub fn create(&self, id: i32) -> ApiResult<(bool)> {
        let _res: Organization = diesel::insert_into(organizations)
            .values(&Organization { id })
            .get_result(&*self.conn)?;

        Ok(true)
    }

    pub fn delete(&self, id: i32) -> ApiResult<(bool)> {
        let find = organizations.find(id);
        let delete = diesel::delete(find).execute(&*self.conn);

        match delete {
            Err(diesel::NotFound) => Ok(false),
            Err(err) => Err(ApiError::from(err)),
            Ok(_) => Ok(true)
        }
    }

    pub fn exists(&self, id: i32) -> ApiResult<bool> {
        let query = organizations.find(id).first::<Organization>(&*self.conn);

        match query {
            Err(diesel::NotFound) => Ok(false),
            Err(err) => Err(ApiError::from(err)),
            Ok(_) => Ok(true)
        }
    }
}