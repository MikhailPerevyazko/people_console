use std::collections::HashMap;

use chrono::{DateTime, Local};

use crate::bd_manager::{SerdePerson, SerdePersons};

pub struct Person {
    name: String,
    surname: String,
    middle_name: String,
    date_of_birth: DateTime<Local>,
    gender: bool,
}
pub struct PersonStorage {
    persons: HashMap<i32, Person>,
}

impl Person {
    pub fn new(
        name: String,
        surname: String,
        middle_name: String,
        date_of_birth: DateTime<Local>,
        gender: bool,
    ) -> Self {
        Self {
            name,
            surname,
            middle_name,
            date_of_birth,
            gender,
        }
    }
}

impl PersonStorage {
    pub fn new(persons: HashMap<i32, Person>) -> Self {
        Self { persons }
    }
}

impl Person {
    fn into_serde_person(&self, id: &i32) -> SerdePerson {
        SerdePerson::new(
            id.to_owned(),
            self.name.to_owned(),
            self.surname.to_owned(),
            self.middle_name.to_owned(),
            self.date_of_birth.to_string(),
            self.gender.to_owned(),
        )
    }
}
impl Into<SerdePersons> for PersonStorage {
    fn into(self) -> SerdePersons {
        let result = self
            .persons
            .iter()
            .fold(Vec::new(), |mut res, (id, person)| {
                res.push(person.into_serde_person(id));
                res
            });
        SerdePersons::new(result)
    }
}

impl PersonStorage {
    fn add(&mut self) {
        todo!()
    }
    fn get(&self) -> &[Person] {
        todo!()
    }
    fn delete(&mut self) {
        todo!()
    }
    fn find(&self) -> &[Person] {
        todo!()
    }
}
