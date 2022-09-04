# \SyncApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_settings_req**](SyncApi.md#get_settings_req) | **POST** /sync/settings/fetch | Fetch Settings
[**get_unreads_req**](SyncApi.md#get_unreads_req) | **GET** /sync/unreads | Fetch Unreads
[**set_settings_req**](SyncApi.md#set_settings_req) | **POST** /sync/settings/set | Set Settings



## get_settings_req

> ::std::collections::HashMap<String, crate::models::Array> get_settings_req(options_fetch_settings)
Fetch Settings

Fetch settings from server filtered by keys.  This will return an object with the requested keys, each value is a tuple of `(timestamp, value)`, the value is the previously uploaded data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**options_fetch_settings** | [**OptionsFetchSettings**](OptionsFetchSettings.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::Array>**](array.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unreads_req

> Vec<crate::models::ChannelUnread> get_unreads_req()
Fetch Unreads

Fetch information about unread state on channels.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ChannelUnread>**](ChannelUnread.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_settings_req

> set_settings_req(request_body, timestamp)
Set Settings

Upload data to save to settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**::std::collections::HashMap<String, String>**](String.md) |  | [required] |
**timestamp** | Option<**i64**> | Timestamp of settings change.  Used to avoid feedback loops. |  |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

