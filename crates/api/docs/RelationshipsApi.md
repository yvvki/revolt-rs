# \RelationshipsApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_friend_req**](RelationshipsApi.md#add_friend_req) | **PUT** /users/{target}/friend | Accept Friend Request
[**block_user_req**](RelationshipsApi.md#block_user_req) | **PUT** /users/{target}/block | Block User
[**find_mutual_req**](RelationshipsApi.md#find_mutual_req) | **GET** /users/{target}/mutual | Fetch Mutual Friends And Servers
[**remove_friend_req**](RelationshipsApi.md#remove_friend_req) | **DELETE** /users/{target}/friend | Deny Friend Request / Remove Friend
[**send_friend_request_req**](RelationshipsApi.md#send_friend_request_req) | **POST** /users/friend | Send Friend Request
[**unblock_user_req**](RelationshipsApi.md#unblock_user_req) | **DELETE** /users/{target}/block | Unblock User



## add_friend_req

> crate::models::User add_friend_req(target)
Accept Friend Request

Accept another user's friend request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_user_req

> crate::models::User block_user_req(target)
Block User

Block another user by their id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_mutual_req

> crate::models::MutualResponse find_mutual_req(target)
Fetch Mutual Friends And Servers

Retrieve a list of mutual friends and servers with another user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::MutualResponse**](MutualResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_friend_req

> crate::models::User remove_friend_req(target)
Deny Friend Request / Remove Friend

Denies another user's friend request or removes an existing friend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_friend_request_req

> crate::models::User send_friend_request_req(data_send_friend_request)
Send Friend Request

Send a friend request to another user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_send_friend_request** | [**DataSendFriendRequest**](DataSendFriendRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unblock_user_req

> crate::models::User unblock_user_req(target)
Unblock User

Unblock another user by their id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

