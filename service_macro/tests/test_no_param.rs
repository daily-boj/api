#[cfg(test)]
mod test_no_param {
    use service_macro::service;
    use shaped::route_path::{RoutePath, RoutePathPart};
    use shaped::Service;
    use std::sync::Arc;

    #[service("/example")]
    fn example() -> &'static str {
        "Hello, world!"
    }

    #[test]
    fn test_path() {
        assert_eq!(
            RoutePath::from_raw_parts(vec![RoutePathPart::Constant("example".to_owned())]),
            example.path(),
        );
    }

    #[test]
    fn test_response() {
        assert_eq!("Hello, world!", example.execute(Arc::new(()), ()));
    }
}
