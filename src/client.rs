use auth::auth_service_client::AuthServiceClient;
use auth::{LoginRequest, LoginReply};

pub mod auth {
    tonic::include_proto!("auth");
}

pub async fn grpc_login_request(
    email: String,
    password: String,
) -> Result<LoginReply, Box<dyn std::error::Error>> {
    let mut client = AuthServiceClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(LoginRequest { email, password });
    let response = client.login(request).await?.into_inner();

    Ok(response)
}
