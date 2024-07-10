# JsonChunkResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status_code** | Option<**i32**> | HTTP status code to allow for error handling. This is for errors encountered while streaming the response. | [optional]
**done** | Option<**bool**> | Whether the model has finished generating audio. | [optional]
**data** | Option<**String**> | Binary audio data encoded as base64. The underlying bytes will be in the requested output format. | [optional]
**step_time** | Option<**i32**> | The time in milliseconds that the model took to generate this chunk of audio. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


