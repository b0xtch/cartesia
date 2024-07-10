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
pub struct OutputFormat {
    #[serde(rename = "container")]
    pub container: Container,
    #[serde(rename = "encoding")]
    pub encoding: Encoding,
    #[serde(rename = "sample_rate")]
    pub sample_rate: SampleRate,
}

impl OutputFormat {
    pub fn new(container: Container, encoding: Encoding, sample_rate: SampleRate) -> OutputFormat {
        OutputFormat {
            container,
            encoding,
            sample_rate,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Container {
    #[serde(rename = "raw")]
    Raw,
}

impl Default for Container {
    fn default() -> Container {
        Self::Raw
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Encoding {
    #[serde(rename = "pcm_s16le")]
    S16le,
    #[serde(rename = "pcm_f32le")]
    F32le,
    #[serde(rename = "pcm_mulaw")]
    Mulaw,
    #[serde(rename = "pcm_alaw")]
    Alaw,
}

impl Default for Encoding {
    fn default() -> Encoding {
        Self::S16le
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SampleRate {
    #[serde(rename = "8000")]
    Variant8000,
    #[serde(rename = "16000")]
    Variant16000,
    #[serde(rename = "22050")]
    Variant22050,
    #[serde(rename = "24000")]
    Variant24000,
    #[serde(rename = "44100")]
    Variant44100,
}

impl Default for SampleRate {
    fn default() -> SampleRate {
        Self::Variant8000
    }
}

