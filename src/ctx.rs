#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: String,
}

// constructor
impl Ctx {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

// property access
impl Ctx {
    pub fn user_id(&self) -> String {
        self.user_id.clone()
    }
}
