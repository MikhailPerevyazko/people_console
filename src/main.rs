mod bd_manager;
mod cmd_manager;
mod storage;
mod ui;
mod yaml_bd;

use storage::PersonStorage;
use ui::UI;

use crate::bd_manager::{BDOperation, SerdePersons};
use crate::cmd_manager::TUI;
use crate::storage::Person;
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
    println!("{:?}", file_data);
    let tui = TUI {};
    //*Вывести всю инофрмацию?
    println!("Вывести всю информацию? Yes/No? ");
    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Can't read answer");
    let answer: String = input_line.trim_end().to_string();

    if answer == "Yes".to_string() || answer == "yes".to_string() {
        println!("{:?}", file_data);
        tui.show_all_info(&file_data);
    } else if answer == "No".to_string() || answer == "no".to_string() {
        println!("Goodbye!")
    } else {
        println!("Wrong answer!")
    }

    //*Вывести информацию по id?
    println!("Найти по id? Yes/No:");
    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Can't read answer");
    let answer: String = input_line.trim_end().to_string();

    if answer == "Yes".to_string() || answer == "yes".to_string() {
        tui.show_info(&file_data);
    } else if answer == "No".to_string() || answer == "no".to_string() {
        println!("Goodbye!")
    } else {
        println!("Wrong answer!")
    }

    //*Добавить информацию о персоне.
    println!("Добавить информацию? Yes/No:");
    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Can't read answer");
    let answer: String = input_line.trim_end().to_string();

    if answer == "Yes".to_string() || answer == "yes".to_string() {
        let new_info = tui.add_info(&mut file_data);
        println!("{:?}", file_data);
        let data: SerdePersons = file_data.into();
        connecting_to_file.save(&data).unwrap();
    } else if answer == "No".to_string() || answer == "no".to_string() {
        println!("Goodbye!")
    } else {
        println!("Wrong answer!")
    }
}
