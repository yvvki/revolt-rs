# \ServerCustomisationApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**emoji_list_list_emoji**](ServerCustomisationApi.md#emoji_list_list_emoji) | **GET** /servers/{target}/emojis | Fetch Server Emoji



## emoji_list_list_emoji

> Vec<crate::models::Emoji> emoji_list_list_emoji(target)
Fetch Server Emoji

Fetch all emoji on a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Emoji>**](Emoji.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

