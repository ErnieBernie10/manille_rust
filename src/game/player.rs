use uuid::Uuid;

pub struct Player {
    pub id: Uuid
}

impl Player {

    pub fn new() -> Player {
        Player {
            id: Uuid::new_v4()
        }
    }
}
