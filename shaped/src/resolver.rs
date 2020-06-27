use crate::{Provider, Service};

pub struct Resolver<ConcreteProvider, ConcreteService, Parameters, Response>
where
    ConcreteProvider: Provider<Item = Parameters>,
    ConcreteService: Service<Param = Parameters, Response = Response>,
{
    provider: ConcreteProvider,
    service: ConcreteService,
}

impl<ConcreteProvider, ConcreteService, Parameters, Response>
    Resolver<ConcreteProvider, ConcreteService, Parameters, Response>
where
    ConcreteProvider: Provider<Item = Parameters>,
    ConcreteService: Service<Param = Parameters, Response = Response>,
{
    pub fn resolve(&self) -> Result<Vec<(String, Response)>, route_path::RoutePathFormatError> {
        let mut responses = vec![];
        for params in self.provider.provide() {
            responses.push((
                self.service
                    .path()
                    .format(self.service.make_variables(&params))?,
                self.service.execute(params),
            ));
        }
        Ok(responses)
    }
}
