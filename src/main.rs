use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration;
use zero2prod::telemetry;
use sqlx::postgres::PgPoolOptions;




#[tokio::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Redirect all 'log' events to the subscriber
   let subscriber = telemetry::get_subscriber(
        "zero2prod".into(),
        "info".into(),
        std::io::stdout,
    );
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    // .expect("Failed to create Postgres connection pool.");
    let sender_email = configuration.email_client.sender()
        .expect("Invalid sender email address");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
    );

    let address = format!("{}:{}", configuration.application.host , configuration.application.port);    

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
