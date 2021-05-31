use sqlx::postgres::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subcriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subcriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let connection_pool = PgPool::connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(address).expect("Failed to bind address");

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );

    run(listener, connection_pool, email_client)?.await
}
