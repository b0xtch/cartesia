/*
 * Cartesia REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2024-06-10
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoicesPostRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "embedding")]
    pub embedding: Vec<f64>,
}

impl VoicesPostRequest {
    pub fn new(name: String, description: String, embedding: Vec<f64>) -> VoicesPostRequest {
        VoicesPostRequest {
            name,
            description,
            embedding,
        }
    }
}
