use crate::yaml_bd::YamlBD;
use std::{io, path::PathBuf};

mod bd_manager;
mod cmd_manager;
mod storage;
mod ui;
mod yaml_bd;

fn main() {
    //Подключение к файлу с инфомрацией.
    let path: PathBuf = ["config.yaml"].iter().collect();
    let connecting_to_file = YamlBD { file_path: path };
    let get_data_file = bd_manager::BDOperation::load(&connecting_to_file);

    //Вывести всю информацию?
    let mut answer: String = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Can't read answer");

    if answer == "Yes" {
        cmd_manager::TUI::show_all_info();
    } else if answer == "No" {
        todo!()
    } else {
        println!("Wrong answer.")
    }
}
