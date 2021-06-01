use super::helpers::spawn_app;

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        // the health check is exposed at /health_check
        // the health check is behind a GET method
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // the health check always returns 200
    assert!(resp.status().is_success());
    // the health check's response has no body
    assert_eq!(Some(0), resp.content_length());
}
