# DataLogin

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Email | 
**password** | **String** | Password | 
**captcha** | Option<**String**> | Captcha verification code | [optional]
**friendly_name** | Option<**String**> | Friendly name used for the session | [optional]
**mfa_ticket** | **String** | Unvalidated MFA ticket  Used to resolve the correct account | 
**mfa_response** | Option<[**crate::models::MfaResponse**](MFAResponse.md)> | Valid MFA response  This will take precedence over the `password` field where applicable | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


