# \UserSafetyApi

All URIs are relative to *https://api.revolt.chat*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_strike_create_strike**](UserSafetyApi.md#create_strike_create_strike) | **POST** /safety/strikes | Create Strike
[**delete_strike_delete_strike**](UserSafetyApi.md#delete_strike_delete_strike) | **DELETE** /safety/strikes/{strike_id} | Delete Strike
[**edit_report_edit_report**](UserSafetyApi.md#edit_report_edit_report) | **PATCH** /safety/reports/{report} | Edit Report
[**edit_strike_edit_strike**](UserSafetyApi.md#edit_strike_edit_strike) | **POST** /safety/strikes/{strike_id} | Edit Strike
[**fetch_report_fetch_report**](UserSafetyApi.md#fetch_report_fetch_report) | **GET** /safety/report/{id} | Fetch Report
[**fetch_reports_fetch_reports**](UserSafetyApi.md#fetch_reports_fetch_reports) | **GET** /safety/reports | Fetch Reports
[**fetch_snapshots_fetch_snapshots**](UserSafetyApi.md#fetch_snapshots_fetch_snapshots) | **GET** /safety/snapshot/{report_id} | Fetch Snapshots
[**fetch_strikes_fetch_strikes**](UserSafetyApi.md#fetch_strikes_fetch_strikes) | **GET** /safety/strikes/{user_id} | Fetch Strikes
[**report_content_report_content**](UserSafetyApi.md#report_content_report_content) | **POST** /safety/report | Report Content



## create_strike_create_strike

> crate::models::AccountStrike create_strike_create_strike(data_create_strike)
Create Strike

Create a new account strike

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_create_strike** | [**DataCreateStrike**](DataCreateStrike.md) |  | [required] |

### Return type

[**crate::models::AccountStrike**](AccountStrike.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_strike_delete_strike

> delete_strike_delete_strike(strike_id)
Delete Strike

Delete a strike by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**strike_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_report_edit_report

> crate::models::Report edit_report_edit_report(report, data_edit_report)
Edit Report

Edit a report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report** | **String** |  | [required] |
**data_edit_report** | [**DataEditReport**](DataEditReport.md) |  | [required] |

### Return type

[**crate::models::Report**](Report.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_strike_edit_strike

> edit_strike_edit_strike(strike_id, data_edit_account_strike)
Edit Strike

Edit a strike by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**strike_id** | **String** |  | [required] |
**data_edit_account_strike** | [**DataEditAccountStrike**](DataEditAccountStrike.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_report_fetch_report

> crate::models::Report fetch_report_fetch_report(id)
Fetch Report

Fetch a report by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Report**](Report.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_reports_fetch_reports

> Vec<crate::models::Report> fetch_reports_fetch_reports(content_id, author_id, status)
Fetch Reports

Fetch all available reports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_id** | Option<**String**> | Find reports against messages, servers, or users |  |
**author_id** | Option<**String**> | Find reports created by user |  |
**status** | Option<[**ReportStatusString**](.md)> | Report status to include in search |  |

### Return type

[**Vec<crate::models::Report>**](Report.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_snapshots_fetch_snapshots

> Vec<crate::models::SnapshotWithContext> fetch_snapshots_fetch_snapshots(report_id)
Fetch Snapshots

Fetch a snapshots for a given report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::SnapshotWithContext>**](SnapshotWithContext.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_strikes_fetch_strikes

> Vec<crate::models::AccountStrike> fetch_strikes_fetch_strikes(user_id)
Fetch Strikes

Fetch strikes for a user by their ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::AccountStrike>**](AccountStrike.md)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_content_report_content

> report_content_report_content(data_report_content)
Report Content

Report a piece of content to the moderation team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_report_content** | [**DataReportContent**](DataReportContent.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Session Token](../README.md#Session Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

