use crate::models::Demographics;
use crate::server::ApiResult;

pub struct DemographyService;

impl DemographyService {
    pub fn new() -> Self {
        DemographyService {}
    }

    pub fn publish(&self, data: DemographyIn) -> ApiResult<()> {
        if let (Some(country), Some(city), Some(birth_date)) =
            (&data.country, &data.city, &data.birth_date)
        {
            let demo_data = Demographics::new(country, city, birth_date)?;
            println!(
                "Published: {}{}{}",
                demo_data.get_country_code(),
                demo_data.get_city_code(),
                demo_data.get_birth_date()
            );
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DemographyIn {
    pub city: Option<String>,
    pub country: Option<String>,
    pub birth_date: Option<String>,
}
