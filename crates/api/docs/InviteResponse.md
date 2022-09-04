# InviteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** |  | 
**code** | **String** | Invite code | 
**server_id** | **String** | Id of the server | 
**server_name** | **String** | Name of the server | 
**server_icon** | Option<[**crate::models::File**](File.md)> | Attachment for server icon | [optional]
**server_banner** | Option<[**crate::models::File**](File.md)> | Attachment for server banner | [optional]
**channel_id** | **String** | Id of group channel | 
**channel_name** | **String** | Name of group channel | 
**channel_description** | Option<**String**> | Description of group channel | [optional]
**user_name** | **String** | Name of user who created the invite | 
**user_avatar** | Option<[**crate::models::File**](File.md)> | Avatar of the user who created the invite | [optional]
**member_count** | **i64** | Number of members in this server | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


