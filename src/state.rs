use druid::{Data, Lens};

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
}
