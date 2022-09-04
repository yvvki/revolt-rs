# DataMessageSend

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nonce** | Option<**String**> | Unique token to prevent duplicate message sending  **This is deprecated and replaced by `Idempotency-Key`!** | [optional]
**content** | Option<**String**> | Message content to send | [optional]
**attachments** | Option<**Vec<String>**> | Attachments to include in message | [optional]
**replies** | Option<[**Vec<crate::models::Reply>**](Reply.md)> | Messages to reply to | [optional]
**embeds** | Option<[**Vec<crate::models::SendableEmbed>**](SendableEmbed.md)> | Embeds to include in message  Text embed content contributes to the content length cap | [optional]
**masquerade** | Option<[**crate::models::DataMessageSendMasquerade**](DataMessageSend_masquerade.md)> |  | [optional]
**interactions** | Option<[**crate::models::DataMessageSendInteractions**](DataMessageSend_interactions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


