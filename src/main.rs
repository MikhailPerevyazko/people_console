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
    let path: PathBuf = ["config.yaml"].iter().collect();
    let connecting_to_file = YamlBD { file_path: path };
    let get_data_file = connecting_to_file.load();
    let file_data: PersonStorage = match get_data_file {
        Ok(data) => data.into(),
        Err(_err) => SerdePersons::default().into(),
    };

    println!("{:#?}", file_data);

    // //*Вывести всю информацию?
    // let mut answer: String = String::new();
    // io::stdin()
    //     .read_line(&mut answer)
    //     .expect("Can't read answer");

    // if answer == "Yes" {
    //     let tui_fn = TUI {};
    //     tui_fn::show_all_info();
    // } else if answer == "No" {
    //     println!("Good luck!")
    // } else {
    //     println!("Wrong answer.")
    // }
}
