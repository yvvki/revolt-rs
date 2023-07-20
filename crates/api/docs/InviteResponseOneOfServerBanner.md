# InviteResponseOneOfServerBanner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** | Unique Id | 
**tag** | **String** | Tag / bucket this file was uploaded to | 
**filename** | **String** | Original filename | 
**metadata** | [**crate::models::FileMetadata**](File_metadata.md) |  | 
**content_type** | **String** | Raw content type of this file | 
**size** | **i32** | Size of this file (in bytes) | 
**deleted** | Option<**bool**> | Whether this file was deleted | [optional]
**reported** | Option<**bool**> | Whether this file was reported | [optional]
**message_id** | Option<**String**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**server_id** | Option<**String**> |  | [optional]
**object_id** | Option<**String**> | Id of the object this file is associated with | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


