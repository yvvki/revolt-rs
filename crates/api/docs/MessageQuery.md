# MessageQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Maximum number of messages to fetch  For fetching nearby messages, this is \\`(limit + 1)\\`. | [optional]
**channel** | Option<**String**> | Parent channel ID | [optional]
**author** | Option<**String**> | Message author ID | [optional]
**query** | Option<**String**> | Search query | [optional]
**nearby** | **String** | Message id to search around  Specifying 'nearby' ignores 'before', 'after' and 'sort'. It will also take half of limit rounded as the limits to each side. It also fetches the message ID specified. | 
**before** | Option<**String**> | Message id before which messages should be fetched | [optional]
**after** | Option<**String**> | Message id after which messages should be fetched | [optional]
**sort** | Option<[**crate::models::MessageSort**](MessageSort.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


