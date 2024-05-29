# \CrateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_joke**](CrateApi.md#delete_joke) | **DELETE** /api/v1/joke/{id} | 
[**get_joke**](CrateApi.md#get_joke) | **GET** /api/v1/joke/{id} | 
[**joke**](CrateApi.md#joke) | **GET** /api/v1/joke | 
[**jokes**](CrateApi.md#jokes) | **GET** /api/v1/jokes | 
[**post_joke**](CrateApi.md#post_joke) | **POST** /api/v1/joke/add | 
[**update_joke**](CrateApi.md#update_joke) | **PUT** /api/v1/joke/{id} | 



## delete_joke

> serde_json::Value delete_joke(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_joke

> models::Joke get_joke(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Joke**](Joke.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## joke

> models::Joke joke()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Joke**](Joke.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jokes

> Vec<models::Joke> jokes()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Joke>**](Joke.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_joke

> serde_json::Value post_joke(post_joke_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_joke_request** | [**PostJokeRequest**](PostJokeRequest.md) | Joke to add | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_joke

> serde_json::Value update_joke(id, post_joke_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**post_joke_request** | [**PostJokeRequest**](PostJokeRequest.md) | Joke to update | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

