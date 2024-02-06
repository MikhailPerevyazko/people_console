use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};

use crate::storage::{self, Person, PersonStorage};
use crate::ui::UI;

use std::io::stdin;
use std::{io, vec};

pub struct TUI {}

impl UI for TUI {
    fn add_info(&self, data: &crate::storage::PersonStorage) -> String {
        println!("Ввести новый id?");

        let mut new_id: Vec<i32> = vec![];

        let mut id: String = String::new();
        io::stdin().read_line(&mut id).expect("Can't read new id.");
        for i in id.lines() {
            new_id.push(i.parse::<i32>().unwrap_or_default());
        }

        let mut new_person: Vec<_> = Vec::new();

        println!("\n");
        println!("Введите новый name: ");
        let mut new_name = String::new();
        io::stdin()
            .read_line(&mut new_name)
            .expect("Can't read new name.");
        for line in new_name.lines() {
            new_person.push(line.to_string());
        }

        println!("Введите новый surname: ");
        let mut new_surname = String::new();
        io::stdin()
            .read_line(&mut new_surname)
            .expect("Can't read new surname.");
        for line in new_surname.lines() {
            new_person.push(line.to_string());
        }

        println!("Введите новый middle name: ");
        let mut new_middle_name = String::new();
        io::stdin()
            .read_line(&mut new_middle_name)
            .expect("Can't read new name.");
        for line in new_middle_name.lines() {
            new_person.push(line.to_string());
        }

        println!("Введите новый date of birth: ");
        let mut new_date_of_birth: String = String::new();
        io::stdin()
            .read_line(&mut new_date_of_birth)
            .expect("Can't read new date of birth.");

        let from: NaiveDateTime = new_date_of_birth.parse().unwrap();
        let date_time: DateTime<Local> = Local.from_local_datetime(&from).unwrap();

        for line in date_time {
            new_person.push(line);
        }

        // let person = Person {
        //     name: new_person[0],
        //     surname: new_person[1],
        //     middle_name: new_person[3],
        //     date_of_birth: new_person[4],
        //     gender: false,
        // };

        data.add(new_id, person);
        return "good".to_string();
    }

    fn show_info(&self, data: &PersonStorage) -> String {
        println!("Вывести информацию по id?");
        let mut id: Vec<i32> = vec![];
        let mut find_id: String = String::new();
        io::stdin().read_line(&mut find_id).expect("Can't read id.");
        for i in find_id.lines() {
            id.push(i.parse::<i32>().unwrap_or_default());
        }
        data.get(Some(id));
        return "good".to_string();
    }

    fn show_all_info(&self, data: &PersonStorage) -> String {
        println!("Вывести всю информацию?");
        data.get(None);
        return "good".to_string();
    }

    fn delet_param(&self, _data: &PersonStorage, _param: crate::ui::DeleteParam) -> String {
        todo!()
    }

    fn find_param(&self, _data: &PersonStorage, _param: crate::ui::FindParam) -> String {
        todo!()
    }
}
