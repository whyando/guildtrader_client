# \UserApi

All URIs are relative to *https://guildtrader.whyando.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**profile**](UserApi.md#profile) | **GET** /api/user | Get player's profile and inventory
[**register**](UserApi.md#register) | **POST** /api/register | Register a new player



## profile

> models::User profile()
Get player's profile and inventory

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::User**](User.md)

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register

> models::Register201Response register(register_request)
Register a new player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_request** | [**RegisterRequest**](RegisterRequest.md) |  | [required] |

### Return type

[**models::Register201Response**](register_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

