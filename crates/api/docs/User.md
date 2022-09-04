# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** | Unique Id | 
**username** | **String** | Username | 
**avatar** | Option<[**crate::models::UserAvatar**](User_avatar.md)> |  | [optional]
**relations** | Option<[**Vec<crate::models::Relationship>**](Relationship.md)> | Relationships with other users | [optional]
**badges** | Option<**i32**> | Bitfield of user badges | [optional]
**status** | Option<[**crate::models::UserStatus**](User_status.md)> |  | [optional]
**profile** | Option<[**crate::models::UserProfile**](User_profile.md)> |  | [optional]
**flags** | Option<**i32**> | Enum of user flags | [optional]
**privileged** | Option<**bool**> | Whether this user is privileged | [optional]
**bot** | Option<[**crate::models::UserBot**](User_bot.md)> |  | [optional]
**relationship** | Option<[**crate::models::RelationshipStatus**](RelationshipStatus.md)> | Current session user's relationship with this user | [optional]
**online** | Option<**bool**> | Whether this user is currently online | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


