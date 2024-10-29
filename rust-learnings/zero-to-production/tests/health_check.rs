use std::net::TcpListener;

use zerotoprod::run;

#[tokio::test]
async fn test_health_check() {
    let addr = spawn_app();

    let response = reqwest::get(&format!("{}/health_check/", &addr))
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to launch test server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
