#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(PartialEq, Clone)]
pub struct DbConnection {
    users: Vec<User>,
}

impl DbConnection {
    pub fn new() -> DbConnection {
        DbConnection {
            users: vec![
                User {
                    username: "joe".to_string(),
                    password: "xjoe".to_string(),
                },
                User {
                    username: "susan".to_string(),
                    password: "xsusan".to_string(),
                },
            ],
        }
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        if let Some(u) = self.users.iter().find(|u| u.username == username) {
            Some(u)
        } else {
            None
        }
    }
}
