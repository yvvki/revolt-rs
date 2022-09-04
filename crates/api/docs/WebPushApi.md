# \WebPushApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscribe_req**](WebPushApi.md#subscribe_req) | **POST** /push/subscribe | Push Subscribe
[**unsubscribe_req**](WebPushApi.md#unsubscribe_req) | **POST** /push/unsubscribe | Unsubscribe



## subscribe_req

> subscribe_req(web_push_subscription)
Push Subscribe

Create a new Web Push subscription.  If an existing subscription exists on this session, it will be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_push_subscription** | [**WebPushSubscription**](WebPushSubscription.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe_req

> unsubscribe_req()
Unsubscribe

Remove the Web Push subscription associated with the current session.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

