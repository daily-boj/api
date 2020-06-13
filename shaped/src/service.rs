use route_path::RoutePath;

pub trait Service {
    type Param;
    type Response;

    fn path(&self) -> RoutePath;

    fn execute(&self, params: Self::Param) -> Self::Response;
}
