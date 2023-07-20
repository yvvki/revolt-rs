# \BotsApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_create_bot**](BotsApi.md#create_create_bot) | **POST** /bots/create | Create Bot
[**delete_delete_bot**](BotsApi.md#delete_delete_bot) | **DELETE** /bots/{target} | Delete Bot
[**edit_edit_bot**](BotsApi.md#edit_edit_bot) | **PATCH** /bots/{target} | Edit Bot
[**fetch_fetch_bot**](BotsApi.md#fetch_fetch_bot) | **GET** /bots/{bot} | Fetch Bot
[**fetch_owned_fetch_owned_bots**](BotsApi.md#fetch_owned_fetch_owned_bots) | **GET** /bots/@me | Fetch Owned Bots
[**fetch_public_fetch_public_bot**](BotsApi.md#fetch_public_fetch_public_bot) | **GET** /bots/{target}/invite | Fetch Public Bot
[**invite_invite_bot**](BotsApi.md#invite_invite_bot) | **POST** /bots/{target}/invite | Invite Bot



## create_create_bot

> crate::models::Bot create_create_bot(data_create_bot)
Create Bot

Create a new Revolt bot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_create_bot** | [**DataCreateBot**](DataCreateBot.md) |  | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_delete_bot

> delete_delete_bot(target)
Delete Bot

Delete a bot by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_edit_bot

> crate::models::Bot edit_edit_bot(target, data_edit_bot)
Edit Bot

Edit bot details by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**data_edit_bot** | [**DataEditBot**](DataEditBot.md) |  | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_fetch_bot

> crate::models::FetchBotResponse fetch_fetch_bot(bot)
Fetch Bot

Fetch details of a bot you own by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot** | **String** |  | [required] |

### Return type

[**crate::models::FetchBotResponse**](FetchBotResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_owned_fetch_owned_bots

> crate::models::OwnedBotsResponse fetch_owned_fetch_owned_bots()
Fetch Owned Bots

Fetch all of the bots that you have control over.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OwnedBotsResponse**](OwnedBotsResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_public_fetch_public_bot

> crate::models::PublicBot fetch_public_fetch_public_bot(target)
Fetch Public Bot

Fetch details of a public (or owned) bot by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::PublicBot**](PublicBot.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_invite_bot

> invite_invite_bot(target, invite_bot_destination)
Invite Bot

Invite a bot to a server or group by its id.`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**invite_bot_destination** | [**InviteBotDestination**](InviteBotDestination.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

