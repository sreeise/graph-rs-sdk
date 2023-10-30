# Paging Additional Information

For paging, the response bodies are returned in a result, `Result<T, ErrorMessage>` when calling `body()` or `into_body()`
where errors are typically due to deserialization when the Graph Api returns error messages in the `Response` body.
For instance, if you were to call the Graph Api using paging with a custom type and your access token has already
expired the response body will be an error because the response body could not be converted to your custom type.
Because of the way Microsoft Graph returns errors as `Response` bodies, using `serde_json::Value`, for paging
calls will return those errors as `Ok(serde_json::Value)` instead of `Err(ErrorMessage)`. So just keep
this in mind if you do a paging call and specify the body as `serde_json::Value`.

If you get an unsuccessful status code from the `Response` object you can typically assume
that your response body is an error. With paging, the `Result<T, ErrorMessage>` will include any
Microsoft Graph specific error from the Response body in `ErrorMessage` except when you specify
`serde_json::Value` as the type for `Response` body in the paging call as mentioned above.

You can however almost always get original response body using `serde_json::Value` from a paging call because
this sdk stores the response in a `serde_json::Value`, transferred in `Response` as `Vec<u8>`,
for each `Response`. To get the original response body as `serde_json::Value` when using custom types, first
add a use statement for `HttpResponseExt`, the sdk trait for `http::Response`: `use graph_rs_sdk::http::HttpResponseExt;`
call the `json` method on the `http::Response<Result<T, ErrorMessage>>` which returns an `Option<serde_json::Value>`.
This `serde_json::Value`, in unsuccessful responses, will almost always be the Microsoft Graph Error.
You can convert this `serde_json::Value` to the provided type, `ErrorMessage`,
from `graph_rs_sdk::error::ErrorMessage`, or to whatever type you choose.

```rust
use graph_rs_sdk::http::HttpResponseExt;

fn main() {
  // Given response = http::Response<T>>
  println!("{:#?}", response.url()); // Get the url of the request.
  println!("{:#?}", response.json()); // Get the original JSON that came in the Response
}
```

Performance wise, It is better to use `http::Response::body()` and `http::Response::into_body()` for any type,
whether its custom types or `serde_json::Value`, instead of `HttpResponseExt::json()` because
in successful responses the body from `body()` or `into_body()` has already been converted.
The `HttpResponseExt::json` method must convert from `Vec<u8>`.
In general, this method can be used for any use case. However, its provided if needed for debugging and
for error messages that Microsoft Graph returns.

There are different levels of support for paging Microsoft Graph APIs. See the documentation,
[Paging Microsoft Graph data in your app](https://learn.microsoft.com/en-us/graph/paging), for more info on
supported APIs and availability.
