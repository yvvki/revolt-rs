# InviteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**code** | **String** | Invite code | 
**server_id** | **String** | Id of the server | 
**server_name** | **String** | Name of the server | 
**server_icon** | Option<[**crate::models::InviteResponseOneOfServerIcon**](InviteResponse_oneOf_server_icon.md)> |  | [optional]
**server_banner** | Option<[**crate::models::InviteResponseOneOfServerBanner**](InviteResponse_oneOf_server_banner.md)> |  | [optional]
**server_flags** | Option<**i32**> | Enum of server flags | [optional]
**channel_id** | **String** | Id of group channel | 
**channel_name** | **String** | Name of group channel | 
**channel_description** | Option<**String**> | Description of group channel | [optional]
**user_name** | **String** | Name of user who created the invite | 
**user_avatar** | Option<[**crate::models::InviteResponseOneOfUserAvatar**](InviteResponse_oneOf_user_avatar.md)> |  | [optional]
**member_count** | **i64** | Number of members in this server | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


