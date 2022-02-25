// https://developer.dhl.com/api-reference/location-finder#reference-docs-section

#[derive(Debug, Clone)]
pub enum GetSpl {
    ByAddress {
        country_code: String,
        address_locality: Option<String>,
        postal_code: Option<String>,
        street_address: Option<String>,
        provider_type: Option<String>,
        location_type: Option<String>,
        service_type: Option<String>,
        radius: Option<i64>,
        limit: Option<i64>,
        hide_closed_locations: Option<bool>,
    },
    ByGeo {
        latitude: f64,
        longitude: f64,
        provider_type: Option<String>,
        location_type: Option<String>,
        service_type: Option<String>,
        radius: Option<i64>,
        limit: Option<i64>,
        hide_closed_locations: Option<bool>,
    },
    ByKeywordId {
        keyword_id: String,
        country_code: String,
        postal_code: String,
    },
    ById(String),
}

impl GetSpl {
    pub fn by_address(country_code: String) -> Self {
        GetSpl::ByAddress {
            country_code,
            address_locality: None,
            postal_code: None,
            street_address: None,
            provider_type: None,
            location_type: None,
            service_type: None,
            radius: None,
            limit: None,
            hide_closed_locations: None,
        }
    }

    pub fn by_geo(latitude: f64, longitude: f64) -> Self {
        GetSpl::ByGeo {
            latitude,
            longitude,
            provider_type: None,
            location_type: None,
            service_type: None,
            radius: None,
            limit: None,
            hide_closed_locations: None,
        }
    }

    pub fn by_keyword_id(keyword_id: String, country_code: String, postal_code: String) -> Self {
        GetSpl::ByKeywordId {
            keyword_id,
            country_code,
            postal_code,
        }
    }

    pub fn address_locality(&mut self, value: Option<String>) {
        match self {
            GetSpl::ByAddress {
                address_locality, ..
            } => {
                *address_locality = value;
            }
            _ => {}
        }
    }

    pub fn postal_code(&mut self, value: Option<String>) {
        match self {
            GetSpl::ByAddress { postal_code, .. } => {
                *postal_code = value;
            }
            _ => {}
        }
    }

    pub fn street_address(&mut self, value: Option<String>) {
        match self {
            GetSpl::ByAddress { street_address, .. } => {
                *street_address = value;
            }
            _ => {}
        }
    }

    pub fn provider_type(&mut self, value: Option<String>) {
        match self {
            GetSpl::ByAddress { provider_type, .. } => {
                *provider_type = value;
            }
            GetSpl::ByGeo { provider_type, .. } => {
                *provider_type = value;
            }
            _ => {}
        }
    }

    pub fn location_type(&mut self, value: Option<String>) {
        match self {
            GetSpl::ByAddress { location_type, .. } => {
                *location_type = value;
            }
            GetSpl::ByGeo { location_type, .. } => {
                *location_type = value;
            }
            _ => {}
        }
    }

    pub fn service_type(&mut self, value: Option<String>) {
        match self {
            GetSpl::ByAddress { service_type, .. } => {
                *service_type = value;
            }
            GetSpl::ByGeo { service_type, .. } => {
                *service_type = value;
            }
            _ => {}
        }
    }

    pub fn radius(&mut self, value: Option<i64>) {
        match self {
            GetSpl::ByAddress { radius, .. } => {
                *radius = value;
            }
            GetSpl::ByGeo { radius, .. } => {
                *radius = value;
            }
            _ => {}
        }
    }

    pub fn limit(&mut self, value: Option<i64>) {
        match self {
            GetSpl::ByAddress { limit, .. } => {
                *limit = value;
            }
            GetSpl::ByGeo { limit, .. } => {
                *limit = value;
            }
            _ => {}
        }
    }

    pub fn hide_closed_locations(&mut self, value: Option<bool>) {
        match self {
            GetSpl::ByAddress {
                hide_closed_locations,
                ..
            } => {
                *hide_closed_locations = value;
            }
            GetSpl::ByGeo {
                hide_closed_locations,
                ..
            } => {
                *hide_closed_locations = value;
            }
            _ => {}
        }
    }
}
