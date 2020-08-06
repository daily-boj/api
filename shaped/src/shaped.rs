use crate::{Provider, Resolver, Service};
use rayon::prelude::*;
use serde::Serialize;
use serde_json;
use std::fs;
use std::path::Path;
use std::sync::Arc;
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

pub struct Shaped {
    resolvers:
        Vec<Box<dyn Fn() -> (String, Vec<(String, String)>, Vec<ShapedError>) + Sync + Send>>,
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
        ConcreteService: Service<Context = (), Param = Parameters, Response = Response> + 'static,
        Parameters: Sized + Send + 'static,
        Response: Serialize + Sync + Send + 'static,
    {
        self.with_resolver(Arc::new(()), Resolver::new(provider, service))
    }
    pub fn with_context<ConcreteProvider, ConcreteService, Context, Parameters, Response>(
        &mut self,
        context: Arc<Context>,
        provider: ConcreteProvider,
        service: ConcreteService,
    ) -> &mut Self
    where
        ConcreteProvider: Provider<Item = Parameters> + 'static,
        ConcreteService:
            Service<Context = Context, Param = Parameters, Response = Response> + 'static,

        Context: Sync + Send + 'static,
        Parameters: Sized + Send + 'static,
        Response: Serialize + Sync + Send + 'static,
    {
        self.with_resolver(context, Resolver::new(provider, service))
    }
    pub fn with_resolver<ConcreteProvider, ConcreteService, Context, Parameters, Response>(
        &mut self,
        context: Arc<Context>,
        resolver: Resolver<ConcreteProvider, ConcreteService, Context, Parameters, Response>,
    ) -> &mut Self
    where
        ConcreteProvider: Provider<Item = Parameters> + 'static,
        ConcreteService:
            Service<Context = Context, Param = Parameters, Response = Response> + 'static,
        Context: Sync + Send + 'static,
        Parameters: Sized + Send + 'static,
        Response: Serialize + Send + 'static,
    {
        self.resolvers.push(Box::new(move || {
            let (success, route_failure) = resolver.resolve(context.clone());
            let (success, json_failure): (Vec<_>, Vec<_>) = success
                .into_iter()
                .map(|(path, resp)| serde_json::to_string(&resp).map(|v| (path.clone(), v)))
                .partition(Result::is_ok);
            let success: Vec<_> = success.into_iter().filter_map(Result::ok).collect();

            let failure: Vec<_> = route_failure
                .into_iter()
                .map(ShapedError::from)
                .chain(
                    json_failure
                        .into_iter()
                        .filter_map(Result::err)
                        .map(ShapedError::from),
                )
                .collect();

            (resolver.service.path().to_string(), success, failure)
        }));
        self
    }

    pub fn generate_on<P: AsRef<Path>>(&self, base: P) -> Vec<RouteGeneration> {
        let base = base.as_ref();

        self.resolvers
            .par_iter()
            .map(|resolve| {
                let (path, success, resolve_errors) = resolve();
                let (success, io_errors): (Vec<_>, Vec<_>) = success
                    .par_iter()
                    .map(|(path, response)| -> Result<(), ShapedError> {
                        let path_json = base.join(format!("{}.json", path));
                        let path_normal = base.join(format!("{}", path));
                        if let Some(parent) = path_json.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        fs::write(path_json, response)?;
                        fs::write(path_normal, response)?;
                        Ok(())
                    })
                    .partition(Result::is_ok);
                let errors: Vec<_> = resolve_errors
                    .into_iter()
                    .chain(io_errors.into_iter().filter_map(Result::err))
                    .collect();
                RouteGeneration {
                    name: path,
                    success: success.len(),
                    errors,
                }
            })
            .collect()
    }
}

pub struct RouteGeneration {
    pub name: String,
    pub success: usize,
    pub errors: Vec<ShapedError>,
}
