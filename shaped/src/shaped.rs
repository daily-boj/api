use crate::{Provider, Resolver, Service};
use serde::Serialize;
use serde_json;
use std::fs;
use std::path::Path;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShapedError {
    #[error("Unexpected serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Invalid route path format: {0}")]
    RoutePath(#[from] route_path::RoutePathFormatError),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = StdResult<T, ShapedError>;

pub struct Shaped {
    resolvers: Vec<Box<dyn Fn() -> Result<Vec<(String, String)>>>>,
}
impl Shaped {
    pub fn new() -> Self {
        Shaped {
            resolvers: Vec::new(),
        }
    }
    pub fn with<ConcreteProvider, ConcreteService, Parameters, Response>(
        &mut self,
        provider: ConcreteProvider,
        service: ConcreteService,
    ) -> &mut Self
    where
        ConcreteProvider: Provider<Item = Parameters> + 'static,
        ConcreteService: Service<Param = Parameters, Response = Response> + 'static,
        Parameters: 'static,
        Response: Serialize + 'static,
    {
        self.with_resolver(Resolver::new(provider, service))
    }
    pub fn with_resolver<ConcreteProvider, ConcreteService, Parameters, Response>(
        &mut self,
        resolver: Resolver<ConcreteProvider, ConcreteService, Parameters, Response>,
    ) -> &mut Self
    where
        ConcreteProvider: Provider<Item = Parameters> + 'static,
        ConcreteService: Service<Param = Parameters, Response = Response> + 'static,
        Parameters: 'static,
        Response: Serialize + 'static,
    {
        self.resolvers.push(Box::new(move || {
            let resolved = resolver.resolve()?;
            let resolved: StdResult<Vec<_>, _> = resolved
                .iter()
                .map(|(path, resp)| serde_json::to_string(resp).map(|v| (path.clone(), v)))
                .collect();

            Ok(resolved?)
        }));
        self
    }

    pub fn generate_on<P: AsRef<Path>>(&self, base: P) -> Result<()> {
        let base = base.as_ref();

        for resolve in &self.resolvers {
            for (path, response) in resolve()? {
                let path = base.join(format!("{}.json", path));
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(path, response)?;
            }
        }

        Ok(())
    }
}
