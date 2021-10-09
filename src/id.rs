use uuid::Uuid;

pub type Id = Uuid;

pub fn random_id() -> Id {
    Uuid::new_v4()
}