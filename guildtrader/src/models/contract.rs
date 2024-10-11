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
pub struct Contract {
    #[serde(rename = "guild")]
    pub guild: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "tier")]
    pub tier: i32,
    #[serde(rename = "item")]
    pub item: String,
    #[serde(rename = "quantity")]
    pub quantity: i64,
    #[serde(rename = "starts_at")]
    pub starts_at: String,
    #[serde(rename = "completes_at")]
    pub completes_at: String,
}

impl Contract {
    pub fn new(guild: String, status: String, tier: i32, item: String, quantity: i64, starts_at: String, completes_at: String) -> Contract {
        Contract {
            guild,
            status,
            tier,
            item,
            quantity,
            starts_at,
            completes_at,
        }
    }
}

