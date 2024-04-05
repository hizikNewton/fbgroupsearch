use std::net::TcpListener;

#[tokio::test]
async fn fb_check_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/fb_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    print!("{}",response.status().is_success());
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let server = fbsearch::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    address
}