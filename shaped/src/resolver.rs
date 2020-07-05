use crate::{Provider, Service};
use rayon::prelude::*;
use std::sync::Arc;

pub struct Resolver<ConcreteProvider, ConcreteService, Context, Parameters, Response>
where
    ConcreteProvider: Provider<Item = Parameters>,
    ConcreteService: Service<Context = Context, Param = Parameters, Response = Response>,
    Context: Sync + Send,
    Parameters: Sized + Send,
    Response: Send,
{
    provider: ConcreteProvider,
    pub(crate) service: ConcreteService,
}

impl<ConcreteProvider, ConcreteService, Context, Parameters, Response>
    Resolver<ConcreteProvider, ConcreteService, Context, Parameters, Response>
where
    ConcreteProvider: Provider<Item = Parameters> + Send,
    ConcreteService: Service<Context = Context, Param = Parameters, Response = Response> + Send,
    Context: Sync + Send,
    Parameters: Sized + Send,
    Response: Send,
{
    pub fn new(provider: ConcreteProvider, service: ConcreteService) -> Self {
        Resolver { provider, service }
    }
    pub fn resolve(
        &self,
        context: Arc<Context>,
    ) -> (
        Vec<(String, Response)>,
        Vec<route_path::RoutePathFormatError>,
    ) {
        let (success, failure): (Vec<_>, Vec<_>) = self
            .provider
            .provide()
            .into_par_iter()
            .map(
                |params: Parameters| -> Result<(String, Response), route_path::RoutePathFormatError> {
                    Ok((
                        self.service
                            .path()
                            .format(self.service.make_variables(&params))?,
                        self.service.execute(context.clone(), params),
                    ))
                },
            )
            .partition(Result::is_ok);
        let success: Vec<_> = success.into_iter().filter_map(Result::ok).collect();
        let failure: Vec<_> = failure.into_iter().filter_map(Result::err).collect();

        (success, failure)
    }
}
