use crate::repo::room::RoomRepo;

#[derive(Clone)]
pub struct AppState {
    pub rooms: RoomRepo,
}
