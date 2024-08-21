/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccountGrantedPoliciesPermissionDetailsResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(
        rename = "grantedProjectPolicies",
        skip_serializing_if = "Option::is_none"
    )]
    pub granted_project_policies:
        Option<Vec<models::GrantedProjectAccessPolicyPermissionDetailsResponseModel>>,
}

impl ServiceAccountGrantedPoliciesPermissionDetailsResponseModel {
    pub fn new() -> ServiceAccountGrantedPoliciesPermissionDetailsResponseModel {
        ServiceAccountGrantedPoliciesPermissionDetailsResponseModel {
            object: None,
            granted_project_policies: None,
        }
    }
}
