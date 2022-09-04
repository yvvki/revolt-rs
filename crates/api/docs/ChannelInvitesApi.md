# \ChannelInvitesApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**invite_create_req**](ChannelInvitesApi.md#invite_create_req) | **POST** /channels/{target}/invites | Create Invite



## invite_create_req

> crate::models::Invite invite_create_req(target)
Create Invite

Creates an invite to this channel.  Channel must be a `TextChannel`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::Invite**](Invite.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

