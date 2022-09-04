# \EmojisApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**emoji_create_create_emoji**](EmojisApi.md#emoji_create_create_emoji) | **PUT** /custom/emoji/{id} | Create New Emoji
[**emoji_delete_delete_emoji**](EmojisApi.md#emoji_delete_delete_emoji) | **DELETE** /custom/emoji/{id} | Delete Emoji
[**emoji_fetch_fetch_emoji**](EmojisApi.md#emoji_fetch_fetch_emoji) | **GET** /custom/emoji/{id} | Fetch Emoji



## emoji_create_create_emoji

> crate::models::Emoji emoji_create_create_emoji(id, data_create_emoji)
Create New Emoji

Create an emoji by its Autumn upload id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**data_create_emoji** | [**DataCreateEmoji**](DataCreateEmoji.md) |  | [required] |

### Return type

[**crate::models::Emoji**](Emoji.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emoji_delete_delete_emoji

> emoji_delete_delete_emoji(id)
Delete Emoji

Delete an emoji by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emoji_fetch_fetch_emoji

> crate::models::Emoji emoji_fetch_fetch_emoji(id)
Fetch Emoji

Fetch an emoji by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Emoji**](Emoji.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

