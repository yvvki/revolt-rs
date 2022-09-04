# \MessagingApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**channel_ack_req**](MessagingApi.md#channel_ack_req) | **PUT** /channels/{target}/ack/{message} | Acknowledge Message
[**message_bulk_delete_req**](MessagingApi.md#message_bulk_delete_req) | **DELETE** /channels/{target}/messages/bulk | Bulk Delete Messages
[**message_delete_req**](MessagingApi.md#message_delete_req) | **DELETE** /channels/{target}/messages/{msg} | Delete Message
[**message_edit_req**](MessagingApi.md#message_edit_req) | **PATCH** /channels/{target}/messages/{msg} | Edit Message
[**message_fetch_req**](MessagingApi.md#message_fetch_req) | **GET** /channels/{target}/messages/{msg} | Fetch Message
[**message_query_req**](MessagingApi.md#message_query_req) | **GET** /channels/{target}/messages | Fetch Messages
[**message_query_stale_req**](MessagingApi.md#message_query_stale_req) | **POST** /channels/{_target}/messages/stale | Poll Message Changes
[**message_search_req**](MessagingApi.md#message_search_req) | **POST** /channels/{target}/search | Search for Messages
[**message_send_message_send**](MessagingApi.md#message_send_message_send) | **POST** /channels/{target}/messages | Send Message



## channel_ack_req

> channel_ack_req(target, message)
Acknowledge Message

Lets the server and all other clients know that we've seen this message id in this channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**message** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_bulk_delete_req

> message_bulk_delete_req(target, options_bulk_delete)
Bulk Delete Messages

Delete multiple messages you've sent or one you have permission to delete.  This will always require `ManageMessages` permission regardless of whether you own the message or not.  Messages must have been sent within the past 1 week.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**options_bulk_delete** | [**OptionsBulkDelete**](OptionsBulkDelete.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_delete_req

> message_delete_req(target, msg)
Delete Message

Delete a message you've sent or one you have permission to delete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**msg** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_edit_req

> crate::models::Message message_edit_req(target, msg, data_edit_message)
Edit Message

Edits a message that you've previously sent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**msg** | **String** |  | [required] |
**data_edit_message** | [**DataEditMessage**](DataEditMessage.md) |  | [required] |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_fetch_req

> crate::models::Message message_fetch_req(target, msg)
Fetch Message

Retrieves a message by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**msg** | **String** |  | [required] |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_query_req

> crate::models::BulkMessageResponse message_query_req(target, limit, before, after, sort, nearby, include_users)
Fetch Messages

Fetch multiple messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**limit** | Option<**i64**> | Maximum number of messages to fetch  For fetching nearby messages, this is \\`(limit + 1)\\`. |  |
**before** | Option<**String**> | Message id before which messages should be fetched |  |
**after** | Option<**String**> | Message id after which messages should be fetched |  |
**sort** | Option<[**MessageSort**](.md)> | Message sort direction |  |
**nearby** | Option<**String**> | Message id to search around  Specifying 'nearby' ignores 'before', 'after' and 'sort'. It will also take half of limit rounded as the limits to each side. It also fetches the message ID specified. |  |
**include_users** | Option<**bool**> | Whether to include user (and member, if server channel) objects |  |

### Return type

[**crate::models::BulkMessageResponse**](BulkMessageResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_query_stale_req

> message_query_stale_req(_target, options_query_stale)
Poll Message Changes

This route returns any changed message objects and tells you if any have been deleted.  Don't actually poll this route, instead use this to update your local database.  **DEPRECATED**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_target** | **String** |  | [required] |
**options_query_stale** | [**OptionsQueryStale**](OptionsQueryStale.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_search_req

> crate::models::BulkMessageResponse message_search_req(target, options_message_search)
Search for Messages

This route searches for messages within the given parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**options_message_search** | [**OptionsMessageSearch**](OptionsMessageSearch.md) |  | [required] |

### Return type

[**crate::models::BulkMessageResponse**](BulkMessageResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_send_message_send

> crate::models::Message message_send_message_send(target, data_message_send, idempotency_key)
Send Message

Sends a message to the given channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**data_message_send** | [**DataMessageSend**](DataMessageSend.md) |  | [required] |
**idempotency_key** | Option<**String**> | Unique key to prevent duplicate requests |  |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

