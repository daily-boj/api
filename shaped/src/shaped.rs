use crate::{OpenApiSettings, Provider, Resolver, Service};
use okapi::openapi3::{Components, OpenApi, PathItem, Tag};
use rayon::prelude::*;
use schemars::{schema::RootSchema, schema_for, JsonSchema};
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
    openapi_settings: Option<OpenApiSettings>,
    resolvers:
        Vec<Box<dyn Fn() -> (String, Vec<(String, String)>, Vec<ShapedError>) + Sync + Send>>,
    schemas: Vec<(String, RootSchema)>,
    paths: Vec<(String, PathItem)>,
}
impl Shaped {
    pub fn no_openapi() -> Self {
        Shaped {
            resolvers: Vec::new(),
            openapi_settings: None,
            schemas: Vec::new(),
            paths: Vec::new(),
        }
    }
    pub fn openapi(openapi_settings: OpenApiSettings) -> Self {
        Shaped {
            resolvers: Vec::new(),
            openapi_settings: Some(openapi_settings),
            schemas: Vec::new(),
            paths: Vec::new(),
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
        Response: JsonSchema + Serialize + Sync + Send + 'static,
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
        Response: JsonSchema + Serialize + Sync + Send + 'static,
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
        Response: JsonSchema + Serialize + Send + 'static,
    {
        if self.openapi_settings.is_some() {
            self.schemas
                .push((Response::schema_name(), schema_for!(Response)));
            self.paths.push((
                format!("/{:#}.json", resolver.service.path()),
                resolver.service.openapi_detail(),
            ))
        }
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

        if let Some(settings) = &self.openapi_settings {
            let openapi = OpenApi {
                openapi: "3.0.0".to_owned(),
                info: settings.info.clone(),
                servers: vec![settings.server.clone()],
                paths: self.paths.iter().cloned().collect(),
                components: Some(Components {
                    schemas: self
                        .schemas
                        .iter()
                        .flat_map(|(name, schema)| {
                            schema
                                .definitions
                                .iter()
                                .map(|(name, schema)| (name.clone(), schema.clone().into_object()))
                                .chain(std::iter::once((name.clone(), schema.schema.clone())))
                        })
                        .collect(),
                    ..Default::default()
                }),
                tags: Vec::<Tag>::default(),
                ..Default::default()
            };

            if fs::create_dir_all(base).is_err() {
                eprintln!("Failed to create folder: {:?}", base.as_os_str());
            }
            if fs::write(
                base.join("swagger.json"),
                serde_json::to_string_pretty(&openapi).expect("will not fail"),
            )
            .is_err()
            {
                eprintln!(
                    "Failed to create file: {:?}",
                    base.join("swagger.json").as_os_str()
                )
            }
        }

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
