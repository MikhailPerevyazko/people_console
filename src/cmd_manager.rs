use crate::storage::PersonStorage;
use crate::ui::UI;

use std::io;

pub struct TUI {}

impl UI for TUI {
    fn add_info(&self, data: &crate::storage::PersonStorage) -> String {
        println!("Вы хотите добавить нового пользователя? да/нет: ");
        let mut answer: String = String::new();
        io::stdin().read_line(&mut answer).expect("Ошибка ввода");
        answer = answer.trim().to_lowercase();
        if answer != "да" {
            return "До свидания!".to_string();
        } else {
            add();
            return "Запись прошла успешно".to_string();
        }
    }
    fn show_info(&self, data: &crate::storage::PersonStorage) -> String {
        todo!()
    }
    fn show_all_info(&self, data: &crate::storage::PersonStorage) -> String {
        todo!()
    }
    fn delet_param(
        &self,
        data: &crate::storage::PersonStorage,
        param: crate::ui::DeleteParam,
    ) -> String {
        todo!()
    }
    fn find_param(
        &self,
        data: &crate::storage::PersonStorage,
        param: crate::ui::FindParam,
    ) -> String {
        todo!()
    }
}
