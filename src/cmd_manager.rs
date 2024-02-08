use crate::storage::{Person, PersonStorage};
use crate::ui::UI;
use chrono::NaiveDate;

use std::{io, vec};

pub struct TUI {}

impl UI for TUI {
    fn add_info(&self, data: &crate::storage::PersonStorage) -> String {
        println!("Введите новый id?");
        //let mut new_id: Vec<i32> = vec![];
        let mut id: String = String::new();
        io::stdin().read_line(&mut id).expect("Can't read new id.");
        let new_id = id.parse::<i32>().unwrap_or_default();

        // for i in id.lines() {
        //     new_id.push(i.parse::<i32>().unwrap_or_default());
        // }

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

        println!("Введите новый date of birth в формате 'yyyy-mm-dd': ");
        let mut string_date: String = String::new();
        io::stdin()
            .read_line(&mut string_date)
            .expect("Wrong date.");

        let new_date = &string_date.len();
        string_date.truncate(new_date - 1);
        let new_date_str: &str = &string_date.as_str();

        let parsed_date = NaiveDate::parse_from_str(&new_date_str, "%Y-%m-%d").unwrap();

        let mut new_gender_string: String = String::new();
        println!("Введите gender М/Ж ?:");
        io::stdin()
            .read_line(&mut new_gender_string)
            .expect("Can't read gender");

        let new_gender;
        if new_gender_string == "М" {
            new_gender: bool = true;
        } else if new_date_str == "Ж" {
            new_gender: bool = false;
        }

        let person = Person {
            name: new_person[0].to_string(),
            surname: new_person[1].to_string(),
            middle_name: new_person[2].to_string(),
            date_of_birth: parsed_date,
            gender: new_gender,
        };

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
