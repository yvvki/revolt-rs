# MfaTicket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** | Unique Id | 
**account_id** | **String** | Account Id | 
**token** | **String** | Unique Token | 
**validated** | **bool** | Whether this ticket has been validated (can be used for account actions) | 
**authorised** | **bool** | Whether this ticket is authorised (can be used to log a user in) | 
**last_totp_code** | Option<**String**> | TOTP code at time of ticket creation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


