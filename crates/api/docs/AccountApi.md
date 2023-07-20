# \AccountApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_email_change_email**](AccountApi.md#change_email_change_email) | **PATCH** /auth/account/change/email | Change Email
[**change_password_change_password**](AccountApi.md#change_password_change_password) | **PATCH** /auth/account/change/password | Change Password
[**confirm_deletion_confirm_deletion**](AccountApi.md#confirm_deletion_confirm_deletion) | **PUT** /auth/account/delete | Confirm Account Deletion
[**create_account_create_account**](AccountApi.md#create_account_create_account) | **POST** /auth/account/create | Create Account
[**delete_account_delete_account**](AccountApi.md#delete_account_delete_account) | **POST** /auth/account/delete | Delete Account
[**disable_account_disable_account**](AccountApi.md#disable_account_disable_account) | **POST** /auth/account/disable | Disable Account
[**fetch_account_fetch_account**](AccountApi.md#fetch_account_fetch_account) | **GET** /auth/account/ | Fetch Account
[**password_reset_password_reset**](AccountApi.md#password_reset_password_reset) | **PATCH** /auth/account/reset_password | Password Reset
[**resend_verification_resend_verification**](AccountApi.md#resend_verification_resend_verification) | **POST** /auth/account/reverify | Resend Verification
[**send_password_reset_send_password_reset**](AccountApi.md#send_password_reset_send_password_reset) | **POST** /auth/account/reset_password | Send Password Reset
[**verify_email_verify_email**](AccountApi.md#verify_email_verify_email) | **POST** /auth/account/verify/{code} | Verify Email



## change_email_change_email

> change_email_change_email(data_change_email)
Change Email

Change the associated account email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_change_email** | [**DataChangeEmail**](DataChangeEmail.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_password_change_password

> change_password_change_password(data_change_password)
Change Password

Change the current account password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_change_password** | [**DataChangePassword**](DataChangePassword.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## confirm_deletion_confirm_deletion

> confirm_deletion_confirm_deletion(data_account_deletion)
Confirm Account Deletion

Schedule an account for deletion by confirming the received token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_account_deletion** | [**DataAccountDeletion**](DataAccountDeletion.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_account_create_account

> create_account_create_account(data_create_account)
Create Account

Create a new account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_create_account** | [**DataCreateAccount**](DataCreateAccount.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_account_delete_account

> delete_account_delete_account()
Delete Account

Request to have an account deleted.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Valid MFA Ticket](../README.md#Valid MFA Ticket), [Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_account_disable_account

> disable_account_disable_account()
Disable Account

Disable an account.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Valid MFA Ticket](../README.md#Valid MFA Ticket), [Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_account_fetch_account

> crate::models::AccountInfo fetch_account_fetch_account()
Fetch Account

Fetch account information from the current session.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccountInfo**](AccountInfo.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## password_reset_password_reset

> password_reset_password_reset(data_password_reset)
Password Reset

Confirm password reset and change the password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_password_reset** | [**DataPasswordReset**](DataPasswordReset.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_verification_resend_verification

> resend_verification_resend_verification(data_resend_verification)
Resend Verification

Resend account creation verification email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_resend_verification** | [**DataResendVerification**](DataResendVerification.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_password_reset_send_password_reset

> send_password_reset_send_password_reset(data_send_password_reset)
Send Password Reset

Send an email to reset account password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_send_password_reset** | [**DataSendPasswordReset**](DataSendPasswordReset.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_email_verify_email

> crate::models::ResponseVerify verify_email_verify_email(code)
Verify Email

Verify an email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

[**crate::models::ResponseVerify**](ResponseVerify.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

