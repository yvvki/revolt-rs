# \WebhooksApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**webhook_create_req**](WebhooksApi.md#webhook_create_req) | **POST** /channels/{target}/webhooks | Creates a webhook
[**webhook_fetch_all_req**](WebhooksApi.md#webhook_fetch_all_req) | **GET** /channels/{channel_id}/webhooks | Gets all webhooks



## webhook_create_req

> crate::models::Webhook webhook_create_req(target, create_webhook_body)
Creates a webhook

Creates a webhook which 3rd party platforms can use to send messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**create_webhook_body** | [**CreateWebhookBody**](CreateWebhookBody.md) |  | [required] |

### Return type

[**crate::models::Webhook**](Webhook.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_fetch_all_req

> Vec<crate::models::Webhook> webhook_fetch_all_req(channel_id)
Gets all webhooks

Gets all webhooks inside the channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Webhook>**](Webhook.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

