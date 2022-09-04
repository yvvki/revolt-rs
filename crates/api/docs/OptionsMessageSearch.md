# OptionsMessageSearch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | **String** | Full-text search query  See [MongoDB documentation](https://docs.mongodb.com/manual/text-search/#-text-operator) for more information. | 
**limit** | Option<**i64**> | Maximum number of messages to fetch | [optional]
**before** | Option<**String**> | Message id before which messages should be fetched | [optional]
**after** | Option<**String**> | Message id after which messages should be fetched | [optional]
**sort** | Option<[**crate::models::MessageSort**](MessageSort.md)> | Message sort direction  By default, it will be sorted by relevance. | [optional]
**include_users** | Option<**bool**> | Whether to include user (and member, if server channel) objects | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


