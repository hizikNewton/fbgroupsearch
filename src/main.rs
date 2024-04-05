use fbsearch::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use std::net::TcpListener;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!(
        "{}:{}",
        configuration.database.host, configuration.application_port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener)?.await
}
