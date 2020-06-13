#[cfg(test)]
mod test_parse {
    use route_path::{RoutePath, RoutePathPart};

    #[test]
    fn test_constant() -> anyhow::Result<()> {
        assert_eq!(
            "a/b/c/d".parse::<RoutePath>()?,
            RoutePath::from_raw_parts(vec![
                RoutePathPart::Constant("a".to_owned()),
                RoutePathPart::Constant("b".to_owned()),
                RoutePathPart::Constant("c".to_owned()),
                RoutePathPart::Constant("d".to_owned()),
            ]),
        );
        Ok(())
    }
    #[test]
    fn test_trailing_leading_slash() -> anyhow::Result<()> {
        assert_eq!(
            RoutePath::from_raw_parts(vec![
                RoutePathPart::Constant("a".to_owned()),
                RoutePathPart::Constant("b".to_owned()),
                RoutePathPart::Constant("c".to_owned()),
                RoutePathPart::Constant("d".to_owned()),
            ]),
            "/a/b/c/d".parse::<RoutePath>()?,
        );
        assert_eq!(
            RoutePath::from_raw_parts(vec![
                RoutePathPart::Constant("a".to_owned()),
                RoutePathPart::Constant("b".to_owned()),
                RoutePathPart::Constant("c".to_owned()),
                RoutePathPart::Constant("d".to_owned()),
            ]),
            "a/b/c/d/".parse::<RoutePath>()?,
        );
        Ok(())
    }

    #[test]
    fn test_variable() -> anyhow::Result<()> {
        assert_eq!(
            RoutePath::from_raw_parts(vec![
                RoutePathPart::Constant("a".to_owned()),
                RoutePathPart::Variable("b".to_owned()),
                RoutePathPart::Constant("c".to_owned()),
                RoutePathPart::Variable("d".to_owned()),
            ]),
            "a/:b/c/:d".parse::<RoutePath>()?,
        );
        Ok(())
    }
}
