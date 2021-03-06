use chrono::prelude::*;
use rand::Rng;

fn make_birthdays(birthday_vec: &mut Vec<u32>, amount_of_birthdays: u32) {
    for _ in 0..amount_of_birthdays {
        birthday_vec.push(rand::thread_rng().gen_range(1, 366))
    }
}

fn month_number_to_month(month_number: u32) -> &'static str {
    match month_number {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        _ => "December",
    }
}

fn get_birthday_string(birthday_number: u32) -> String {
    let date = NaiveDate::from_yo(0, birthday_number);
    let month = month_number_to_month(date.month());
    let day = date.day().to_string();

    format!("{}, {}", month, &day)
}

fn pair_exists(birthday_vec: &[u32]) -> Option<(usize, usize, u32)> {
    for i in 0..birthday_vec.len() {
        for j in i + 1..birthday_vec.len() {
            if birthday_vec[i] == birthday_vec[j] {
                return Some((i, j, birthday_vec[i]));
            }
        }
    }

    None
}

#[allow(dead_code)]
fn print_birtdays(birthday_vec: &[u32]) {
    for (i, birthday) in birthday_vec.iter().enumerate() {
        println!("{}) {}", format!("{:2}", i), get_birthday_string(*birthday),);
    }
}

fn run_birthday_paradox(number_of_birthdays: u32, runs: u32) {
    let mut matching_runs = 0;
    let mut birthday_vec: Vec<u32> = Vec::new();

    for _ in 0..runs {
        make_birthdays(&mut birthday_vec, number_of_birthdays);
        print_birtdays(&birthday_vec);

        match pair_exists(&birthday_vec) {
            Some(data) => {
                println!(
                    "There are two birthdays, {} and {}, on: {}",
                    data.0,
                    data.1,
                    get_birthday_string(data.2)
                );
                matching_runs += 1;
            }

            None => println!("There are no birthday matches"),
        }

        birthday_vec.clear();
    }

    println!("{}%", (matching_runs as f32 / runs as f32) * 100.0);
}

fn main() {
    run_birthday_paradox(23, 100);
}
