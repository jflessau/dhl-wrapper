use crate::error::Error as LibError;
use chrono::NaiveTime;
use convert_case::{Case, Casing};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// API docs: https://developer.dhl.com/api-reference/location-finder#reference-docs-section

// TODO - split this to return either a list of locations or just one location.
/// Parameters of a get request returning DHL Service Point Locations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GetDhlSpl {
    ByAddress {
        country_code: DhlCountryCodes,
        address_locality: Option<String>,
        postal_code: Option<String>,
        street_address: Option<String>,
        provider_type: Option<String>,
        location_type: Option<String>,
        service_type: Option<DhlServiceType>,
        radius: Option<i64>,
        limit: Option<i64>,
        hide_closed_locations: Option<bool>,
    },
    ByGeo {
        latitude: f64,
        longitude: f64,
        provider_type: Option<String>,
        location_type: Option<String>,
        service_type: Option<DhlServiceType>,
        radius: Option<i64>,
        limit: Option<i64>,
        hide_closed_locations: Option<bool>,
    },
    ByKeywordId {
        keyword_id: String,
        country_code: DhlCountryCodes,
        postal_code: String,
    },
    ById(String),
}

impl GetDhlSpl {
    pub fn by_address(country_code: DhlCountryCodes) -> Self {
        GetDhlSpl::ByAddress {
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
        GetDhlSpl::ByGeo {
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

    pub fn by_keyword_id(
        keyword_id: String,
        country_code: DhlCountryCodes,
        postal_code: String,
    ) -> Self {
        GetDhlSpl::ByKeywordId {
            keyword_id,
            country_code,
            postal_code,
        }
    }

    pub fn address_locality(&mut self, value: Option<String>) {
        match self {
            GetDhlSpl::ByAddress {
                address_locality, ..
            } => {
                *address_locality = value;
            }
            _ => {}
        }
    }

    pub fn postal_code(&mut self, value: Option<String>) {
        match self {
            GetDhlSpl::ByAddress { postal_code, .. } => {
                *postal_code = value;
            }
            _ => {}
        }
    }

    pub fn street_address(&mut self, value: Option<String>) {
        match self {
            GetDhlSpl::ByAddress { street_address, .. } => {
                *street_address = value;
            }
            _ => {}
        }
    }

    pub fn provider_type(&mut self, value: Option<String>) {
        match self {
            GetDhlSpl::ByAddress { provider_type, .. } => {
                *provider_type = value;
            }
            GetDhlSpl::ByGeo { provider_type, .. } => {
                *provider_type = value;
            }
            _ => {}
        }
    }

    pub fn location_type(&mut self, value: Option<String>) {
        match self {
            GetDhlSpl::ByAddress { location_type, .. } => {
                *location_type = value;
            }
            GetDhlSpl::ByGeo { location_type, .. } => {
                *location_type = value;
            }
            _ => {}
        }
    }

    pub fn service_type(&mut self, value: Option<DhlServiceType>) {
        match self {
            GetDhlSpl::ByAddress { service_type, .. } => {
                *service_type = value;
            }
            GetDhlSpl::ByGeo { service_type, .. } => {
                *service_type = value;
            }
            _ => {}
        }
    }

    pub fn radius(&mut self, value: Option<i64>) {
        match self {
            GetDhlSpl::ByAddress { radius, .. } => {
                *radius = value;
            }
            GetDhlSpl::ByGeo { radius, .. } => {
                *radius = value;
            }
            _ => {}
        }
    }

    pub fn limit(&mut self, value: Option<i64>) {
        match self {
            GetDhlSpl::ByAddress { limit, .. } => {
                *limit = value;
            }
            GetDhlSpl::ByGeo { limit, .. } => {
                *limit = value;
            }
            _ => {}
        }
    }

    pub fn hide_closed_locations(&mut self, value: Option<bool>) {
        match self {
            GetDhlSpl::ByAddress {
                hide_closed_locations,
                ..
            } => {
                *hide_closed_locations = value;
            }
            GetDhlSpl::ByGeo {
                hide_closed_locations,
                ..
            } => {
                *hide_closed_locations = value;
            }
            _ => {}
        }
    }

    pub async fn send(&self) -> Result<GetDhlSplsResponse, LibError> {
        let url = match self {
            GetDhlSpl::ByAddress { .. } => {
                format!(
                    "https://api-sandbox.dhl.com/location-finder/v1/find-by-address{}",
                    serializable_to_params(self)
                )
            }
            GetDhlSpl::ByGeo { .. } => {
                format!(
                    "https://api-sandbox.dhl.com/location-finder/v1/find-by-geo{}",
                    serializable_to_params(self)
                )
            }
            GetDhlSpl::ByKeywordId { .. } => {
                format!(
                    "https://api-sandbox.dhl.com/location-finder/v1/find-by-keyword-id{}",
                    serializable_to_params(self)
                )
            }
            GetDhlSpl::ById(id) => {
                format!(
                    "https://api-sandbox.dhl.com/location-finder/v1/locations/{}",
                    id
                )
            }
        };

        println!("url: {}", url);

        dotenv().ok(); // TODO
        let client = reqwest::Client::new();
        let api_key = dotenv::var("DHL_LOCATION_FINDER_API_KEY").unwrap_or_else(|_| "".to_string());
        let res_bytes = client
            .get(url)
            .header("DHL-API-Key", api_key)
            .send()
            .await?
            .bytes()
            .await?;

        let res_not_ok = serde_json::from_slice::<GetDhlSplResponseNotOk>(&res_bytes);
        if let Ok(v) = res_not_ok {
            return Err(LibError::GetDhlSplResponseNotOk {
                status: v.status,
                title: v.title,
                detail: v.detail,
            });
        }

        let res = match self {
            GetDhlSpl::ByAddress { .. } | GetDhlSpl::ByGeo { .. } => {
                serde_json::from_slice::<GetDhlSplsResponse>(&res_bytes)?
            }
            GetDhlSpl::ByKeywordId { .. } | GetDhlSpl::ById(_) => {
                println!("bytes: {:#?}", res_bytes);
                let get_dhl_spl_response = serde_json::from_slice::<GetDhlSplResponse>(&res_bytes)?;

                GetDhlSplsResponse {
                    locations: vec![get_dhl_spl_response.location],
                }
            }
        };

        Ok(res)
    }
}

pub fn serializable_to_params<T: Serialize>(serializable: &T) -> String {
    let value = serde_json::to_value(serializable).unwrap(); // TODO

    let mut params = Vec::new();

    if let Value::Object(v) = value {
        let v = v.into_iter().collect::<Vec<(String, Value)>>();
        if let Some(v) = v.first() {
            if let Value::Object(v) = &v.1 {
                let mut n = 0;
                for value in v.iter() {
                    let prefix = if n > 0 { "&" } else { "" };

                    match value.1 {
                        Value::Bool(v) => {
                            params.push(format!(
                                "{}{}={}",
                                prefix,
                                value.0.to_case(Case::Camel),
                                v
                            ));
                            n += 1;
                        }
                        Value::Number(v) => {
                            params.push(format!(
                                "{}{}={}",
                                prefix,
                                value.0.to_case(Case::Camel),
                                v
                            ));
                            n += 1;
                        }
                        Value::String(v) => {
                            params.push(format!(
                                "{}{}={}",
                                prefix,
                                value.0.to_case(Case::Camel),
                                v
                            ));
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

                return query;
            }
        }
    }

    String::new()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDhlSplsResponse {
    pub locations: Vec<DhlServicePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDhlSplResponse {
    pub location: DhlServicePoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DhlServicePoint {
    pub url: String,
    pub location: DhlServicePointLocation,
    pub name: String,
    pub distance: i64,
    pub place: DhlPlace,
    pub opening_hours: Vec<DhlOpeningHours>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DhlServicePointLocation {
    pub ids: Vec<DhlServicePointLocationId>,
    pub keyword: String,
    pub keyword_id: String,
    pub r#type: String,
    pub lean_locker: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DhlServicePointLocationId {
    pub location_id: String,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DhlPlace {
    pub address: DhlAddress,
    pub geo: DhlGeo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DhlAddress {
    pub country_code: String,
    pub postal_code: String,
    pub address_locality: String,
    pub street_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DhlGeo {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DhlOpeningHours {
    pub opens: NaiveTime,
    pub closes: NaiveTime,
    pub day_of_week: DhlWeekday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DhlWeekday {
    #[serde(alias = "http://schema.org/Monday")]
    Mon,
    #[serde(alias = "http://schema.org/Tuesday")]
    Tue,
    #[serde(alias = "http://schema.org/Wednesday")]
    Wed,
    #[serde(alias = "http://schema.org/Thursday")]
    Thu,
    #[serde(alias = "http://schema.org/Friday")]
    Fri,
    #[serde(alias = "http://schema.org/Saturday")]
    Sat,
    #[serde(alias = "http://schema.org/Sunday")]
    Sun,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDhlSplResponseNotOk {
    pub status: String,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DhlDevisions {
    PostAndParcel,
    EcomSolutions,
    Express,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DhlServiceType {
    #[serde(alias = "parcel:pick-up")]
    ParcelPickUp,
    #[serde(alias = "parcel:drop-off")]
    ParcelDropOff,
    #[serde(alias = "express:pick-up")]
    ExpressPickUp,
    #[serde(alias = "express:drop-off")]
    ExpressDropOff,
    #[serde(alias = "express:drop-off-account")]
    ExpressDropOffAccount,
    #[serde(alias = "express:drop-off-easy")]
    ExpressDropOffEasy,
    #[serde(alias = "express:drop-off-prelabeled")]
    ExpressDropOffPrelabeled,
    #[serde(alias = "parcel:pick-up-registered")]
    ParcelPickUpRegistered,
    #[serde(alias = "parcel:pick-up-unregistered")]
    ParcelPickUpUnregistered,
    #[serde(alias = "parcel:drop-off-unregistered")]
    ParcelDropOffUnregistered,
    #[serde(alias = "letter-service")]
    LetterService,
    #[serde(alias = "postbank")]
    Postbank,
    #[serde(alias = "cash-on-delivery")]
    CashOnDelivery,
    #[serde(alias = "franking")]
    Franking,
    #[serde(alias = "cash-service")]
    CashService,
    #[serde(alias = "packaging-material")]
    PackagingMaterial,
    #[serde(alias = "postident")]
    Postident,
    #[serde(alias = "age-verification")]
    AgeVerification,
    #[serde(alias = "handicapped-access")]
    HandicappedAccess,
    #[serde(alias = "parking")]
    Parking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DhlCountryCodes {
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
