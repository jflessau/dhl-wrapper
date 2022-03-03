use crate::error::Error as LibError;
use async_trait::async_trait;
use chrono::NaiveTime;
use convert_case::{Case, Casing};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// API docs: https://developer.dhl.com/api-reference/location-finder#reference-docs-section

pub struct LocationFinderUnifiedApi {
    api_mode: ApiMode,
    api_key: String,
}

pub enum ApiMode {
    Sandbox,
    Production,
}

impl LocationFinderUnifiedApi {
    pub fn new(api_mode: ApiMode, api_key: String) -> Self {
        LocationFinderUnifiedApi { api_mode, api_key }
    }

    pub async fn send<T>(&self, request: T) -> Result<T::Response, LibError>
    where
        T: LocationFinderUnifiedRequest,
        T::Response: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let res_bytes = client
            .get(request.url(&self.api_mode)?)
            .header("DHL-API-Key", &self.api_key)
            .send()
            .await?
            .bytes()
            .await?;

        if let Ok(v) = serde_json::from_slice::<ResponseNotOk>(&res_bytes) {
            return Err(LibError::ResponseNotOk {
                status: v.status,
                title: v.title,
                detail: v.detail,
            });
        }

        let res = serde_json::from_slice::<T::Response>(&res_bytes)?;

        Ok(res)
    }
}

#[async_trait]
pub trait LocationFinderUnifiedRequest {
    type Response;

    fn url(&self, api_mode: &ApiMode) -> Result<String, LibError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseNotOk {
    status: i64,
    title: String,
    detail: String,
}

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
impl LocationFinderUnifiedRequest for GetSplsByAddressRequest {
    type Response = GetSplsResponse;

    fn url(&self, api_mode: &ApiMode) -> Result<String, LibError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/find-by-address",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/find-by-address",
        };

        let r = format!("{}{}", base_url, serializable_to_url_params(self)?);

        Ok(r)
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
impl LocationFinderUnifiedRequest for GetSplsByGeoRequest {
    type Response = GetSplsResponse;

    fn url(&self, api_mode: &ApiMode) -> Result<String, LibError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/find-by-geo",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/find-by-geo",
        };

        Ok(format!("{}{}", base_url, serializable_to_url_params(self)?))
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
impl LocationFinderUnifiedRequest for GetSplByKeywordIdRequest {
    type Response = GetSplResponse;

    fn url(&self, api_mode: &ApiMode) -> Result<String, LibError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/find-by-keyword-id",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/find-by-keyword-id",
        };

        Ok(format!("{}{}", base_url, serializable_to_url_params(self)?))
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
impl LocationFinderUnifiedRequest for GetSplByIdRequest {
    type Response = GetSplResponse;

    fn url(&self, api_mode: &ApiMode) -> Result<String, LibError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/locations",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/locations",
        };

