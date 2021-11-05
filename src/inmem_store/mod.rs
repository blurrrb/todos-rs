pub mod todos;

use uuid::Uuid;

use crate::todos::Todo;
use std::collections::HashMap;

pub struct InMemStore {
    store: HashMap<Uuid, Todo>,
}

impl InMemStore {
    pub fn new() -> Self {
        InMemStore {
            store: HashMap::new(),
        }
    }
}
