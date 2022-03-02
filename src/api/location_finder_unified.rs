use crate::{api::LocationFinderUnified, error::Error as LibError, ApiMode};
use async_trait::async_trait;
use chrono::NaiveTime;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// API docs: https://developer.dhl.com/api-reference/location-finder#reference-docs-section

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSplsByAddressRequest {
    country_code: CountryCode,
    address_locality: Option<String>,
    postal_code: Option<String>,
    street_address: Option<String>,
    provider_type: Option<ProviderType>,
    location_type: Option<LocationType>,
    service_type: Option<ServiceType>,
    radius: Option<i64>,
    limit: Option<i64>,
    hide_closed_locations: Option<bool>,
}

impl GetSplsByAddressRequest {
    pub fn new(country_code: CountryCode) -> Self {
        GetSplsByAddressRequest {
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

    pub fn address_locality(mut self, value: Option<String>) -> Self {
        self.address_locality = value;

        self
    }

    pub fn postal_code(mut self, value: Option<String>) -> Self {
        self.postal_code = value;

        self
    }

    pub fn street_address(mut self, value: Option<String>) -> Self {
        self.street_address = value;

        self
    }

    pub fn provider_type(mut self, value: Option<ProviderType>) -> Self {
        self.provider_type = value;

        self
    }

    pub fn location_type(mut self, value: Option<LocationType>) -> Self {
        self.location_type = value;

        self
    }

    pub fn service_type(mut self, value: Option<ServiceType>) -> Self {
        self.service_type = value;

        self
    }

    pub fn radius(mut self, value: Option<i64>) -> Self {
        self.radius = value;

        self
    }

    pub fn limit(mut self, value: Option<i64>) -> Self {
        self.limit = value;

        self
    }

    pub fn hide_closed_locations(mut self, value: Option<bool>) -> Self {
        self.hide_closed_locations = value;

        self
    }
}

#[async_trait]
impl LocationFinderUnified for GetSplsByAddressRequest {
    type Response = GetSplsResponse;
    type ResponseNotOk = GetSplResponseNotOk;

    fn url(&self, api_mode: &ApiMode) -> Result<String, LibError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/find-by-address",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/find-by-address",
        };

        let r = format!("{}{}", base_url, serializable_to_url_params(self)?);

        Ok(r)
    }

