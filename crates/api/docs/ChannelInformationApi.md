# \ChannelInformationApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**channel_delete_req**](ChannelInformationApi.md#channel_delete_req) | **DELETE** /channels/{target} | Close Channel
[**channel_edit_req**](ChannelInformationApi.md#channel_edit_req) | **PATCH** /channels/{target} | Edit Channel
[**channel_fetch_req**](ChannelInformationApi.md#channel_fetch_req) | **GET** /channels/{target} | Fetch Channel



## channel_delete_req

> channel_delete_req(target, leave_silently)
Close Channel

Deletes a server channel, leaves a group or closes a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**leave_silently** | Option<**bool**> | Whether to not send a leave message |  |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channel_edit_req

> crate::models::Channel channel_edit_req(target, data_edit_channel)
Edit Channel

Edit a channel object by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**data_edit_channel** | [**DataEditChannel**](DataEditChannel.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channel_fetch_req

> crate::models::Channel channel_fetch_req(target)
Fetch Channel

Fetch channel by its id.

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

