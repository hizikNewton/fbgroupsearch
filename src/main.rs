use fbsearch::{configuration::get_configuration, startup::run};
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!(
        "{}:{}",
        configuration.database.host, configuration.application_port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener,connection)?.await
}
