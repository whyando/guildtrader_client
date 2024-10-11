# \TradeApi

All URIs are relative to *https://guildtrader.whyando.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assets**](TradeApi.md#assets) | **GET** /api/market | Get a list of available assets and their current market prices.
[**buy**](TradeApi.md#buy) | **POST** /api/market/buy | Buy an asset at the current market price
[**sell**](TradeApi.md#sell) | **POST** /api/market/sell | Sell an asset at the current market price



## assets

> models::Assets200Response assets()
Get a list of available assets and their current market prices.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Assets200Response**](assets_200_response.md)

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## buy

> models::Buy200Response buy(buy_request)
Buy an asset at the current market price

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**buy_request** | [**BuyRequest**](BuyRequest.md) |  | [required] |

### Return type

[**models::Buy200Response**](buy_200_response.md)

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sell

> models::Buy200Response sell(buy_request)
Sell an asset at the current market price

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**buy_request** | [**BuyRequest**](BuyRequest.md) |  | [required] |

### Return type

[**models::Buy200Response**](buy_200_response.md)

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

