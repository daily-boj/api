# service_macro

Service declaring easily done.

## Usage

What you write is:

```rs
#[service("/version")]
pub fn get_api_version() -> &'static str {
  "1.0"
}
```

What you get is:

```rs
fn __get_api_version_raw() -> &'static str {
  "1.0"
}

pub struct get_api_version;

impl libapi::service::Service for get_api_version {
  type Param = ();
  type Response = &'static str;

  fn get_route(&self) -> RoutePath {
    "/version".parse()
  }

  fn execute(&self, param: Self::Param) -> Self::Response {
    let () = param;
    __get_api_version_raw();
  }
}
```
