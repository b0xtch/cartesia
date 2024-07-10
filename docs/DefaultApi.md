# \DefaultApi

All URIs are relative to *https://api.cartesia.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_voice_from_clip**](DefaultApi.md#clone_voice_from_clip) | **POST** /voices/clone/clip | Clone Voice (Clip)
[**root_get**](DefaultApi.md#root_get) | **GET** / | API Status and Version
[**tts_bytes_post**](DefaultApi.md#tts_bytes_post) | **POST** /tts/bytes | Stream Speech (Bytes)
[**tts_sse_post**](DefaultApi.md#tts_sse_post) | **POST** /tts/sse | Stream Speech (Server-Sent Events)
[**voices_clone_url_post**](DefaultApi.md#voices_clone_url_post) | **POST** /voices/clone/url | Clone Voice (URL)
[**voices_get**](DefaultApi.md#voices_get) | **GET** /voices | List Voices
[**voices_id_delete**](DefaultApi.md#voices_id_delete) | **DELETE** /voices/{id} | Delete Voice
[**voices_id_get**](DefaultApi.md#voices_id_get) | **GET** /voices/{id} | Get Voice
[**voices_post**](DefaultApi.md#voices_post) | **POST** /voices | Create Voice



## clone_voice_from_clip

> models::VoiceEmbedding clone_voice_from_clip(cartesia_version, clip)
Clone Voice (Clip)

Clones a voice from an audio clip uploaded as a file. The clip is uploaded using multipart/form-data with a `clip` field containing the audio file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cartesia_version** | **String** | The version of the Cartesia API to use. | [required] |
**clip** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::VoiceEmbedding**](Voice_Embedding.md)

### Authorization

[APIKeyHeader](../README.md#APIKeyHeader)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## root_get

> models::Get200Response root_get()
API Status and Version

Returns the server's version and a status message which can be useful for sanity checking.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Get200Response**](__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tts_bytes_post

> std::path::PathBuf tts_bytes_post(cartesia_version, tts_request)
Stream Speech (Bytes)

Generate audio from a transcript using a given voice and model. The audio is streamed out as raw bytes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cartesia_version** | **String** | The version of the Cartesia API to use. | [required] |
**tts_request** | [**TtsRequest**](TtsRequest.md) |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[APIKeyHeader](../README.md#APIKeyHeader)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/octet-stream, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tts_sse_post

> models::TtsSsePost200Response tts_sse_post(cartesia_version, tts_request)
Stream Speech (Server-Sent Events)

Generate audio from a transcript using a given voice and model. The audio is streamed out as Server-Sent Events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cartesia_version** | **String** | The version of the Cartesia API to use. | [required] |
**tts_request** | [**TtsRequest**](TtsRequest.md) |  | [required] |

### Return type

[**models::TtsSsePost200Response**](_tts_sse_post_200_response.md)

### Authorization

[APIKeyHeader](../README.md#APIKeyHeader)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/event-stream, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## voices_clone_url_post

> models::VoiceEmbedding voices_clone_url_post(cartesia_version, link)
Clone Voice (URL)

Clone a voice from an online URL. Currently only supports YouTube.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cartesia_version** | **String** | The version of the Cartesia API to use. | [required] |
**link** | **String** |  | [required] |

### Return type

[**models::VoiceEmbedding**](Voice_Embedding.md)

### Authorization

[APIKeyHeader](../README.md#APIKeyHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## voices_get

> Vec<models::Voice> voices_get(cartesia_version)
List Voices

List all voices available to the user, that is, public voices and the user's own voices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cartesia_version** | **String** | The version of the Cartesia API to use. | [required] |

### Return type

[**Vec<models::Voice>**](Voice.md)

### Authorization

[APIKeyHeader](../README.md#APIKeyHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## voices_id_delete

> voices_id_delete(cartesia_version, id)
Delete Voice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cartesia_version** | **String** | The version of the Cartesia API to use. | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[APIKeyHeader](../README.md#APIKeyHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## voices_id_get

> models::Voice voices_id_get(cartesia_version, id)
Get Voice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cartesia_version** | **String** | The version of the Cartesia API to use. | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::Voice**](Voice.md)

### Authorization

[APIKeyHeader](../README.md#APIKeyHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## voices_post

> models::Voice voices_post(cartesia_version, voices_post_request)
Create Voice

Create a new voice with a given name, description, and embedding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cartesia_version** | **String** | The version of the Cartesia API to use. | [required] |
**voices_post_request** | [**VoicesPostRequest**](VoicesPostRequest.md) |  | [required] |

### Return type

[**models::Voice**](Voice.md)

### Authorization

[APIKeyHeader](../README.md#APIKeyHeader)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