    async fn send(&self, api_key: &str, api_mode: &ApiMode) -> Result<Self::Response, LibError> {
        let client = reqwest::Client::new();
        let res_bytes = client
            .get(self.url(api_mode)?)
            .header("DHL-API-Key", api_key)
            .send()
            .await?
            .bytes()
            .await?;

        if let Ok(v) = serde_json::from_slice::<GetSplResponseNotOk>(&res_bytes) {
            return Err(LibError::GetSplResponseNotOk {
                status: v.status,
                title: v.title,
                detail: v.detail,
            });
        }

        let res = serde_json::from_slice::<GetSplsResponse>(&res_bytes)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSplsByGeoRequest {
    latitude: f64,
    longitude: f64,
    provider_type: Option<ProviderType>,
    location_type: Option<LocationType>,
    service_type: Option<ServiceType>,
    radius: Option<i64>,
    limit: Option<i64>,
    hide_closed_locations: Option<bool>,
}

impl GetSplsByGeoRequest {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        GetSplsByGeoRequest {
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

    pub fn provider_type(mut self, value: Option<ProviderType>) -> Self {
        self.provider_type = value;

        self
    }

    pub fn location_type(mut self, value: Option<LocationType>) -> Self {
        self.location_type = value;

        self
    }

    pub fn service_type(mut self, value: Option<ServiceType>) -> Self {
        self.service_type = value;

        self
    }

    pub fn radius(mut self, value: Option<i64>) -> Self {
        self.radius = value;

        self
    }

    pub fn limit(mut self, value: Option<i64>) -> Self {
        self.limit = value;

        self
    }

    pub fn hide_closed_locations(mut self, value: Option<bool>) -> Self {
        self.hide_closed_locations = value;

        self
    }
}

#[async_trait]
impl LocationFinderUnified for GetSplsByGeoRequest {
    type Response = GetSplsResponse;
    type ResponseNotOk = GetSplResponseNotOk;

    fn url(&self, api_mode: &ApiMode) -> Result<String, LibError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/find-by-geo",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/find-by-geo",
        };

        Ok(format!("{}{}", base_url, serializable_to_url_params(self)?))
    }

    async fn send(&self, api_key: &str, api_mode: &ApiMode) -> Result<Self::Response, LibError> {
        let client = reqwest::Client::new();
        let res_bytes = client
            .get(self.url(api_mode)?)
            .header("DHL-API-Key", api_key)
            .send()
            .await?
            .bytes()
            .await?;

        if let Ok(v) = serde_json::from_slice::<GetSplResponseNotOk>(&res_bytes) {
            return Err(LibError::GetSplResponseNotOk {
                status: v.status,
                title: v.title,
                detail: v.detail,
            });
        }

        let res = serde_json::from_slice::<GetSplsResponse>(&res_bytes)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSplByKeywordIdRequest {
    keyword_id: String,
    country_code: CountryCode,
    postal_code: String,
}

impl GetSplByKeywordIdRequest {
    pub fn new(keyword_id: String, country_code: CountryCode, postal_code: String) -> Self {
        GetSplByKeywordIdRequest {
            keyword_id,
            country_code,
            postal_code,
        }
    }
}

#[async_trait]
impl LocationFinderUnified for GetSplByKeywordIdRequest {
    type Response = GetSplResponse;
    type ResponseNotOk = GetSplResponseNotOk;

    fn url(&self, api_mode: &ApiMode) -> Result<String, LibError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/find-by-keyword-id",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/find-by-keyword-id",
        };

        Ok(format!("{}{}", base_url, serializable_to_url_params(self)?))
    }

    async fn send(&self, api_key: &str, api_mode: &ApiMode) -> Result<Self::Response, LibError> {
        let client = reqwest::Client::new();
        let res_bytes = client
            .get(self.url(api_mode)?)
            .header("DHL-API-Key", api_key)
            .send()
            .await?
            .bytes()
            .await?;

        if let Ok(v) = serde_json::from_slice::<GetSplResponseNotOk>(&res_bytes) {
            return Err(LibError::GetSplResponseNotOk {
                status: v.status,
                title: v.title,
                detail: v.detail,
            });
        }

        let res = serde_json::from_slice::<GetSplResponse>(&res_bytes)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSplByIdRequest {
    id: String,
}

impl GetSplByIdRequest {
    pub fn new(id: String) -> Self {
        GetSplByIdRequest { id }
    }
}

#[async_trait]
impl LocationFinderUnified for GetSplByIdRequest {
    type Response = GetSplResponse;
    type ResponseNotOk = GetSplResponseNotOk;

    fn url(&self, api_mode: &ApiMode) -> Result<String, LibError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/locations",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/locations",
        };

        Ok(format!("{}/{}", base_url, self.id))
    }

    async fn send(&self, api_key: &str, api_mode: &ApiMode) -> Result<Self::Response, LibError> {
        let client = reqwest::Client::new();
        let res_bytes = client
            .get(self.url(api_mode)?)
            .header("DHL-API-Key", api_key)
            .send()
            .await?
            .bytes()
            .await?;

        if let Ok(v) = serde_json::from_slice::<GetSplResponseNotOk>(&res_bytes) {
            return Err(LibError::GetSplResponseNotOk {
                status: v.status,
                title: v.title,
                detail: v.detail,
            });
        }

        let res = serde_json::from_slice::<GetSplResponse>(&res_bytes)?;

        Ok(res)
    }
}

pub fn serializable_to_url_params<T: Serialize>(serializable: &T) -> Result<String, LibError> {
    let value = serde_json::to_value(serializable)?;

    let mut params = Vec::new();

    if let Value::Object(v) = value {
        let v = v.into_iter().collect::<Vec<(String, Value)>>();
        let mut n = 0;
        for value in v.iter() {
            let prefix = if n > 0 { "&" } else { "" };

            match &value.1 {
                Value::Bool(v) => {
                    params.push(format!("{}{}={}", prefix, value.0.to_case(Case::Camel), v));
                    n += 1;
                }
                Value::Number(v) => {
                    params.push(format!("{}{}={}", prefix, value.0.to_case(Case::Camel), v));
                    n += 1;
                }
                Value::String(v) => {
                    params.push(format!("{}{}={}", prefix, value.0.to_case(Case::Camel), v));
                    n += 1;
                }
                _ => {}
            }
        }

        let mut query = String::new();
        if !params.is_empty() {
            query.push_str("?");
        }

        for param in params {
            query.push_str(&param);
        }

        return Ok(query);
    }

    Ok(String::new())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSplsResponse {
    pub locations: Vec<ServicePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSplResponse {
    pub url: String,
    pub location: ServicePointLocation,
    pub name: String,
    pub place: Place,
    pub opening_hours: Vec<OpeningHours>,
    pub closure_periods: Vec<String>, // TODO - docs say nothing about the format
    pub service_types: Vec<ServiceType>,
    pub average_capacity_day_of_week: Vec<WeekdayCapacity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekdayCapacity {
    pub day_of_week: Weekday,
    pub capacity: Capacity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capacity {
    #[serde(alias = "very-low")]
    VeryLow,
    #[serde(alias = "low")]
    Low,
    #[serde(alias = "high")]
    High,
    // TODO - see if there are more variants
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePoint {
    pub url: String,
    pub location: ServicePointLocation,
    pub name: String,
    pub distance: i64,
    pub place: Place,
    pub opening_hours: Vec<OpeningHours>,
    pub closure_periods: Vec<String>, // TODO - docs say nothing about the format
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePointLocation {
    pub ids: Vec<ServicePointLocationId>,
    pub keyword: String,
    pub keyword_id: String,
    pub r#type: String,
    pub lean_locker: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePointLocationId {
    pub location_id: String,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub address: Address,
    pub geo: Geo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub country_code: String,
    pub postal_code: String,
    pub address_locality: String,
    pub street_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geo {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningHours {
    pub opens: NaiveTime,
    pub closes: NaiveTime,
    pub day_of_week: Weekday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    Parcel,
    Express,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationType {
    Servicepoint,
    Locker,
    Postoffice,
    Postbank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Weekday {
    #[serde(alias = "http://schema.org/Monday", alias = "Monday")]
    Mon,
    #[serde(alias = "http://schema.org/Tuesday", alias = "Tuesday")]
    Tue,
    #[serde(alias = "http://schema.org/Wednesday", alias = "Wednesday")]
    Wed,
    #[serde(alias = "http://schema.org/Thursday", alias = "Thursday")]
    Thu,
    #[serde(alias = "http://schema.org/Friday", alias = "Friday")]
    Fri,
    #[serde(alias = "http://schema.org/Saturday", alias = "Saturday")]
    Sat,
    #[serde(alias = "http://schema.org/Sunday", alias = "Sunday")]
    Sun,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSplResponseNotOk {
    pub status: i64,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Devisions {
    PostAndParcel,
    EcomSolutions,
    Express,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    #[serde(alias = "parcel:pick-up", rename = "parcel:pick-up")]
    ParcelPickUp,
    #[serde(alias = "parcel:drop-off", rename = "parcel:drop-off")]
    ParcelDropOff,
    #[serde(alias = "express:pick-up", rename = "xpress:pick-up")]
    ExpressPickUp,
    #[serde(alias = "express:drop-off", rename = "express:drop-off")]
    ExpressDropOff,
    #[serde(
        alias = "express:drop-off-account",
        rename = "express:drop-off-account"
    )]
    ExpressDropOffAccount,
    #[serde(alias = "express:drop-off-easy", rename = "express:drop-off-easy")]
    ExpressDropOffEasy,
    #[serde(
        alias = "express:drop-off-prelabeled",
        rename = "express:drop-off-prelabeled"
    )]
    ExpressDropOffPrelabeled,
    #[serde(
        alias = "parcel:pick-up-registered",
        rename = "parcel:pick-up-registere"
    )]
    ParcelPickUpRegistered,
    #[serde(
        alias = "parcel:pick-up-unregistered",
        rename = "parcel:pick-up-unregistered"
    )]
    ParcelPickUpUnregistered,
    #[serde(
        alias = "parcel:drop-off-unregistered",
        rename = "parcel:drop-off-unregistered"
    )]
    ParcelDropOffUnregistered,
    #[serde(alias = "letter-service", rename = "letter-service")]
    LetterService,
    #[serde(alias = "postbank", rename = "postbank")]
    Postbank,
    #[serde(alias = "cash-on-delivery", rename = "cash-on-delivery")]
    CashOnDelivery,
    #[serde(alias = "franking", rename = "franking")]
    Franking,
    #[serde(alias = "cash-service", rename = "cash-service")]
    CashService,
    #[serde(alias = "packaging-material", rename = "packaging-material")]
    PackagingMaterial,
    #[serde(alias = "postident", rename = "postident")]
    Postident,
    #[serde(alias = "age-verification", rename = "age-verification")]
    AgeVerification,
    #[serde(alias = "handicapped-access", rename = "handicapped-access")]
    HandicappedAccess,
    #[serde(alias = "parking", rename = "parking")]
    Parking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CountryCode {
    Ae, // United Arab Emirates
    Af, // Afghanistan
    Al, // Albania
    Am, // Armenia
    Ao, // Angola
    Ar, // Argentina
    At, // Austria
    Au, // Australia
    Ba, // Bosnia and Herzegovina
    Bb, // Barbados
    Bd, // Bangladesh
    Be, // Belgium
    Bf, // Burkina Faso
    Bg, // Bulgaria
    Bh, // Bahrain
    Bj, // Benin
    Bm, // Bermuda
    Bn, // Brunei Darussalam
    Bo, // Bolivia
    Br, // Brazil
    Bs, // Bahamas
    Bt, // Bhutan
    Bw, // Botswana
    By, // Belarus
    Ca, // Canada
    Cg, // Congo
    Ch, // Switzerland
    Ci, // Côte d'Ivoire
    Ck, // Cook Islands
    Cl, // Chile
    Cn, // China
    Co, // Colombia
    Cr, // Costa Rica
    Cv, // Cabo Verde
    Cy, // Cyprus
    Cz, // Czechia
    De, // Germany
    Dk, // Denmark
    Do, // Dominican Republic
    Dz, // Algeria
    Ec, // Ecuador
    Ee, // Estonia
    Eg, // Egypt
    Es, // Spain
    Et, // Ethiopia
    Fi, // Finland
    Fj, // Fiji
    Fr, // France
    Gb, // United Kingdom of Great Britain and Northern Ireland
    Ge, // Georgia
    Gf, // French Guiana
    Gg, // Guernsey
    Gh, // Ghana
    Gm, // Gambia
    Gp, // Guadeloupe
    Gr, // Greece
    Gt, // Guatemala
    Gw, // Guinea-Bissau
    Hk, // Hong Kong
    Hn, // Honduras
    Hr, // Croatia
    Ht, // Haiti
    Hu, // Hungary
    Id, // Indonesia
    Ie, // Ireland
    Il, // Israel
    In, // India
    Iq, // Iraq
    Ir, // Iran
    Is, // Iceland
    It, // Italy
    Je, // Jersey
    Jm, // Jamaica
    Jo, // Jordan
    Jp, // Japan
    Ke, // Kenya
    Kg, // Kyrgyzstan
    Kh, // Cambodia
    Ki, // Kiribati
    Km, // Comoros
    Kp, // North Korea
    Kr, // South Korea
    Kv, // Kosovo
    Kw, // Kuwait
    Ky, // Cayman Islands
    Kz, // Kazakhstan
    La, // Laos
    Lb, // Lebanon
    Lk, // Sri Lanka
    Lr, // Liberia
    Ls, // Lesotho
    Lt, // Lithuania
    Lu, // Luxembourg
    Lv, // Latvia
    Ma, // Morocco
    Md, // Moldova
    Mg, // Madagascar
    Mk, // North Macedonia
    Ml, // Mali
    Mm, // Myanmar
    Mn, // Mongolia
    Mo, // Macao
    Mp, // Northern Mariana Islands
    Mq, // Martinique
    Mr, // Mauritania
    Mt, // Malta
    Mu, // Mauritius
    Mv, // Maldives
    Mw, // Malawi
    Mx, // Mexico
    My, // Malaysia
    Mz, // Mozambique
    Na, // Namibia
    Nc, // New Caledonia
    Ng, // Nigeria
    Ni, // Nicaragua
    Nl, // Netherlands
    No, // Norway
    Np, // Nepal
    Nr, // Nauru
    Nu, // Niue
    Nz, // New Zealand
    Om, // Oman
    Pa, // Panama
    Pe, // Peru
    Pf, // French Polynesia
    Pg, // Papua New Guinea
    Ph, // Philippines
    Pk, // Pakistan
    Pl, // Poland
    Pr, // Puerto Rico
    Pt, // Portugal
    Py, // Paraguay
    Qa, // Qatar
    Re, // Réunion
    Ro, // Romania
    Rs, // Serbia
    Ru, // Russian
    Rw, // Rwanda
    Sa, // Saudi Arabia
    Sb, // Solomon Islands
    Sc, // Seychelles
    Sd, // Sudan
    Se, // Sweden
    Sg, // Singapore
    Si, // Slovenia
    Sk, // Slovakia
    Sl, // Sierra Leone
    Sn, // Senegal
    Ss, // South Sudan
    Sv, // El Salvador
    Sy, // Syrian Arab Republic
    Sz, // Eswatini
    Tg, // Togo
    Th, // Thailand
    Tl, // Timor-Leste
    Tn, // Tunisia
    To, // Tonga
    Tr, // Turkey
    Tt, // Trinidad and Tobago
    Tv, // Tuvalu
    Tw, // Taiwan
    Tz, // Tanzania
    Ua, // Ukraine
    Ug, // Uganda
    Us, // United States of America
    Uy, // Uruguay
    Uz, // Uzbekistan
    Ve, // Venezuela
    Vg, // Virgin Islands
    Vi, // Virgin Islands
    Vn, // Viet Nam
    Vu, // Vanuatu
    Ws, // Samoa
    Xc, // Ceuta
    Xm, // Montenegro
    Yt, // Mayotte
    Za, // South Africa
    Zm, // Zambia
    Zw, // Zimbabwe
}
