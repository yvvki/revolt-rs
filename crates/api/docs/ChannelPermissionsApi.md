# \ChannelPermissionsApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**permissions_set_default_req**](ChannelPermissionsApi.md#permissions_set_default_req) | **PUT** /channels/{target}/permissions/default | Set Default Permission
[**permissions_set_req**](ChannelPermissionsApi.md#permissions_set_req) | **PUT** /channels/{target}/permissions/{role_id} | Set Role Permission



## permissions_set_default_req

> crate::models::Channel permissions_set_default_req(target, data_default_channel_permissions)
Set Default Permission

Sets permissions for the default role in this channel.  Channel must be a `Group`, `TextChannel` or `VoiceChannel`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**data_default_channel_permissions** | [**DataDefaultChannelPermissions**](DataDefaultChannelPermissions.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permissions_set_req

> crate::models::Channel permissions_set_req(target, role_id, data)
Set Role Permission

Sets permissions for the specified role in this channel.  Channel must be a `TextChannel` or `VoiceChannel`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**data** | [**Data**](Data.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

