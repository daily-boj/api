use service_macro::service;

#[service("/meta/get_version")]
pub fn get_version() -> &'static str {
    "1.0"
}
