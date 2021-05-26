use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let resp = client
        // the health check is exposed at /health_check
        // the health check is behind a GET method
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // the health check always returns 200
    assert!(resp.status().is_success());
    // the health check's response has no body
    assert_eq!(Some(0), resp.content_length());
}

/// Launch our application in the background.
fn spawn_app() -> String {
    // Bind port 0 trigger an OS scan for an available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to listen to address");
    // Launch the server as a background task
    let _ = tokio::spawn(server);

    // Return the application address
    format!("http://127.0.0.1:{}", port)
}
