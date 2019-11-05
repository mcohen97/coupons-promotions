use crate::server::{ApiResult, ApiError};
use chrono::NaiveDate;
use iso3166_1::CountryCode;
use iata_types::CityCode;
use std::str::FromStr;

const INVALID_COUNTRY: &str = "Invalid country code (Must be iso3166_1)";
const INVALID_DATE: &str = "Invalid date (must be month/day/year)";

pub struct Demographics<'a> {
    country: CountryCode<'a>,
    city: CityCode,
    birth_date: NaiveDate
}

impl<'a> Demographics<'a> {
    pub fn new(country: &str, city: &str, birth_date: &str) -> ApiResult<Self> {
        let country = iso3166_1::alpha2(&country)
        .or(iso3166_1::alpha3(&country))
        .ok_or::<ApiError>(INVALID_COUNTRY.into())?;

        let city = CityCode::from_str(&city)?;
        let birth_date = Self::parse_date(&birth_date)?;

        Ok(Demographics {country, city, birth_date })
    }

    fn parse_date(date: &str) -> ApiResult<NaiveDate> {
        let split: Vec<&str> = date.split("/").collect();
        if split.len() != 3 {
            return Err(INVALID_DATE.into());
        }
        let month = split[0].parse().map_err(|_| ApiError::from(INVALID_DATE))?;
        let day = split[1].parse().map_err(|_| ApiError::from(INVALID_DATE))?;
        let year = split[2].parse().map_err(|_| ApiError::from(INVALID_DATE))?;

        NaiveDate::from_ymd_opt(year, month, day).ok_or(INVALID_DATE.into())
    }

    pub fn get_country_code(&self) -> &str {
        self.country.alpha3
    }

    pub fn get_city_code(&self) -> CityCode {
        self.city
    }

    pub fn get_birth_date(&self) -> NaiveDate {
        self.birth_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID_COUNTRY: &str = "ARG";
    const VALID_CITY: &str = "MDZ";
    const VALID_DATE: &str = "05/04/1998";

    #[test]
    fn valid_data() {
        let result = Demographics::new(VALID_COUNTRY, VALID_CITY, VALID_DATE);
        assert!(result.is_ok());
    }

     #[test]
    fn invalid_country_code() {
        let result = Demographics::new("XYZ", VALID_CITY, VALID_DATE);
        assert!(result.is_err());
    }

    #[test]
    fn country_name_instead_of_code() {
        let result = Demographics::new("Argentina", VALID_CITY, VALID_DATE);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_city_code() {
        let result = Demographics::new(VALID_COUNTRY,"ewew" , VALID_DATE);
        assert!(result.is_err());
    }

    #[test]
    fn city_name_instead_of_code() {
        let result = Demographics::new("Mendoza", VALID_CITY, VALID_DATE);
        assert!(result.is_err());
    }

}