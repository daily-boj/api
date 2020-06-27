use route_path::RoutePath;
use std::collections::HashMap;

pub trait Service {
    type Param;
    type Response;

    fn path(&self) -> RoutePath;

    fn make_variables(&self, params: &Self::Param) -> HashMap<String, String>;

    fn execute(&self, params: Self::Param) -> Self::Response;
}
