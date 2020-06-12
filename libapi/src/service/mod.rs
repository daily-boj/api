use route_path::RoutePath;

trait Service {
    type Param;

    fn path(&self) -> RoutePath;
}
