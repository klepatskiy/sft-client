use std::sync::Arc;
use druid::{Data, Lens};
use im::Vector;

#[derive(Clone, Data, Lens)]
pub struct Friend {
    pub name: String,
    pub online: bool,
}

#[derive(Clone, Data, Lens)]
pub struct FriendState {
    pub friends: Arc<Vector<Friend>>,
}

impl FriendState {
    // pub fn add_friend(&mut self, name: String, online: bool) {
    // }
    //
    // pub fn remove_friend(&mut self, name: &str) {
    // }
    //
    // pub fn update_status(&mut self, name: &str, online: bool) {
    // }
}
