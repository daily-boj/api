use okapi::openapi3::PathItem;
use route_path::RoutePath;
use std::collections::HashMap;
use std::sync::Arc;

pub trait Service: Sync + Send {
    type Context;
    type Param: Sized + Send;
    type Response: Send;

    fn path(&self) -> RoutePath;

    fn openapi_detail(&self) -> PathItem;

    fn make_variables(&self, params: &Self::Param) -> HashMap<String, String>;

    fn execute(&self, context: Arc<Self::Context>, params: Self::Param) -> Self::Response;
}
