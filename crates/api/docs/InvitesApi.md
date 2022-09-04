# \InvitesApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**invite_delete_req**](InvitesApi.md#invite_delete_req) | **DELETE** /invites/{target} | Delete Invite
[**invite_fetch_req**](InvitesApi.md#invite_fetch_req) | **GET** /invites/{target} | Fetch Invite
[**invite_join_req**](InvitesApi.md#invite_join_req) | **POST** /invites/{target} | Join Invite



## invite_delete_req

> invite_delete_req(target)
Delete Invite

Delete an invite by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_fetch_req

> crate::models::InviteResponse invite_fetch_req(target)
Fetch Invite

Fetch an invite by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::InviteResponse**](InviteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_join_req

> crate::models::InviteJoinResponse invite_join_req(target)
Join Invite

Join an invite by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::InviteJoinResponse**](InviteJoinResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

