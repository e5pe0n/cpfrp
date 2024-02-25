use web_sys::wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InsertingPerson {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DbPriviledge {
    CanRead,
    CanWrite,
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub password: String,
    pub priviledge: Vec<DbPriviledge>,
}

#[derive(PartialEq, Clone)]
pub struct DbConnection {
    users: Vec<User>,
    persons: Vec<Person>,
}

impl DbConnection {
    pub fn new() -> DbConnection {
        DbConnection {
            users: vec![
                User {
                    username: "joe".to_string(),
                    password: "xjoe".to_string(),
                    priviledge: vec![DbPriviledge::CanRead],
                },
                User {
                    username: "susan".to_string(),
                    password: "xsusan".to_string(),
                    priviledge: vec![DbPriviledge::CanRead, DbPriviledge::CanWrite],
                },
            ],
            persons: vec![
                Person {
                    id: 1,
                    name: "alice".to_string(),
                },
                Person {
                    id: 2,
                    name: "bob".to_string(),
                },
                Person {
                    id: 3,
                    name: "aaa".to_string(),
                },
                Person {
                    id: 4,
                    name: "aaaaa".to_string(),
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

    pub fn get_person_by_id(&self, id: u32) -> Option<&Person> {
        if let Some(p) = self.persons.iter().find(|p| p.id == id) {
            Some(p)
        } else {
            None
        }
    }

    pub fn get_persons_by_partial_name(&self, subname: &str) -> Vec<Person> {
        self.persons
            .iter()
            .filter(|p| p.name.contains(subname))
            .cloned()
            .collect()
    }

    pub fn delete_by_id(&mut self, id: u32) -> bool {
        if let Some((i, _)) = self.persons.iter().enumerate().find(|(_, p)| p.id == id) {
            self.persons.remove(i);
            true
        } else {
            false
        }
    }

    pub fn insert_person(&mut self, person: InsertingPerson) -> u32 {
        let new_id = if self.persons.is_empty() {
            1
        } else {
            self.persons[self.persons.len() - 1].id + 1
        };
        self.persons.push(Person {
            id: new_id,
            name: person.name,
        });
        new_id
    }

    pub fn update_person(&mut self, person: Person) -> bool {
        if let Some((i, _)) = self
            .persons
            .iter()
            .enumerate()
            .find(|(_, p)| p.id == person.id)
        {
            self.persons[i] = person;
            true
        } else {
            false
        }
    }
}
