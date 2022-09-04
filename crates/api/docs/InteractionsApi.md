# \InteractionsApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**message_clear_reactions_clear_reactions**](InteractionsApi.md#message_clear_reactions_clear_reactions) | **DELETE** /channels/{target}/messages/{msg}/reactions | Remove All Reactions from Message
[**message_react_react_message**](InteractionsApi.md#message_react_react_message) | **PUT** /channels/{target}/messages/{msg}/reactions/{emoji} | Add Reaction to Message
[**message_unreact_unreact_message**](InteractionsApi.md#message_unreact_unreact_message) | **DELETE** /channels/{target}/messages/{msg}/reactions/{emoji} | Remove Reaction(s) to Message



## message_clear_reactions_clear_reactions

> message_clear_reactions_clear_reactions(target, msg)
Remove All Reactions from Message

Remove your own, someone else's or all of a given reaction.  Requires `ManageMessages` permission.

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


## message_react_react_message

> message_react_react_message(target, msg, emoji)
Add Reaction to Message

React to a given message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**msg** | **String** |  | [required] |
**emoji** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_unreact_unreact_message

> message_unreact_unreact_message(target, msg, emoji, user_id, remove_all)
Remove Reaction(s) to Message

Remove your own, someone else's or all of a given reaction.  Requires `ManageMessages` if changing others' reactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**msg** | **String** |  | [required] |
**emoji** | **String** |  | [required] |
**user_id** | Option<**String**> | Remove a specific user's reaction |  |
**remove_all** | Option<**bool**> | Remove all reactions |  |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

