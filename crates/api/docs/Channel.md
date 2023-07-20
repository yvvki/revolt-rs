# Channel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel_type** | **String** |  | 
**_id** | **String** | Unique Id | 
**user** | **String** | Id of the user this channel belongs to | 
**active** | **bool** | Whether this direct message channel is currently open on both sides | 
**recipients** | **Vec<String>** | Array of user ids participating in channel | 
**last_message_id** | Option<**String**> | Id of the last message sent in this channel | [optional]
**name** | **String** | Display name of the channel | 
**owner** | **String** | User id of the owner of the group | 
**description** | Option<**String**> | Channel description | [optional]
**icon** | Option<[**crate::models::ChannelOneOf2Icon**](Channel_oneOf_2_icon.md)> |  | [optional]
**permissions** | Option<**i64**> | Permissions assigned to members of this group (does not apply to the owner of the group) | [optional]
**nsfw** | Option<**bool**> | Whether this channel is marked as not safe for work | [optional]
**server** | **String** | Id of the server this channel belongs to | 
**default_permissions** | Option<[**crate::models::ChannelOneOf3DefaultPermissions**](Channel_oneOf_3_default_permissions.md)> |  | [optional]
**role_permissions** | Option<[**::std::collections::HashMap<String, crate::models::OverrideField>**](OverrideField.md)> | Permissions assigned based on role to this channel | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


