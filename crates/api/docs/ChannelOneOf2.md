# ChannelOneOf2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel_type** | **String** |  | 
**_id** | **String** | Unique Id | 
**name** | **String** | Display name of the channel | 
**owner** | **String** | User id of the owner of the group | 
**description** | Option<**String**> | Channel description | [optional]
**recipients** | **Vec<String>** | Array of user ids participating in channel | 
**icon** | Option<[**crate::models::File**](File.md)> | Custom icon attachment | [optional]
**last_message_id** | Option<**String**> | Id of the last message sent in this channel | [optional]
**permissions** | Option<**i64**> | Permissions assigned to members of this group (does not apply to the owner of the group) | [optional]
**nsfw** | Option<**bool**> | Whether this group is marked as not safe for work | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


