use crate::repo::room::RoomRepo;
use crate::repo::user::UserRepo;

#[derive(Clone)]
pub struct AppState {
    pub rooms: RoomRepo,
    pub users: UserRepo,
}
