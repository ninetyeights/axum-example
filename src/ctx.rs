#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: u64,
}

// Constructor.
impl Ctx {
    pub fn new(user_id: u64) -> Ctx {
        Self { user_id }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
