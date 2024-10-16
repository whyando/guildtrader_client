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
pub struct Cancel200ResponseRefund {
    #[serde(rename = "item")]
    pub item: String,
    #[serde(rename = "quantity")]
    pub quantity: i64,
}

impl Cancel200ResponseRefund {
    pub fn new(item: String, quantity: i64) -> Cancel200ResponseRefund {
        Cancel200ResponseRefund {
            item,
            quantity,
        }
    }
}

