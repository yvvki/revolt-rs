# FetchServerResponseAnyOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channels** | **Vec<String>** | Channels within this server | 
**_id** | **String** | Unique Id | 
**owner** | **String** | User id of the owner | 
**name** | **String** | Name of the server | 
**description** | Option<**String**> | Description for the server | [optional]
**categories** | Option<[**Vec<crate::models::Category>**](Category.md)> | Categories for this server | [optional]
**system_messages** | Option<[**crate::models::ServerSystemMessages**](Server_system_messages.md)> |  | [optional]
**roles** | Option<[**::std::collections::HashMap<String, crate::models::Role>**](Role.md)> | Roles for this server | [optional]
**default_permissions** | **i64** | Default set of server and channel permissions | 
**icon** | Option<[**crate::models::ServerIcon**](Server_icon.md)> |  | [optional]
**banner** | Option<[**crate::models::ServerBanner**](Server_banner.md)> |  | [optional]
**flags** | Option<**i32**> | Bitfield of server flags | [optional]
**nsfw** | Option<**bool**> | Whether this server is flagged as not safe for work | [optional]
**analytics** | Option<**bool**> | Whether to enable analytics | [optional]
**discoverable** | Option<**bool**> | Whether this server should be publicly discoverable | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


