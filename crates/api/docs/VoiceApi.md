# \VoiceApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**voice_join_req**](VoiceApi.md#voice_join_req) | **POST** /channels/{target}/join_call | Join Call



## voice_join_req

> crate::models::CreateVoiceUserResponse voice_join_req(target)
Join Call

Asks the voice server for a token to join the call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::CreateVoiceUserResponse**](CreateVoiceUserResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

