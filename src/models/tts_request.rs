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
pub struct TtsRequest {
    #[serde(rename = "model_id")]
    pub model_id: String,
    #[serde(rename = "transcript")]
    pub transcript: String,
    /// The maximum duration of the audio in seconds.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "voice")]
    pub voice: Box<models::Voice1>,
    #[serde(rename = "output_format")]
    pub output_format: Box<models::OutputFormat>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
}

impl TtsRequest {
    pub fn new(
        model_id: String,
        transcript: String,
        voice: models::Voice1,
        output_format: models::OutputFormat,
    ) -> TtsRequest {
        TtsRequest {
            model_id,
            transcript,
            duration: None,
            voice: Box::new(voice),
            output_format: Box::new(output_format),
            language: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "zh")]
    Zh,
    #[serde(rename = "ja")]
    Ja,
}

impl Default for Language {
    fn default() -> Language {
        Self::En
    }
}
