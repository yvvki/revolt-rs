# \ServerPermissionsApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**permissions_set_default_req**](ServerPermissionsApi.md#permissions_set_default_req) | **PUT** /servers/{target}/permissions/default | Set Default Permission
[**permissions_set_req**](ServerPermissionsApi.md#permissions_set_req) | **PUT** /servers/{target}/permissions/{role_id} | Set Role Permission
[**roles_create_req**](ServerPermissionsApi.md#roles_create_req) | **POST** /servers/{target}/roles | Create Role
[**roles_delete_req**](ServerPermissionsApi.md#roles_delete_req) | **DELETE** /servers/{target}/roles/{role_id} | Delete Role
[**roles_edit_req**](ServerPermissionsApi.md#roles_edit_req) | **PATCH** /servers/{target}/roles/{role_id} | Edit Role



## permissions_set_default_req

> crate::models::Server permissions_set_default_req(target, data_permissions_value)
Set Default Permission

Sets permissions for the default role in this server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**data_permissions_value** | [**DataPermissionsValue**](DataPermissionsValue.md) |  | [required] |

### Return type

[**crate::models::Server**](Server.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permissions_set_req

> crate::models::Server permissions_set_req(target, role_id, data_set_server_role_permission)
Set Role Permission

Sets permissions for the specified role in the server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**data_set_server_role_permission** | [**DataSetServerRolePermission**](DataSetServerRolePermission.md) |  | [required] |

### Return type

[**crate::models::Server**](Server.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## roles_create_req

> crate::models::NewRoleResponse roles_create_req(target, data_create_role)
Create Role

Creates a new server role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**data_create_role** | [**DataCreateRole**](DataCreateRole.md) |  | [required] |

### Return type

[**crate::models::NewRoleResponse**](NewRoleResponse.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## roles_delete_req

> roles_delete_req(target, role_id)
Delete Role

Delete a server role by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**role_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## roles_edit_req

> crate::models::Role roles_edit_req(target, role_id, data_edit_role)
Edit Role

Edit a role by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**data_edit_role** | [**DataEditRole**](DataEditRole.md) |  | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

