#[cfg(test)]
mod test_format {
    use route_path::RoutePath;
    use std::collections::HashMap;

    #[test]
    fn test_constant() -> anyhow::Result<()> {
        assert_eq!(
            "a/b/c/d".parse::<RoutePath>()?.format(HashMap::new())?,
            "a/b/c/d".to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_variable() -> anyhow::Result<()> {
        assert_eq!(
            "a/:b/c/:d".parse::<RoutePath>()?.format({
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
