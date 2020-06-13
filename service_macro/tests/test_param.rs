#[cfg(test)]
mod test_macro {
    use service_macro::service;
    use shaped::route_path::{RoutePath, RoutePathPart};
    use shaped::Service;

    #[service("/example/:name")]
    fn example(name: String) -> String {
        format!("Hello, {}!", name)
    }

    #[test]
    fn test_path() {
        assert_eq!(
            RoutePath::from_raw_parts(vec![
                RoutePathPart::Constant("example".to_owned()),
                RoutePathPart::Variable("name".to_owned())
            ]),
            example.path(),
        );
    }

    #[test]
    fn test_response() {
        assert_eq!("Hello, world!", example.execute(("world".to_owned(),)));
    }
}
