use crate::storage::{Person, PersonStorage};
use crate::ui::UI;
use chrono::NaiveDate;

use rand::distributions;
use rand::{thread_rng, Rng};
use std::{io, vec};

pub struct TUI {}

impl UI for TUI {
    fn add_info(&self, data: &PersonStorage) -> String {
        //Пользователь получает сгенерированный id.
        let random_num: i32 = rand::thread_rng().gen_range(1, 10000);
        let new_id = random_num;
        
        //Создаем вектор, в который будут записываться новые данные.
        //Пользователь вводит новый name и записывает в вектор.
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

        //Пользователь вводит новый surname и записывает в вектор.
        println!("Введите новый surname: ");
        let mut new_surname = String::new();
        io::stdin()
            .read_line(&mut new_surname)
            .expect("Can't read new surname.");
        for line in new_surname.lines() {
            new_person.push(line.to_string());
        }

        //Пользователь вводит новый surname и записывает в вектор.
        println!("Введите новый middle name: ");
        let mut new_middle_name = String::new();
        io::stdin()
            .read_line(&mut new_middle_name)
            .expect("Can't read new middle name.");
        for line in new_middle_name.lines() {
            new_person.push(line.to_string());
        }

        //Пользователь вводит новый date of birth.
        println!("Введите новый date of birth в формате 'yyyy-mm-dd': ");
        let mut string_date: String = String::new();
        io::stdin()
            .read_line(&mut string_date)
            .expect("Wrong date format.");

        let new_date = &string_date.len();
        string_date.truncate(new_date - 1);
        let new_date_str: &str = &string_date.as_str();

        let parsed_date = NaiveDate::parse_from_str(&new_date_str, "%Y-%m-%d").unwrap();

        //Пользователь вводит новый gender.
        let mut new_gender_string: String = String::new();
        println!("Введите gender male/female ?:");
        io::stdin()
            .read_line(&mut new_gender_string)
            .expect("Can't read gender");

        println!("new_gender_string: {}", new_gender_string);

        let mut new_gender: bool = true;
        for line in new_gender_string.lines() {
            if line == "male" {
                new_gender = true;
            } else if line == "female" {
                new_gender = false;
            } else {
                println!("Wrong gender format.")
            }
        }

        let new_person = Person {
            name: new_person[0].to_string(),
            surname: new_person[1].to_string(),
            middle_name: new_person[2].to_string(),
            date_of_birth: parsed_date,
            gender: new_gender,
        };

        data:PersonStorage.add(new_id, new_person);
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
