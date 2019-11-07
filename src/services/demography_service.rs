use crate::models::Demographics;
use crate::server::ApiResult;

pub struct DemographyService;

impl DemographyService {
    pub fn new() -> Self {
        DemographyService {}
    }

    pub fn publish(&self, data: DemographyIn) -> ApiResult<()> {
        unimplemented!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DemographyIn {
    pub city: String,
    pub country: String,
    pub birth_date: String,
}

