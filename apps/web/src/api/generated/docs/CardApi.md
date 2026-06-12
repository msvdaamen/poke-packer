# CardApi

All URIs are relative to *http://localhost:3000*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**listCards**](CardApi.md#listcards) | **GET** /cards/ |  |



## listCards

> Array&lt;Card&gt; listCards()



### Example

```ts
import {
  Configuration,
  CardApi,
} from '';
import type { ListCardsRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new CardApi();

  try {
    const data = await api.listCards();
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**Array&lt;Card&gt;**](Card.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | The available cards. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

