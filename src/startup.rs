use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use web::Data;

use crate::routes;
use crate::email_client::EmailClient;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);

    let email_client = Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
        })
    .listen(listener)?
    .run();

    Ok(server)
}
