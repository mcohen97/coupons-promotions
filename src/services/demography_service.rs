use crate::models::Demographics;
use std::borrow::Cow;
use crate::messages::{DemographyData};

pub struct DemographyServices;

impl DemographyServices {
    pub fn new() -> Self {
        DemographyServices {}
    }

    pub fn build_demographics_if_valid(&self, data: Option<DemographyIn>) -> (Cow<'static, str>, Option<DemographyData>) {
        match data {
            Some(d) => self.build(d),
            None => ("Demographic data was not sent".into(), None)
        }
    }

    fn build(&self, data: DemographyIn) -> (Cow<'static, str>, Option<DemographyData>) {
        let DemographyIn { country, city, birth_date } = data;
        match Demographics::new(&country, &city, &birth_date) {
            Ok(data) => {
                let data = DemographyData {
                    country: data.get_country_code().to_string(),
                    city: data.get_city_code().as_str().to_string(),
                    birth_date: data.get_birth_date().to_string(),
                };
                ("Demographic data was valid".into(), Some(data))
            }
            Err(e) => (e.get_message(), None)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DemographyIn {
    pub city: String,
    pub country: String,
    pub birth_date: String,
}

