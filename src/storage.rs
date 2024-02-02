use std::collections::HashMap;

use crate::bd_manager::{YamlPerson, YamlPersons};

pub struct Person {}
pub struct PersonStorage {
    persons: HashMap<String, Person>,
}

impl Into<YamlPerson> for Person {
    fn into(self) -> YamlPerson {
        todo!()
    }
}
impl Into<YamlPersons> for PersonStorage {
    fn into(self) -> YamlPersons {
        todo!()
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
