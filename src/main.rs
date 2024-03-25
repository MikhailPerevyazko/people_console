mod bd_manager;
mod cmd_manager;
mod storage;
mod ui;
mod yaml_bd;

use storage::PersonStorage;
use ui::UI;

use crate::bd_manager::{BDOperation, SerdePersons};
use crate::cmd_manager::TUI;
use crate::yaml_bd::YamlBD;
use std::{io, path::PathBuf};

fn main() {
    //* Подключение к файлу с инфомрацией.
    let path: PathBuf = PathBuf::from("/home/Mikhail/projects/People/person_console/config.yaml");
    let connecting_to_file = YamlBD { file_path: path };
    let get_data_file = connecting_to_file.load();
    let mut file_data: PersonStorage = match get_data_file {
        Ok(data) => data.into(),
        Err(err) => {
            println!("{err}");
            SerdePersons::default().into()
        }
    };

    let tui = TUI {};

    println!("Что вы хотите сделать?\n1 - Вывести всю информацию\n2 - Найти персону по ID,\n3 - Добавить новую персону\nВаш ответ:");
    let mut answer_line: String = String::new();
    io::stdin()
        .read_line(&mut answer_line)
        .expect("Can't read answer");
    let answer: String = answer_line.trim_end().to_string();

    if answer == "1".to_string() {
        tui.show_all_info(&file_data);
    } else if answer == "2".to_string() {
        tui.show_info(&file_data);
    } else if answer == "3".to_string() {
        let new_info = tui.add_info(&mut file_data);
        let data: SerdePersons = file_data.into();
        connecting_to_file.save(&data).unwrap();
        println!("{:#?}", new_info);
    } else {
        println!("Вы выбрали неправильную команду!")
    }
}
