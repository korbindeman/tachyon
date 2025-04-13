use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Upload {
    pub id: String,
    pub name: String,
    pub path: String,
}

impl Upload {
    pub fn new(id: String, name: String, path: String) -> Self {
        Self { id, name, path }
    }
}
