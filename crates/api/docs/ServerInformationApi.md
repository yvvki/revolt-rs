# \ServerInformationApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**channel_create_req**](ServerInformationApi.md#channel_create_req) | **POST** /servers/{target}/channels | Create Channel
[**server_ack_req**](ServerInformationApi.md#server_ack_req) | **PUT** /servers/{target}/ack | Mark Server As Read
[**server_create_req**](ServerInformationApi.md#server_create_req) | **POST** /servers/create | Create Server
[**server_delete_req**](ServerInformationApi.md#server_delete_req) | **DELETE** /servers/{target} | Delete / Leave Server
[**server_edit_req**](ServerInformationApi.md#server_edit_req) | **PATCH** /servers/{target} | Edit Server
[**server_fetch_req**](ServerInformationApi.md#server_fetch_req) | **GET** /servers/{target} | Fetch Server



## channel_create_req

> crate::models::Channel channel_create_req(target, data_create_channel)
Create Channel

Create a new Text or Voice channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**data_create_channel** | [**DataCreateChannel**](DataCreateChannel.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_ack_req

> server_ack_req(target)
Mark Server As Read

Mark all channels in a server as read.

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


## server_create_req

> crate::models::CreateServerResponse server_create_req(data_create_server)
Create Server

Create a new server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_create_server** | [**DataCreateServer**](DataCreateServer.md) |  | [required] |

### Return type

[**crate::models::CreateServerResponse**](CreateServerResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_delete_req

> server_delete_req(target)
Delete / Leave Server

Deletes a server if owner otherwise leaves.

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


## server_edit_req

> crate::models::Server server_edit_req(target, data_edit_server)
Edit Server

Edit a server by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**data_edit_server** | [**DataEditServer**](DataEditServer.md) |  | [required] |

### Return type

[**crate::models::Server**](Server.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_fetch_req

> crate::models::Server server_fetch_req(target)
Fetch Server

Fetch a server by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |

### Return type

[**crate::models::Server**](Server.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

