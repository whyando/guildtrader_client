/*
 * guildtrader
 *
 * Guildtrader API
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct List200Response {
    #[serde(rename = "data")]
    pub data: Vec<models::Guild>,
    #[serde(rename = "length")]
    pub length: i32,
}

impl List200Response {
    pub fn new(data: Vec<models::Guild>, length: i32) -> List200Response {
        List200Response {
            data,
            length,
        }
    }
}

