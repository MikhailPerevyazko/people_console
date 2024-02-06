use crate::storage::PersonStorage;

//Определяем методы для пользовательского интерфейса.
pub trait UI {
    fn add_info(&self, data: &PersonStorage) -> String;
    fn show_info(&self, data: &PersonStorage) -> String;
    fn show_all_info(&self, data: &PersonStorage) -> String;
    fn delet_param(&self, data: &PersonStorage, param: DeleteParam) -> String;
    fn find_param(&self, data: &PersonStorage, param: FindParam) -> String;
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
