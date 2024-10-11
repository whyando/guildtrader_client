# \GuildApi

All URIs are relative to *https://guildtrader.whyando.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel**](GuildApi.md#cancel) | **POST** /api/guild/contract/cancel | Cancel a contract
[**collect**](GuildApi.md#collect) | **POST** /api/guild/contract/collect | Collect a contract
[**contract**](GuildApi.md#contract) | **POST** /api/guild/contract | Engage in a guild contract
[**contracts**](GuildApi.md#contracts) | **GET** /api/guild/contracts | Get all the player's active contracts
[**list**](GuildApi.md#list) | **GET** /api/guilds | List all guilds



## cancel

> models::Cancel200Response cancel(cancel_request)
Cancel a contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cancel_request** | [**CancelRequest**](CancelRequest.md) |  | [required] |

### Return type

[**models::Cancel200Response**](cancel_200_response.md)

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collect

> models::Cancel200Response collect(cancel_request)
Collect a contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cancel_request** | [**CancelRequest**](CancelRequest.md) |  | [required] |

### Return type

[**models::Cancel200Response**](cancel_200_response.md)

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract

> models::Contract contract(contract_request)
Engage in a guild contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_request** | [**ContractRequest**](ContractRequest.md) |  | [required] |

### Return type

[**models::Contract**](Contract.md)

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts

> models::Contracts200Response contracts()
Get all the player's active contracts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Contracts200Response**](contracts_200_response.md)

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> models::List200Response list()
List all guilds

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::List200Response**](list_200_response.md)

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

