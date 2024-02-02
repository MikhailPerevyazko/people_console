use std::{error::Error, path::PathBuf};

use crate::storage::{Person, PersonStorage};

pub struct YamlPerson {}
pub struct YamlPersons {
    persons: Vec<YamlPerson>,
}

impl Into<Person> for YamlPerson {
    fn into(self) -> Person {
        todo!()
    }
}
impl Into<PersonStorage> for YamlPersons {
    fn into(self) -> PersonStorage {
        todo!()
    }
}

impl YamlPersons {
    fn load(file_name: &PathBuf) -> Result<Self, Box<dyn Error>> {
        todo!()
    }
    fn save(&self, file_name: &PathBuf) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
