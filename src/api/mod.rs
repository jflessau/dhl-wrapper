use crate::error::DhlError;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod location_finder;
pub mod shipment_tracking;

/// Serializes a struct's fields into a string of url parameters.
fn serializable_to_url_params<T: Serialize>(serializable: &T) -> Result<String, DhlError> {
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
            query.push('?');
        }

        for param in params {
            query.push_str(&param);
        }

        return Ok(query);
    }

    Ok(String::new())
}

/// In case DHL responds with a 4xx or 5xx status code, the response will
/// deserialized to this struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseNotOk {
    status: u32,
    title: String,
    detail: String,
}

/// DHL service division.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Division {
    Express,
    ParcelDe,
    Ecommerce,
    Dgf,
    ParcelUk,
    PostDe,
    Sameday,
    Freight,
    ParcelNl,
    ParcelPl,
    Dsc,
}

/// DHL service service types.
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

/// Address information used in other structs like [location_finder::Place](location_finder::Place).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub country_code: Option<String>,
    pub postal_code: Option<String>,
    pub address_locality: Option<String>,
    pub street_address: Option<String>,
}

/// Two-letter country codes (<https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>).
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

/// ISO 639-1 2-character language code (<https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LanguageCode {
    Aa, // Afar
    Ab, // Abkhazian
    Af, // Afrikaans
    Am, // Amharic
    Ar, // Arabic
    As, // Assamese
    Ay, // Aymara
    Az, // Azerbaijani
    Ba, // Bashkir
    Be, // Byelorussian
    Bg, // Bulgarian
    Bh, // Bihari
    Bi, // Bislama
    Bn, // Bengali
    Bo, // Tibetan
    Br, // Breton
    Ca, // Catalan
    Co, // Corsican
    Cs, // Czech
    Cy, // Welch
    Da, // Danish
    De, // German
    Dz, // Bhutani
    El, // Greek
    En, // English
    Eo, // Esperanto
    Es, // Spanish
    Et, // Estonian
    Eu, // Basque
    Fa, // Persian
    Fi, // Finnish
    Fj, // Fiji
    Fo, // Faeroese
    Fr, // French
    Fy, // Frisian
    Ga, // Irish
    Gd, // Scots Gaelic
    Gl, // Galician
    Gn, // Guarani
    Gu, // Gujarati
    Ha, // Hausa
    Hi, // Hindi
    He, // Hebrew
    Hr, // Croatian
    Hu, // Hungarian
    Hy, // Armenian
    Ia, // Interlingua
    Id, // Indonesian
    Ie, // Interlingue
    Ik, // Inupiak
    In, // former Indonesian
    Is, // Icelandic
    It, // Italian
    Iu, // Inuktitut (Eskimo)
    Iw, // former Hebrew
    Ja, // Japanese
    Ji, // former Yiddish
    Jw, // Javanese
    Ka, // Georgian
    Kk, // Kazakh
    Kl, // Greenlandic
    Km, // Cambodian
    Kn, // Kannada
    Ko, // Korean
    Ks, // Kashmiri
    Ku, // Kurdish
    Ky, // Kirghiz
    La, // Latin
    Ln, // Lingala
    Lo, // Laothian
    Lt, // Lithuanian
    Lv, // Latvian, Lettish
    Mg, // Malagasy
    Mi, // Maori
    Mk, // Macedonian
    Ml, // Malayalam
    Mn, // Mongolian
    Mo, // Moldavian
    Mr, // Marathi
    Ms, // Malay
    Mt, // Maltese
    My, // Burmese
    Na, // Nauru
    Ne, // Nepali
    Nl, // Dutch
    No, // Norwegian
    Oc, // Occitan
    Om, // (Afan) Oromo
    Or, // Oriya
    Pa, // Punjabi
    Pl, // Polish
    Ps, // Pashto, Pushto
    Pt, // Portuguese
    Qu, // Quechua
    Rm, // Rhaeto-Romance
    Rn, // Kirundi
    Ro, // Romanian
    Ru, // Russian
    Rw, // Kinyarwanda
    Sa, // Sanskrit
    Sd, // Sindhi
    Sg, // Sangro
    Sh, // Serbo-Croatian
    Si, // Singhalese
    Sk, // Slovak
    Sl, // Slovenian
    Sm, // Samoan
    Sn, // Shona
    So, // Somali
    Sq, // Albanian
    Sr, // Serbian
    Ss, // Siswati
    St, // Sesotho
    Su, // Sudanese
    Sv, // Swedish
    Sw, // Swahili
    Ta, // Tamil
    Te, // Tegulu
    Tg, // Tajik
    Th, // Thai
    Ti, // Tigrinya
    Tk, // Turkmen
    Tl, // Tagalog
    Tn, // Setswana
    To, // Tonga
    Tr, // Turkish
    Ts, // Tsonga
    Tt, // Tatar
    Tw, // Twi
    Ug, // Uigur
    Uk, // Ukrainian
    Ur, // Urdu
    Uz, // Uzbek
    Vi, // Vietnamese
    Vo, // Volapuk
    Wo, // Wolof
    Xh, // Xhosa
    Yi, // Yiddish
    Yo, // Yoruba
    Za, // Zhuang
    Zh, // Chinese
    Zu, // Zulu
}
