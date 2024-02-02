use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::io::{self};

#[derive(Serialize, Deserialize, Debug)]
pub struct InformationItem {
    id: String,
    name: String,
    surname: String,
    middle_name: String,
    date_of_birth: String,
    gender: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Information {
    information: Vec<InformationItem>,
}

fn main() {
    show_info();
}

pub fn show_info() {
    // Открываем файл с информацией.
    let open_file = std::fs::File::open("config.yaml").expect("Не удается открыть файл.");
    // Десериализуем содержимое файла, записываем в перменную.
    let scrape_config: Information =
        serde_yaml::from_reader(open_file).expect("Не получается прочитать значения.");

    for item in scrape_config.information {
        println!(
            "ID: {}\nName: {}\nSurname: {}\nMiddle name: {}\nDate of birth: {}\nGender: {}",
            item.id, item.name, item.surname, item.middle_name, item.date_of_birth, item.gender
        );
        println!("\n");
    }

    // Записываем данные нового пользователя.
    println!("Вы хотите добавить нового пользователя? да / нет :");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Ошибка ввода");
    answer = answer.trim().to_lowercase();
    if answer != "да" {
        println!("До свидания!");
    } else {
        make_info_of_new_person();
    }
}

pub fn make_info_of_new_person() {
    //Создаем вектор, в которую будет записываться новая информация.
    let mut new_data_person: Vec<String> = Vec::new();

    //Заполняем вектор новыми данными по очереди.
    println!("\n");
    println!("Введите новый id: ");
    let mut input_id = String::new();
    io::stdin()
        .read_line(&mut input_id)
        .expect("Can't read id.");
    for line in input_id.lines() {
        new_data_person.push(line.to_string());
    }

    println!("Введите новый name: ");
    let mut input_name = String::new();
    io::stdin()
        .read_line(&mut input_name)
        .expect("Can't read name.");
    for line in input_name.lines() {
        new_data_person.push(line.to_string());
    }

    println!("Введите новый surname: ");
    let mut input_surname = String::new();
    io::stdin()
        .read_line(&mut input_surname)
        .expect("Can't read surname.");
    for line in input_surname.lines() {
        new_data_person.push(line.to_string());
    }

    println!("Введите новый middle name: ");
    let mut input_middle_name = String::new();
    io::stdin()
        .read_line(&mut input_middle_name)
        .expect("Can't read middle name.");
    for line in input_middle_name.lines() {
        new_data_person.push(line.to_string());
    }

    println!("Введите новый date of birth: ");
    let mut input_date_of_birth = String::new();
    io::stdin()
        .read_line(&mut input_date_of_birth)
        .expect("Can't read date of birth.");
    for line in input_date_of_birth.lines() {
        new_data_person.push(line.to_string());
    }

    println!("Введите новый gender: ");
    let mut input_gender = String::new();
    io::stdin()
        .read_line(&mut input_gender)
        .expect("Can't read gender.");
    for line in input_gender.lines() {
        new_data_person.push(line.to_string());
    }

    // Открываем файл с информацией.
    let open_file = std::fs::File::open("config.yaml").expect("Не удается открыть файл.");
    // Десериализуем содержимое файла, записываем в перменную.
    let mut scrape_config: Information =
        serde_yaml::from_reader(open_file).expect("Не получается прочитать значения.");

    // Заполянем структуру InformationItem элментами из составленного вектора.
    let new_person_item = InformationItem {
        id: new_data_person[0].to_string(),
        name: new_data_person[1].to_string(),
        surname: new_data_person[2].to_string(),
        middle_name: new_data_person[3].to_string(),
        date_of_birth: new_data_person[4].to_string(),
        gender: new_data_person[5].to_string(),
    };
    println!("\n");
    println!("Вы ввели следующие данные: {:#?}", new_data_person);

    // Добавляем созданные данные в переменную с существующими данными.
    scrape_config.information.push(new_person_item);

    // Перезаписываем yaml-файл с обновленными данными.
    let serialized = serde_yaml::to_string(&scrape_config).unwrap();
    std::fs::write("config.yaml", serialized).unwrap();

    println!("\n");
    println!("Данные обновлены!");
    for item in scrape_config.information {
        println!(
            "ID: {}\nName: {}\nSurname: {}\nMiddle name: {}\nDate of birth: {}\nGender: {}",
            item.id, item.name, item.surname, item.middle_name, item.date_of_birth, item.gender
        );
        println!("\n");
    }
}
