# \DirectMessagingApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_dms_req**](DirectMessagingApi.md#fetch_dms_req) | **GET** /users/dms | Fetch Direct Message Channels
[**open_dm_req**](DirectMessagingApi.md#open_dm_req) | **GET** /users/{target}/dm | Open Direct Message



## fetch_dms_req

> Vec<crate::models::Channel> fetch_dms_req()
Fetch Direct Message Channels

This fetches your direct messages, including any DM and group DM conversations.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open_dm_req

> crate::models::Channel open_dm_req(target)
Open Direct Message

Open a DM with another user.  If the target is oneself, a saved messages channel is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

