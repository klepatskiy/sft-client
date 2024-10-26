use druid::{Data, Lens};
use crate::client::grpc_login_request;

#[derive(Clone, Data, Lens)]
pub struct AuthState {
    pub email: String,
    pub login: String,
    pub display_message: String,
    pub is_logged_in: bool,
}

impl AuthState {
    pub fn reset(&mut self) {
        self.email = "".into();
        self.login = "".into();
        self.display_message = "Please enter your credentials".into();
        self.is_logged_in = false;
    }

    pub async fn login_serve(&mut self) {
        let result = grpc_login_request(self.email.clone(), self.login.clone());

        match result.await {
            Ok(_) => {
                self.is_logged_in = true;
            }
            Err(_) => {
                self.is_logged_in = false;
            }
        }
    }
}
