# \ServerMembersApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ban_create_req**](ServerMembersApi.md#ban_create_req) | **PUT** /servers/{server}/bans/{target} | Ban User
[**ban_list_req**](ServerMembersApi.md#ban_list_req) | **GET** /servers/{target}/bans | Fetch Bans
[**ban_remove_req**](ServerMembersApi.md#ban_remove_req) | **DELETE** /servers/{server}/bans/{target} | Unban user
[**invites_fetch_req**](ServerMembersApi.md#invites_fetch_req) | **GET** /servers/{target}/invites | Fetch Invites
[**member_edit_req**](ServerMembersApi.md#member_edit_req) | **PATCH** /servers/{server}/members/{target} | Edit Member
[**member_fetch_all_req**](ServerMembersApi.md#member_fetch_all_req) | **GET** /servers/{target}/members | Fetch Members
[**member_fetch_req**](ServerMembersApi.md#member_fetch_req) | **GET** /servers/{target}/members/{member} | Fetch Member
[**member_remove_req**](ServerMembersApi.md#member_remove_req) | **DELETE** /servers/{target}/members/{member} | Kick Member



## ban_create_req

> crate::models::ServerBan ban_create_req(server, target, data_ban_create)
Ban User

Ban a user by their id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server** | **String** |  | [required] |
**target** | **String** |  | [required] |
**data_ban_create** | [**DataBanCreate**](DataBanCreate.md) |  | [required] |

### Return type

[**crate::models::ServerBan**](ServerBan.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ban_list_req

> crate::models::BanListResult ban_list_req(target)
Fetch Bans

Fetch all bans on a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::BanListResult**](BanListResult.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ban_remove_req

> ban_remove_req(server, target)
Unban user

Remove a user's ban.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server** | **String** |  | [required] |
**target** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invites_fetch_req

> Vec<crate::models::Invite> invites_fetch_req(target)
Fetch Invites

Fetch all server invites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Invite>**](Invite.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## member_edit_req

> crate::models::Member member_edit_req(server, target, data_member_edit)
Edit Member

Edit a member by their id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server** | **String** |  | [required] |
**target** | **String** |  | [required] |
**data_member_edit** | [**DataMemberEdit**](DataMemberEdit.md) |  | [required] |

### Return type

[**crate::models::Member**](Member.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## member_fetch_all_req

> crate::models::AllMemberResponse member_fetch_all_req(target, exclude_offline)
Fetch Members

Fetch all server members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**exclude_offline** | Option<**bool**> | Whether to exclude offline users |  |

### Return type

[**crate::models::AllMemberResponse**](AllMemberResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## member_fetch_req

> crate::models::Member member_fetch_req(target, member)
Fetch Member

Retrieve a member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**member** | **String** |  | [required] |

### Return type

[**crate::models::Member**](Member.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## member_remove_req

> member_remove_req(target, member)
Kick Member

Removes a member from the server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**member** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

