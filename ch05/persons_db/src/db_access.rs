use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertingPerson {
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum DbPrivilege {
    CanRead,
    CanWrite,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub privileges: Vec<DbPrivilege>,
}

pub struct DbConnection {
    persons: Vec<Person>,
    users: Vec<User>,
}

impl DbConnection {
    pub fn new() -> DbConnection {
        DbConnection {
            persons: vec![],
            users: vec![
                User {
                    username: "joe".to_string(),
                    password: "xjoe".to_string(),
                    privileges: vec![DbPrivilege::CanRead],
                },
                User {
                    username: "susan".to_string(),
                    password: "xsusan".to_string(),
                    privileges: vec![DbPrivilege::CanRead, DbPrivilege::CanWrite],
                },
            ],
        }
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<User> {
        Some(self.users.iter().find(|u| u.username == username)?.clone())
    }

    pub fn get_person_by_id(&self, id: u32) -> Option<Person> {
        Some(self.persons.iter().find(|p| p.id == id)?.clone())
    }

    pub fn get_persons_by_partial_name<'a>(
        &'a self,
        subname: &'a str,
    ) -> impl Iterator<Item = &Person> + 'a {
        self.persons
            .iter()
            .filter(move |p| p.name.contains(subname))
    }

    pub fn delete_by_id(&mut self, id: u32) -> bool {
        if let Some((n, _)) = self.persons.iter().enumerate().find(|(_, p)| p.id == id) {
            self.persons.remove(n);
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
        if let Some((n, _)) = self
            .persons
            .iter()
            .enumerate()
            .find(|(_, p)| p.id == person.id)
        {
            self.persons[n] = person;
            true
        } else {
            false
        }
    }
}
