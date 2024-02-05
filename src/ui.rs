use crate::storage::PersonStorage;

pub trait UI {
    fn add_info(data: &PersonStorage) {
        todo!()
    }
    fn show_info(data: &PersonStorage) {
        todo!()
    }
    fn show_all_info(data: &PersonStorage) {
        todo!()
    }
    fn delet_param(data: &PersonStorage, param: DeleteParam) {
        todo!()
    }
    fn find_param(data: &PersonStorage, param: FindParam) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum DeleteParam {
    Name,
    Surname,
    MiddleName,
    DateOfBirth,
    Gender,
}

#[derive(Debug, Clone)]
pub enum FindParam {
    Name,
    Surname,
    MiddleName,
    DateOfBirth,
    Gender,
}

impl Into<Person> for DeleteParam {
    fn into() -> Person {
        match &self {
            DeleteParam::Name => todo!(),
            DeleteParam::Surname => todo!(),
            DeleteParam::MiddleName => todo!(),
            DeleteParam::DateOfBirth => todo!(),
            DeleteParam::Gender => todo!(),
        }
    }
}
