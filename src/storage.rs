use std::collections::HashMap;
use std::io::{self};
use std::result;

use chrono::{DateTime, Local};

use crate::bd_manager::{SerdePerson, SerdePersons};
#[derive(Clone)]
pub struct Person {
    name: String,
    surname: String,
    middle_name: String,
    date_of_birth: DateTime<Local>,
    gender: bool,
}
#[derive(Clone)]
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
    fn add(&mut self, id: i32, person: Person) {
        let _ = self.persons.insert(id, person);
    }

    fn get(&self, id: Option<Vec<i32>>) -> Option<Vec<(i32, Person)>> {
        match id {
            Some(ids) => {
                let mut result = ids.iter().fold(Vec::new(), |mut res, id| {
                    if let Some(person) = self.persons.get(id) {
                        res.push((id.to_owned(), person.to_owned()));
                    }
                    res
                });
                if result.is_empty() {
                    None
                } else {
                    Some(result)
                }
            }
            None => {
                let result: Vec<(i32, Person)> = self.to_owned().into();
                if result.is_empty() {
                    None
                } else {
                    Some(result)
                }
            }
        }
    }

    fn delete(&mut self, predicate: &dyn Fn(&i32, &mut Person) -> bool) {
        self.persons.retain(predicate);
    }

    fn find(&self, predicant: &dyn Fn(&(&i32, &Person)) -> bool) -> Option<Vec<(i32, Person)>> {
        let result = self
            .persons
            .iter()
            .filter(predicant)
            .map(|(id, person)| (id.to_owned(), person.to_owned()))
            .collect::<Vec<(i32, Person)>>();
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

impl Into<Vec<(i32, Person)>> for PersonStorage {
    fn into(self) -> Vec<(i32, Person)> {
        let result = self
            .persons
            .iter()
            .map(|(id, persons)| (id.to_owned(), persons.to_owned()))
            .collect::<Vec<(i32, Person)>>();
        result
    }
}