        Ok(format!("{}/{}", base_url, self.id))
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

/// Two-letter country codes (https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CountryCode {
    Ad, // Andorra
    Ae, // United Arab Emirates
    Af, // Afghanistan
    Ag, // Antigua and Barbuda
    Ai, // Anguilla
    Al, // Albania
    Am, // Armenia
    Ao, // Angola
    Aq, // Antarctica
    Ar, // Argentina
    As, // American Samoa
    At, // Austria
    Au, // Australia
    Aw, // Aruba
    Ax, // Åland Islands
    Az, // Azerbaijan
    Ba, // Bosnia and Herzegovina
    Bb, // Barbados
    Bd, // Bangladesh
    Be, // Belgium
    Bf, // Burkina Faso
    Bg, // Bulgaria
    Bh, // Bahrain
    Bi, // Burundi
    Bj, // Benin
    Bl, // Saint Barthélemy
    Bm, // Bermuda
    Bn, // Brunei Darussalam
    Bo, // Bolivia (Plurinational State of)
    Bq, // Bonaire, Sint Eustatius and Saba
    Br, // Brazil
    Bs, // Bahamas
    Bt, // Bhutan
    Bv, // Bouvet Island
    Bw, // Botswana
    By, // Belarus
    Bz, // Belize
    Ca, // Canada
    Cc, // Cocos (Keeling) Islands
    Cd, // Congo, Democratic Republic of the
    Cf, // Central African Republic
    Cg, // Congo
    Ch, // Switzerland
    Ci, // Côte d'Ivoire
    Ck, // Cook Islands
    Cl, // Chile
    Cm, // Cameroon
    Cn, // China
    Co, // Colombia
    Cr, // Costa Rica
    Cu, // Cuba
    Cv, // Cabo Verde
    Cw, // Curaçao
    Cx, // Christmas Island
    Cy, // Cyprus
    Cz, // Czechia
    De, // Germany
    Dj, // Djibouti
    Dk, // Denmark
    Dm, // Dominica
    Do, // Dominican Republic
    Dz, // Algeria
    Ec, // Ecuador
    Ee, // Estonia
    Eg, // Egypt
    Eh, // Western Sahara
    Er, // Eritrea
    Es, // Spain
    Et, // Ethiopia
    Fi, // Finland
    Fj, // Fiji
    Fk, // Falkland Islands (Malvinas)
    Fm, // Micronesia (Federated States of)
    Fo, // Faroe Islands
    Fr, // France
    Ga, // Gabon
    Gb, // United Kingdom of Great Britain and Northern Ireland
    Gd, // Grenada
    Ge, // Georgia
    Gf, // French Guiana
    Gg, // Guernsey
    Gh, // Ghana
    Gi, // Gibraltar
    Gl, // Greenland
    Gm, // Gambia
    Gn, // Guinea
    Gp, // Guadeloupe
    Gq, // Equatorial Guinea
    Gr, // Greece
    Gs, // South Georgia and the South Sandwich Islands
    Gt, // Guatemala
    Gu, // Guam
    Gw, // Guinea-Bissau
    Gy, // Guyana
    Hk, // Hong Kong
    Hm, // Heard Island and McDonald Islands
    Hn, // Honduras
    Hr, // Croatia
    Ht, // Haiti
    Hu, // Hungary
    Id, // Indonesia
    Ie, // Ireland
    Il, // Israel
    Im, // Isle of Man
    In, // India
    Io, // British Indian Ocean Territory
    Iq, // Iraq
    Ir, // Iran (Islamic Republic of)
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
    Kn, // Saint Kitts and Nevis
    Kp, // Korea (Democratic People's Republic of)
    Kr, // Korea, Republic of
    Kw, // Kuwait
    Ky, // Cayman Islands
    Kz, // Kazakhstan
    La, // Lao People's Democratic Republic
    Lb, // Lebanon
    Lc, // Saint Lucia
    Li, // Liechtenstein
    Lk, // Sri Lanka
    Lr, // Liberia
    Ls, // Lesotho
    Lt, // Lithuania
    Lu, // Luxembourg
    Lv, // Latvia
    Ly, // Libya
    Ma, // Morocco
    Mc, // Monaco
    Md, // Moldova, Republic of
    Me, // Montenegro
    Mf, // Saint Martin (French part)
    Mg, // Madagascar
    Mh, // Marshall Islands
    Mk, // North Macedonia
    Ml, // Mali
    Mm, // Myanmar
    Mn, // Mongolia
    Mo, // Macao
    Mp, // Northern Mariana Islands
    Mq, // Martinique
    Mr, // Mauritania
    Ms, // Montserrat
    Mt, // Malta
    Mu, // Mauritius
    Mv, // Maldives
    Mw, // Malawi
    Mx, // Mexico
    My, // Malaysia
    Mz, // Mozambique
    Na, // Namibia
    Nc, // New Caledonia
    Ne, // Niger
    Nf, // Norfolk Island
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
    Pm, // Saint Pierre and Miquelon
    Pn, // Pitcairn
    Pr, // Puerto Rico
    Ps, // Palestine, State of
    Pt, // Portugal
    Pw, // Palau
    Py, // Paraguay
    Qa, // Qatar
    Re, // Réunion
    Ro, // Romania
    Rs, // Serbia
    Ru, // Russian Federation
    Rw, // Rwanda
    Sa, // Saudi Arabia
    Sb, // Solomon Islands
    Sc, // Seychelles
    Sd, // Sudan
    Se, // Sweden
    Sg, // Singapore
    Sh, // Saint Helena, Ascension and Tristan da Cunha
    Si, // Slovenia
    Sj, // Svalbard and Jan Mayen
    Sk, // Slovakia
    Sl, // Sierra Leone
    Sm, // San Marino
    Sn, // Senegal
    So, // Somalia
    Sr, // Suriname
    Ss, // South Sudan
    St, // Sao Tome and Principe
    Sv, // El Salvador
    Sx, // Sint Maarten (Dutch part)
    Sy, // Syrian Arab Republic
    Sz, // Eswatini
    Tc, // Turks and Caicos Islands
    Td, // Chad
    Tf, // French Southern Territories
    Tg, // Togo
    Th, // Thailand
    Tj, // Tajikistan
    Tk, // Tokelau
    Tl, // Timor-Leste
    Tm, // Turkmenistan
    Tn, // Tunisia
    To, // Tonga
    Tr, // Turkey
    Tt, // Trinidad and Tobago
    Tv, // Tuvalu
    Tw, // Taiwan, Province of China
    Tz, // Tanzania, United Republic of
    Ua, // Ukraine
    Ug, // Uganda
    Um, // United States Minor Outlying Islands
    Us, // United States of America
    Uy, // Uruguay
    Uz, // Uzbekistan
    Va, // Holy See
    Vc, // Saint Vincent and the Grenadines
    Ve, // Venezuela (Bolivarian Republic of)
    Vg, // Virgin Islands (British)
    Vi, // Virgin Islands (U.S.)
    Vn, // Viet Nam
    Vu, // Vanuatu
    Wf, // Wallis and Futuna
    Ws, // Samoa
    Ye, // Yemen
    Yt, // Mayotte
    Za, // South Africa
    Zm, // Zambia
    Zw, // Zimbabwe
}
