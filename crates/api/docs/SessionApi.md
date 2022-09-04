# \SessionApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edit_edit**](SessionApi.md#edit_edit) | **PATCH** /auth/session/{id} | Edit Session
[**fetch_all_fetch_all**](SessionApi.md#fetch_all_fetch_all) | **GET** /auth/session/all | Fetch Sessions
[**login_login**](SessionApi.md#login_login) | **POST** /auth/session/login | Login
[**logout_logout**](SessionApi.md#logout_logout) | **POST** /auth/session/logout | Logout
[**revoke_all_revoke_all**](SessionApi.md#revoke_all_revoke_all) | **DELETE** /auth/session/all | Delete All Sessions
[**revoke_revoke**](SessionApi.md#revoke_revoke) | **DELETE** /auth/session/{id} | Revoke Session



## edit_edit

> crate::models::SessionInfo edit_edit(id, data_edit_session)
Edit Session

Edit current session information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**data_edit_session** | [**DataEditSession**](DataEditSession.md) |  | [required] |

### Return type

[**crate::models::SessionInfo**](SessionInfo.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_all_fetch_all

> Vec<crate::models::SessionInfo> fetch_all_fetch_all()
Fetch Sessions

Fetch all sessions associated with this account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SessionInfo>**](SessionInfo.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_login

> crate::models::ResponseLogin login_login(data_login)
Login

Login to an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_login** | [**DataLogin**](DataLogin.md) |  | [required] |

### Return type

[**crate::models::ResponseLogin**](ResponseLogin.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout_logout

> logout_logout()
Logout

Delete current session.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_all_revoke_all

> revoke_all_revoke_all(revoke_self)
Delete All Sessions

Delete all active sessions, optionally including current one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revoke_self** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_revoke

> revoke_revoke(id)
Revoke Session

Delete a specific active session.

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

