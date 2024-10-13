use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use uuid::Uuid;

#[derive(GraphQLObject)]
pub struct TestObject {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

impl TestObject {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            name,
        }
    }
}

