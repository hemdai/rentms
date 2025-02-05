#[cfg(test)]
mod integration_tests {
    use super::{app_state::AppState, register};
    use actix_web::{App, HttpServer};
    use reqwest::Client;
    use serde_json::json;
    use std::net::TcpListener; // Replace with actual path

    #[actix_web::test]
    async fn test_register_endpoint() {
        // Set up the server
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let db = setup_mock_database(); // Assume a function that sets up a mock DB

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(AppState { db: db.clone() }))
                .service(register)
        })
        .listen(listener)
        .expect("Failed to start server")
        .run();

        let _ = tokio::spawn(server);

        // Make a request to the server
        let client = Client::new();
        let res = client
            .post(&format!("http://127.0.0.1:{}/register", port))
            .json(&json!({
                "name": "Test User",
                "email": "test@example.com",
                "password": "password123"
            }))
            .send()
            .await
            .expect("Failed to send request");

        // Assert the response
        assert!(res.status().is_success());
        let body: serde_json::Value = res.json().await.expect("Failed to read response body");
        assert_eq!(body["message"], "User Test User created successfully");
    }

    // Mock database setup (You will need to implement this based on your DB layer)
    fn setup_mock_database() -> sea_orm::DatabaseConnection {
        // Mock database setup logic here
        unimplemented!()
    }
}
