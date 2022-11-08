struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn id(&self) -> i32 {
        self.id
    }
}

pub struct QueryRoot;

impl QueryRoot {
    fn todos() -> Vec<Todo> {
        vec![Todo {
            id: 1,
            title: "new pj in Rust".to_string(),
            completed: false,
        }]
    }
}
