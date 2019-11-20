use crate::server::ApiResult;
use crate::models;
use crate::services::Services;
use crate::messages::MessageSender;

#[derive(Clone)]
pub struct ServiceFactory {
    pool: models::Pool,
    message_sender: MessageSender,
}

impl ServiceFactory {
    pub fn new(pool: models::Pool, message_sender: MessageSender) -> Self {
        ServiceFactory { pool, message_sender }
    }

    pub fn as_services(&self) -> ApiResult<Services> {
        let conn = self.pool.get()?;
        Ok(Services::new(conn, self.message_sender.clone()))
    }
}
