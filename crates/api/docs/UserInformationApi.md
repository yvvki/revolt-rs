# \UserInformationApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_username_req**](UserInformationApi.md#change_username_req) | **PATCH** /users/@me/username | Change Username
[**edit_user_req**](UserInformationApi.md#edit_user_req) | **PATCH** /users/{target} | Edit User
[**fetch_profile_req**](UserInformationApi.md#fetch_profile_req) | **GET** /users/{target}/profile | Fetch User Profile
[**fetch_self_req**](UserInformationApi.md#fetch_self_req) | **GET** /users/@me | Fetch Self
[**fetch_user_flags_fetch_user_flags**](UserInformationApi.md#fetch_user_flags_fetch_user_flags) | **GET** /users/{target}/flags | Fetch User Flags
[**fetch_user_req**](UserInformationApi.md#fetch_user_req) | **GET** /users/{target} | Fetch User
[**get_default_avatar_req**](UserInformationApi.md#get_default_avatar_req) | **GET** /users/{target}/default_avatar | Fetch Default Avatar



## change_username_req

> crate::models::User change_username_req(data_change_username)
Change Username

Change your username.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_change_username** | [**DataChangeUsername**](DataChangeUsername.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_user_req

> crate::models::User edit_user_req(target, data_edit_user)
Edit User

Edit currently authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**data_edit_user** | [**DataEditUser**](DataEditUser.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_profile_req

> crate::models::UserProfile fetch_profile_req(target)
Fetch User Profile

Retrieve a user's profile data.  Will fail if you do not have permission to access the other user's profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::UserProfile**](UserProfile.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_self_req

> crate::models::User fetch_self_req()
Fetch Self

Retrieve your user information.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::User**](User.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_flags_fetch_user_flags

> crate::models::FlagResponse fetch_user_flags_fetch_user_flags(target)
Fetch User Flags

Retrieve a user's flags.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::FlagResponse**](FlagResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_req

> crate::models::User fetch_user_req(target)
Fetch User

Retrieve a user's information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_avatar_req

> std::path::PathBuf get_default_avatar_req(target)
Fetch Default Avatar

This returns a default avatar based on the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

