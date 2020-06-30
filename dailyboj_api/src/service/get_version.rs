use service_macro::service;

#[service("/get_version")]
pub fn get_version() -> &'static str {
    "1.0"
}
