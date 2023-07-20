# ChannelOneOf4

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel_type** | **String** |  | 
**_id** | **String** | Unique Id | 
**server** | **String** | Id of the server this channel belongs to | 
**name** | **String** | Display name of the channel | 
**description** | Option<**String**> | Channel description | [optional]
**icon** | Option<[**crate::models::ChannelOneOf2Icon**](Channel_oneOf_2_icon.md)> |  | [optional]
**default_permissions** | Option<[**crate::models::ChannelOneOf3DefaultPermissions**](Channel_oneOf_3_default_permissions.md)> |  | [optional]
**role_permissions** | Option<[**::std::collections::HashMap<String, crate::models::OverrideField>**](OverrideField.md)> | Permissions assigned based on role to this channel | [optional]
**nsfw** | Option<**bool**> | Whether this channel is marked as not safe for work | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


