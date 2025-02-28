# Internet Computer HTTP Certification

## Defining CEL Expressions

[CEL](https://github.com/google/cel-spec) (Common Expression Language) is a portable expression language that can be used to enable different applications to more easily interoperate. It can be seen as the computation or expression counterpart to [Protocol Buffers](https://github.com/protocolbuffers/protobuf).

CEL expressions lie at the heart of the Internet Computer's HTTP certification system. They are used to define the conditions under which a request and response pair should be certified and what should be included from the corresponding request and response objects in the certification.

CEL expressions can be created in two ways, by using the [CEL builder](#using-the-cel-builder), or by directly creating a [CEL expression](#directly-creating-a-cel-expression).

### Converting CEL expressions to their `String` representation

Note that the `CelExpression` enum is not a CEL expression itself, but rather a Rust representation of a CEL expression. To convert a `CelExpression` into its `String` representation, use `CelExpression.to_string` or `create_cel_expr`. This applies to CEL expressions created both by the [CEL builder](#using-the-cel-builder) and [directly](#directly-creating-a-cel-expression).

```rust
use ic_http_certification::cel::CelExpression;

let cel_expr = CelExpression::DefaultCertification(None).to_string();
```

Alternatively:

```rust
use ic_http_certification::cel::{CelExpression, create_cel_expr};

let certification = CelExpression::DefaultCertification(None);
let cel_expr = create_cel_expr(&certification);
```

### Using the CEL builder

The CEL builder interface is provided to ease the creation of CEL expressions through an ergonmic interface. If this interface does not meet your needs, you can also [create CEL expressions directly](#directly-creating-a-cel-expression). To define a CEL expression, start with [DefaultCelBuilder]. This struct provides a set of methods that can be used to define how your request and response pair should be certified.

When certifying requests, the request body and method are always certified. To additionally certify request headers and query parameters, use `with_request_headers` and `with_request_query_parameters` respectively. Both methods take a [str] slice as an argument.

When certifying a response, the response body and status code are always certified. To additionally certify response headers, use `with_response_certification`. This method takes the `DefaultResponseCertification` enum as an argument. To specify header inclusions, use the `CertifiedResponseHeaders` variant of the `DefaultResponseCertification` enum. Or to certify all response headers, with some exclusions, use the `ResponseHeaderExclusions` variant of the `DefaultResponseCertification` enum. Both variants take a [str] slice as an argument.

#### Fully certified request / response pair

To define a fully certified request and response pair, including request headers, query parameters, and response headers use `DefaultCelBuilder::full_certification`. For example:

```rust
use ic_http_certification::{DefaultCelBuilder, DefaultResponseCertification};

let cel_expr = DefaultCelBuilder::full_certification()
    .with_request_headers(&["Accept", "Accept-Encoding", "If-Match"])
    .with_request_query_parameters(&["foo", "bar", "baz"])
    .with_response_certification(DefaultResponseCertification::CertifiedResponseHeaders(&[
        "Cache-Control",
        "ETag",
    ]))
    .build();
```

#### Partially certified request

Any number of request headers or request query parameters can be certified via `with_request_headers` and `with_request_query_parameters` respectively. Both methods will accept empty arrays, which is the same as not calling them at all. If `with_request_headers` is called with an empty array, or it is not called at all, then no request headers will be certified. Likewise for `with_request_query_parameters`, if it is called with an empty array, or not called at all, then no request query parameters will be certified. If both are called with an empty array, or neither are called, then only the request body and method will be certified.

For example, to certify only the request body and method:

```rust
use ic_http_certification::{DefaultCelBuilder, DefaultResponseCertification};

let cel_expr = DefaultCelBuilder::full_certification()
    .with_response_certification(DefaultResponseCertification::CertifiedResponseHeaders(&[
        "Cache-Control",
        "ETag",
    ]))
    .build();
```

Alternatively, this can be done more explicitly:

```rust
use ic_http_certification::{DefaultCelBuilder, DefaultResponseCertification};

let cel_expr = DefaultCelBuilder::full_certification()
    .with_request_headers(&[])
    .with_request_query_parameters(&[])
    .with_response_certification(DefaultResponseCertification::CertifiedResponseHeaders(&[
        "Cache-Control",
        "ETag",
    ]))
    .build();
```

#### Skipping request certification

Request certification can be skipped entirely by using `DefaultCelBuilder::response_certification` instead of `DefaultCelBuilder::full_certification`. For example:

```rust
use ic_http_certification::{DefaultCelBuilder, DefaultResponseCertification};

let cel_expr = DefaultCelBuilder::response_certification()
    .with_response_certification(DefaultResponseCertification::ResponseHeaderExclusions(&[
        "Date",
        "Cookie",
        "Set-Cookie",
    ]))
    .build();
```

#### Partially certified response

Similiarly to request certification, any number of response headers can be provided via the `CertifiedResponseHeaders` variant of the `DefaultResponseCertification` enum when calling `with_response_certification`. The provided array can also be an empty. If the array is empty, or the method is not called, then no response headers will be certified.

For example, to certify only the response body and status code:

```rust
use ic_http_certification::DefaultCelBuilder;

let cel_expr = DefaultCelBuilder::response_certification().build();
```

This can also be done more explicitly:

```rust
use ic_http_certification::{DefaultCelBuilder, DefaultResponseCertification};

let cel_expr = DefaultCelBuilder::response_certification()
    .with_response_certification(DefaultResponseCertification::CertifiedResponseHeaders(&[]))
    .build();
```

The same applies when both when using `DefaultCelBuilder::response_certification` and `DefaultCelBuilder::full_certification`.

```rust
use ic_http_certification::DefaultCelBuilder;

let cel_expr = DefaultCelBuilder::full_certification()
    .with_request_headers(&["Accept", "Accept-Encoding", "If-Match"])
    .with_request_query_parameters(&["foo", "bar", "baz"])
    .build();
```

To skip response certification completely, then certification overall must be skipped completely. It wouldn't be useful to certify a request without certifying a response. So if anything is certified, then it must at least include the response. See the next section for more details on skipping certification entirely.

#### Skipping certification

To skip certification entirely, use `skip_certification`, for example:

```rust
use ic_http_certification::DefaultCelBuilder;

let cel_expr = DefaultCelBuilder::skip_certification();
```

Skipping certification may seem counter-intuitive at first, but it is not always possible to certify a request and response pair. For example, a canister method that will return different data for every user cannot be easily certified.

Typically these requests have been routed through `raw` Internet Computer URLs in the past, but this is dangerous because `raw` URLs allow any responding replica to decide whether or not certification is required. In contrast, by skipping certification using the above method with a non-`raw` URL, a replica will no longer be able to decide whether or not certification is required and instead this decision will be made by the canister itself and the result will go through consensus.

### Directly creating a CEL expression

To define a CEL expression, start with the `CelExpression` enum. This enum provides a set of variants that can be used to define different types of CEL expressions supported by Internet Computer HTTP Gateways. Currently only one variant is supported, known as the "default" certification expression, but more may be added in the future as HTTP certification evolves over time.

When certifying requests, the request body and method are always certified. To additionally certify request headers and query parameters, use the `headers` and `query_paramters` of `DefaultRequestCertification` struct. Both properties take a `str` slice as an argument.

When certifying a response, the response body and status code are always certified. To additionally certify response headers, use the `CertifiedResponseHeaders` variant of the `DefaultResponseCertification` enum. Or to certify all response headers, with some exclusions, use the `ResponseHeaderExclusions` variant of the `DefaultResponseCertification` enum. Both variants take a `str` slice as an argument.

Note that the example CEL expressions provided below are formatted for readability. The actual CEL expressions produced by `CelExpression::to_string` and `create_cel_expr` are minified. The minified CEL expression is preferred because it is more compact, resulting in a smaller payload and a faster evaluation time for the HTTP Gateway that is verifying the certification, but the formatted versions are also accepted.

#### Fully certified request / response pair

To define a fully certified request and response pair, including request headers, query parameters, and response headers:

```rust
use ic_http_certification::cel::{CelExpression, DefaultCertification, DefaultRequestCertification, DefaultResponseCertification};

let cel_expr = CelExpression::DefaultCertification(Some(DefaultCertification {
  request_certification: Some(DefaultRequestCertification {
    headers: &["Accept", "Accept-Encoding", "If-Match"],
    query_parameters: &["foo", "bar", "baz"],
  }),
  response_certification: DefaultResponseCertification::CertifiedResponseHeaders(&[
    "ETag",
    "Cache-Control",
  ]),
}));
```

This will produce the following CEL expression:

```protobuf
default_certification (
  ValidationArgs {
    request_certification: RequestCertification {
      certified_request_headers: ["Accept", "Accept-Encoding", "If-Match"],
      certified_query_parameters: ["foo", "bar", "baz"]
    },
    response_certification: ResponseCertification {
      certified_response_headers: ResponseHeaderList {
        headers: [
          "ETag",
          "Cache-Control"
        ]
      }
    }
  }
)
```

#### Partially certified request

Any number of request headers or query parameters can be provided via the `headers` and `query_parameters` properties of the `DefaultRequestCertification` struct, and both can be an empty array. If the `headers` property is empty, no request headers will be certified. Likewise for the `query_parameters` property, if it is empty then no query parameters will be certified. If both are empty, only the request body and method will be certified.

For example, to certify only the request body and method:

```rust
use ic_http_certification::cel::{CelExpression, DefaultCertification, DefaultRequestCertification, DefaultResponseCertification};

let cel_expr = CelExpression::DefaultCertification(Some(DefaultCertification {
  request_certification: Some(DefaultRequestCertification {
    headers: &[],
    query_parameters: &[],
  }),
  response_certification: DefaultResponseCertification::CertifiedResponseHeaders(&[
    "ETag",
    "Cache-Control",
  ]),
}));
```

This will produce the following CEL expression:

```protobuf
default_certification (
  ValidationArgs {
    request_certification: RequestCertification {
      certified_request_headers: [],
      certified_query_parameters: []
    },
    response_certification: ResponseCertification {
      certified_response_headers: ResponseHeaderList {
        headers: [
          "ETag",
          "Cache-Control"
        ]
      }
    }
  }
)
```

#### Skipping request certification

Request certification can be skipped entirely by setting the `request_certification` property of the `DefaultCertification` struct to `None`. For example:

```rust
use ic_http_certification::cel::{CelExpression, DefaultCertification, DefaultResponseCertification};

let cel_expr = CelExpression::DefaultCertification(Some(DefaultCertification {
  request_certification: None,
  response_certification: DefaultResponseCertification::CertifiedResponseHeaders(&[
    "ETag",
    "Cache-Control",
  ]),
}));
```

This will produce the following CEL expression:

```protobuf
default_certification (
  ValidationArgs {
    no_request_certification: Empty {},
    response_certification: ResponseCertification {
      certified_response_headers: ResponseHeaderList {
        headers: [
          "ETag",
          "Cache-Control"
        ]
      }
    }
  }
)
```

#### Partially certified response

Similiarly to request certification, any number of response headers can be provided via the `CertifiedResponseHeaders` variant of the `DefaultResponseCertification` enum, and it can also be an empty array. If the array is empty, no response headers will be certified. For example:

```rust
use ic_http_certification::cel::{CelExpression, DefaultCertification, DefaultRequestCertification, DefaultResponseCertification};

let cel_expr = CelExpression::DefaultCertification(Some(DefaultCertification {
  request_certification: Some(DefaultRequestCertification {
    headers: &["Accept", "Accept-Encoding", "If-Match"],
    query_parameters: &["foo", "bar", "baz"],
  }),
  response_certification: DefaultResponseCertification::CertifiedResponseHeaders(&[]),
}));
```

This will produce the following CEL expression:

```protobuf
default_certification (
  ValidationArgs {
    request_certification: RequestCertification {
      certified_request_headers: ["Accept", "Accept-Encoding", "If-Match"],
      certified_query_parameters: ["foo", "bar", "baz"]
    },
    response_certification: ResponseCertification {
      certified_response_headers: ResponseHeaderList {
        headers: []
      }
    }
  }
)
```

If the `ResponseHeaderExclusions` variant is used, an empty array will certify _all_ response headers. For example:

```rust
use ic_http_certification::cel::{CelExpression, DefaultCertification, DefaultRequestCertification, DefaultResponseCertification};

let cel_expr = CelExpression::DefaultCertification(Some(DefaultCertification {
  request_certification: Some(DefaultRequestCertification {
    headers: &["Accept", "Accept-Encoding", "If-Match"],
    query_parameters: &["foo", "bar", "baz"],
  }),
  response_certification: DefaultResponseCertification::ResponseHeaderExclusions(&[]),
}));
```

This will produce the following CEL expression:

```protobuf
default_certification (
  ValidationArgs {
    request_certification: RequestCertification {
      certified_request_headers: ["Accept", "Accept-Encoding", "If-Match"],
      certified_query_parameters: ["foo", "bar", "baz"]
    },
    response_certification: ResponseCertification {
      response_header_exclusions: ResponseHeaderList {
        headers: []
      }
    }
  }
)
```

To skip response certification completely, then certification overall must be skipped completely. It wouldn't be useful to certify a request without certifying a response. So if anything is certified, then it must at least include the response. See the next section for more details on skipping certification entirely.

#### Skipping certification

To skip certification entirely:

```rust
use ic_http_certification::cel::{CelExpression, DefaultCertification};

let cel_expr = CelExpression::DefaultCertification(None);
```

This will produce the following CEL expression:

```protobuf
default_certification (
  ValidationArgs {
    no_certification: Empty {}
  }
)
```

Skipping certification may seem counter-intuitive at first, but it is not always possible to certify a request and response pair. For example, a canister method that will return different data for every user cannot be easily certified.

Typically these requests have been routed through `raw` Internet Computer URLs in the past, but this is dangerous because `raw` URLs allow any responding replica to decide whether or not certification is required. In contrast, by skipping certification using the above method with a non-`raw` URL, a replica will no longer be able to decide whether or not certification is required and instead this decision will be made by the canister itself and the result will go through consensus.
