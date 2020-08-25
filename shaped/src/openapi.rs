pub use okapi::openapi3::{
    Callback, Contact, Info, License, MediaType, Operation, Parameter, ParameterValue, PathItem,
    RefOr, RequestBody, Response, Responses,
};
pub use okapi::Map;

#[derive(Default)]
pub struct OpenApiSettings {
    pub info: Info,
}
