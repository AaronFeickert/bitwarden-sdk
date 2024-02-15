/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttestationConveyancePreference {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "indirect")]
    Indirect,
    #[serde(rename = "direct")]
    Direct,
}

impl ToString for AttestationConveyancePreference {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::Indirect => String::from("indirect"),
            Self::Direct => String::from("direct"),
        }
    }
}

impl Default for AttestationConveyancePreference {
    fn default() -> AttestationConveyancePreference {
        Self::None
    }
}