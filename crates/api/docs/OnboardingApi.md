# \OnboardingApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_req**](OnboardingApi.md#complete_req) | **POST** /onboard/complete | Complete Onboarding
[**hello_req**](OnboardingApi.md#hello_req) | **GET** /onboard/hello | Check Onboarding Status



## complete_req

> complete_req(data_onboard)
Complete Onboarding

This sets a new username, completes onboarding and allows a user to start using Revolt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_onboard** | [**DataOnboard**](DataOnboard.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hello_req

> crate::models::DataHello hello_req()
Check Onboarding Status

This will tell you whether the current account requires onboarding or whether you can continue to send requests as usual. You may skip calling this if you're restoring an existing session.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DataHello**](DataHello.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

