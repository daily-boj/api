mod log;

use actix_web::{
    dev, http,
    middleware::{self, errhandlers},
    App, HttpServer,
};
use std::path::PathBuf;

fn index_html<B>(
    res: dev::ServiceResponse<B>,
) -> actix_web::Result<errhandlers::ErrorHandlerResponse<B>> {
    let response = actix_files::NamedFile::open(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("public")
            .join("index.html"),
    )?
    .into_response(res.request())?;

    Ok(errhandlers::ErrorHandlerResponse::Response(
        dev::ServiceResponse::new(res.request().clone(), response.into_body()),
    ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    log::setup_logger().expect("Logger setup failed");
    HttpServer::new(|| {
        App::new()
            .wrap(
                errhandlers::ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, index_html),
            )
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new(
                "/",
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("public"),
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
