use reqwest::StatusCode;

use crate::helpers::spawn_app;

#[tokio::test]
async fn user_logins_with_correct_creds() {
    // Arrange
    let app = spawn_app().await;
    let login_body = serde_json::json!({
        "email": app.test_user.email,
        "password": app.test_user.password
    });

    // Act
    let response = app.test_user.login(&app).await;
    // Assert
    assert!(&response.headers().get("set-cookie").is_some());
    println!("{:?}", response.status());
    assert!(response.status().is_success());

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse");

    assert_eq!(response_body["email"], login_body["email"]);
}

#[tokio::test]
async fn user_cannot_login_with_incorrect_creds() {
    // Arrange
    let app = spawn_app().await;
    let login_body = serde_json::json!({
        "email": "user_not_exist@example.com",
        "password": "P@$$w0rd"
    });

    // Act
    let response = app.post_login(&login_body).await;

    // Assert
    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn user_can_logout() {
    // Arrange
    let app = spawn_app().await;

    // Act
    // Login
    let response = app.test_user.login(&app).await;

    assert!(response.status().is_success());

    let current_user = app.post_current_user().await;
    let user_body: serde_json::Value = current_user.json().await.expect("Failed to parse");

    assert!(user_body["user"]["id"].as_str().is_some());

    // Act
    // Logout
    let response = app.post_logout().await;
    assert!(response.status().is_success());

    let current_user = app.post_current_user().await;

    let user_body: serde_json::Value = current_user.json().await.expect("Failed to parse");

    assert!(user_body["user"]["id"].as_str().is_none());
}
