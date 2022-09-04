# \GroupsApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**group_add_member_req**](GroupsApi.md#group_add_member_req) | **PUT** /channels/{target}/recipients/{member} | Add Member to Group
[**group_create_req**](GroupsApi.md#group_create_req) | **POST** /channels/create | Create Group
[**group_remove_member_req**](GroupsApi.md#group_remove_member_req) | **DELETE** /channels/{target}/recipients/{member} | Remove Member from Group
[**members_fetch_req**](GroupsApi.md#members_fetch_req) | **GET** /channels/{target}/members | Fetch Group Members



## group_add_member_req

> group_add_member_req(target, member)
Add Member to Group

Adds another user to the group.

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


## group_create_req

> crate::models::Channel group_create_req(data_create_group)
Create Group

Create a new group channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_create_group** | [**DataCreateGroup**](DataCreateGroup.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_remove_member_req

> group_remove_member_req(target, member)
Remove Member from Group

Removes a user from the group.

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


## members_fetch_req

> Vec<crate::models::User> members_fetch_req(target)
Fetch Group Members

Retrieves all users who are part of this group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

