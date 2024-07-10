# Rust API client for Cartesia

## Overview

This API client is generated using the openapi spec.

## Installation

Put the package under your project folder in a directory named `cartesia` and add the following to `Cargo.toml` under `[dependencies]`:

```
cartesia = { path = "./cartesia" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.cartesia.ai*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**clone_voice_from_clip**](docs/DefaultApi.md#clone_voice_from_clip) | **POST** /voices/clone/clip | Clone Voice (Clip)
*DefaultApi* | [**root_get**](docs/DefaultApi.md#root_get) | **GET** / | API Status and Version
*DefaultApi* | [**tts_bytes_post**](docs/DefaultApi.md#tts_bytes_post) | **POST** /tts/bytes | Stream Speech (Bytes)
*DefaultApi* | [**tts_sse_post**](docs/DefaultApi.md#tts_sse_post) | **POST** /tts/sse | Stream Speech (Server-Sent Events)
*DefaultApi* | [**voices_clone_url_post**](docs/DefaultApi.md#voices_clone_url_post) | **POST** /voices/clone/url | Clone Voice (URL)
*DefaultApi* | [**voices_get**](docs/DefaultApi.md#voices_get) | **GET** /voices | List Voices
*DefaultApi* | [**voices_id_delete**](docs/DefaultApi.md#voices_id_delete) | **DELETE** /voices/{id} | Delete Voice
*DefaultApi* | [**voices_id_get**](docs/DefaultApi.md#voices_id_get) | **GET** /voices/{id} | Get Voice
*DefaultApi* | [**voices_post**](docs/DefaultApi.md#voices_post) | **POST** /voices | Create Voice


## Documentation For Models

 - [Get200Response](docs/Get200Response.md)
 - [JsonBaseResponse](docs/JsonBaseResponse.md)
 - [JsonChunkResponse](docs/JsonChunkResponse.md)
 - [JsonDoneResponse](docs/JsonDoneResponse.md)
 - [JsonErrorResponse](docs/JsonErrorResponse.md)
 - [OutputFormat](docs/OutputFormat.md)
 - [TtsRequest](docs/TtsRequest.md)
 - [TtsSsePost200Response](docs/TtsSsePost200Response.md)
 - [Voice](docs/Voice.md)
 - [Voice1](docs/Voice1.md)
 - [Voice1OneOf](docs/Voice1OneOf.md)
 - [Voice1OneOf1](docs/Voice1OneOf1.md)
 - [VoiceEmbedding](docs/VoiceEmbedding.md)
 - [VoicesPostRequest](docs/VoicesPostRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

