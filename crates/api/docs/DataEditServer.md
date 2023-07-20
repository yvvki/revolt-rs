# DataEditServer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Server name | [optional]
**description** | Option<**String**> | Server description | [optional]
**icon** | Option<**String**> | Attachment Id for icon | [optional]
**banner** | Option<**String**> | Attachment Id for banner | [optional]
**categories** | Option<[**Vec<crate::models::Category>**](Category.md)> | Category structure for server | [optional]
**system_messages** | Option<[**crate::models::DataEditServerSystemMessages**](DataEditServer_system_messages.md)> |  | [optional]
**flags** | Option<**i32**> | Bitfield of server flags | [optional]
**discoverable** | Option<**bool**> | Whether this server is public and should show up on [Revolt Discover](https://rvlt.gg) | [optional]
**analytics** | Option<**bool**> | Whether analytics should be collected for this server  Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg). | [optional]
**remove** | Option<[**Vec<crate::models::FieldsServer>**](FieldsServer.md)> | Fields to remove from server object | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


