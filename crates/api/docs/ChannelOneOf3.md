# ChannelOneOf3

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel_type** | **String** |  | 
**_id** | **String** | Unique Id | 
**server** | **String** | Id of the server this channel belongs to | 
**name** | **String** | Display name of the channel | 
**description** | Option<**String**> | Channel description | [optional]
**icon** | Option<[**crate::models::File**](File.md)> | Custom icon attachment | [optional]
**last_message_id** | Option<**String**> | Id of the last message sent in this channel | [optional]
**default_permissions** | Option<[**crate::models::OverrideField**](OverrideField.md)> | Default permissions assigned to users in this channel | [optional]
**role_permissions** | Option<[**::std::collections::HashMap<String, crate::models::OverrideField>**](OverrideField.md)> | Permissions assigned based on role to this channel | [optional]
**nsfw** | Option<**bool**> | Whether this channel is marked as not safe for work | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


