use fbsearch::{configuration::get_configuration, startup::run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.database.host, configuration.application_port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    println!("{}", port);
    run(listener)?.await
}
