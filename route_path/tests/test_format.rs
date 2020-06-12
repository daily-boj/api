#[cfg(test)]
mod test_format {
    use route_path::{RoutePath, RoutePathPart};
    use std::collections::HashMap;

    #[test]
    fn test_constant() -> anyhow::Result<()> {
        assert_eq!(
            RoutePath::from_raw_parts(vec![
                RoutePathPart::Constant("a".to_owned()),
                RoutePathPart::Constant("b".to_owned()),
                RoutePathPart::Constant("c".to_owned()),
                RoutePathPart::Constant("d".to_owned()),
            ])
            .format(HashMap::new())?,
            "a/b/c/d".to_owned()
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
            ])
            .format({
                let mut vars = HashMap::new();
                vars.insert("b".to_owned(), "bee".to_owned());
                vars.insert("c".to_owned(), "sea".to_owned());
                vars.insert("d".to_owned(), "dad".to_owned());
                vars
            })?,
            "a/bee/c/dad".to_owned()
        );
        Ok(())
    }
}
