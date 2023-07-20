# \AdminApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**message_query_message_query**](AdminApi.md#message_query_message_query) | **POST** /admin/messages | Globally Fetch Messages
[**stats_stats**](AdminApi.md#stats_stats) | **GET** /admin/stats | Query Stats



## message_query_message_query

> crate::models::BulkMessageResponse message_query_message_query(message_query)
Globally Fetch Messages

This is a privileged route to globally fetch messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_query** | [**MessageQuery**](MessageQuery.md) |  | [required] |

### Return type

[**crate::models::BulkMessageResponse**](BulkMessageResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stats_stats

> crate::models::Stats stats_stats()
Query Stats

Fetch various technical statistics.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Stats**](Stats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

