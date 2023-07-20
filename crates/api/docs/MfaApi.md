# \MfaApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ticket_create_ticket**](MfaApi.md#create_ticket_create_ticket) | **PUT** /auth/mfa/ticket | Create MFA ticket
[**fetch_recovery_fetch_recovery**](MfaApi.md#fetch_recovery_fetch_recovery) | **POST** /auth/mfa/recovery | Fetch Recovery Codes
[**fetch_status_fetch_status**](MfaApi.md#fetch_status_fetch_status) | **GET** /auth/mfa/ | MFA Status
[**generate_recovery_generate_recovery**](MfaApi.md#generate_recovery_generate_recovery) | **PATCH** /auth/mfa/recovery | Generate Recovery Codes
[**get_mfa_methods_get_mfa_methods**](MfaApi.md#get_mfa_methods_get_mfa_methods) | **GET** /auth/mfa/methods | Get MFA Methods
[**totp_disable_totp_disable**](MfaApi.md#totp_disable_totp_disable) | **DELETE** /auth/mfa/totp | Disable TOTP 2FA
[**totp_enable_totp_enable**](MfaApi.md#totp_enable_totp_enable) | **PUT** /auth/mfa/totp | Enable TOTP 2FA
[**totp_generate_secret_totp_generate_secret**](MfaApi.md#totp_generate_secret_totp_generate_secret) | **POST** /auth/mfa/totp | Generate TOTP Secret



## create_ticket_create_ticket

> crate::models::MfaTicket create_ticket_create_ticket(mfa_response)
Create MFA ticket

Create a new MFA ticket or validate an existing one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_response** | [**MfaResponse**](MfaResponse.md) |  | [required] |

### Return type

[**crate::models::MfaTicket**](MFATicket.md)

### Authorization

[Session Token](../README.md#Session Token), [Unvalidated MFA Ticket](../README.md#Unvalidated MFA Ticket)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_recovery_fetch_recovery

> Vec<String> fetch_recovery_fetch_recovery()
Fetch Recovery Codes

Fetch recovery codes for an account.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[Valid MFA Ticket](../README.md#Valid MFA Ticket), [Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_status_fetch_status

> crate::models::MultiFactorStatus fetch_status_fetch_status()
MFA Status

Fetch MFA status of an account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MultiFactorStatus**](MultiFactorStatus.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_recovery_generate_recovery

> Vec<String> generate_recovery_generate_recovery()
Generate Recovery Codes

Re-generate recovery codes for an account.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[Valid MFA Ticket](../README.md#Valid MFA Ticket), [Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mfa_methods_get_mfa_methods

> Vec<crate::models::MfaMethod> get_mfa_methods_get_mfa_methods()
Get MFA Methods

Fetch available MFA methods.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::MfaMethod>**](MFAMethod.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## totp_disable_totp_disable

> totp_disable_totp_disable()
Disable TOTP 2FA

Disable TOTP 2FA for an account.

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


## totp_enable_totp_enable

> totp_enable_totp_enable(mfa_response)
Enable TOTP 2FA

Generate a new secret for TOTP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_response** | [**MfaResponse**](MfaResponse.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## totp_generate_secret_totp_generate_secret

> crate::models::ResponseTotpSecret totp_generate_secret_totp_generate_secret()
Generate TOTP Secret

Generate a new secret for TOTP.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ResponseTotpSecret**](ResponseTotpSecret.md)

### Authorization

[Valid MFA Ticket](../README.md#Valid MFA Ticket), [Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

