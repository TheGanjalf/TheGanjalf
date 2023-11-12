// use std::{io, ptr::null};

// fn main() {
//     let lunghezza_tagio = 0;
//     let quantita_verge = 0;
//     let tipo_verga: [u32; 2] = [6000, 3000];
//     let mut tipo_verga_scelta = String::new();

//     println!("Inserire 1 per verga da 6000, 2 per verga da 3000");
//     io::stdin()
//         .read_line(&mut tipo_verga_scelta)
//         .expect("Imposibile leggere");

//     let mut index_verga: u32 = tipo_verga_scelta
//         .trim()
//         .parse()
//         .expect("Inserire un numero!");

//     println!("tipo di verga selezionata { }", tipo_verga[index_verga]);
//     // per codewar
//     // let split = test_string.split_ascii_whitespace();

//     // for (_pos, _word) in split.enumerate() {
//     //     println!("word = {}", _word);
//     //     let mut temp = _word.chars().rev();
//     //     for _char in temp {
//     //         println!("{}", _char);
//     //     }
//     // }
// }

use std::{any::type_name, fmt::format, path::Prefix};

fn main() {
    let _simon = UserInfo {
        name: Into::into("Simone"),
        surname: Into::into("Braido"),
        date_borday: 01081991,
    };

    println!("Nome: {:?}", _simon.name);
    println!("Cognome: {:?}", _simon.surname);
    println!("Data: {:?}", _simon.date_borday);
    let _simon = UserInfo {
        name: Into::into("Gino"),
        surname: Into::into("Pino"),
        date_borday: 01081991,
    };

    println!("Nome: {:?}", _simon.name);
    println!("Cognome: {:?}", _simon.surname);
    println!("Data: {:?}", _simon.date_borday);

    let length = 12_f32;
    let width = 12_f32;
    let height = 12_f32;
    let result = get_volume_of_cuboid(length, width, height);
    println!("{:?}", result);

    let imb = bmi(26, 1.1);
    println!("{}", imb);

    let number: i64 = 10;
    let mut numb_to_sinrg = number.to_string();
    let mut tot: i64 = 0;
    let mut loops = true;
    while loops {
        for char_num in numb_to_sinrg.chars() {
            let mut test = char_num as i64;
            test -= 48;
            tot += test;
        }
        if tot > 10 {
            numb_to_sinrg = tot.to_string();
            tot = 0;
        } else {
            loops = false;
        }
    }
    println!("{:?}", tot);
}

struct UserInfo {
    name: Box<str>,
    surname: Box<str>,
    date_borday: u32,
}

// fn are_you_playing_banjo(name: &str) -> String {
//     let low_case = name.to_lowercase();
//     let start_with_r = format!("{}{}", name, " plays banjo".to_owned());
//     let non_start_with_r = format!("{}{}", name, " dose not plays banjo".to_owned());
//     if (low_case.starts_with("r")) {
//         return start_with_r;
//     } else {
//         return non_start_with_r;
//     }
// }
fn get_volume_of_cuboid(length: f32, width: f32, height: f32) -> f32 {
    length * width * height
}

fn square_sum(vec: Vec<i32>) -> i32 {
    let mut res = 0;
    for date in vec {
        res += date * date;
    }
    res
}

fn cockroach_speed(s: f64) -> i64 {
    let conversion = s / 0.036;
    conversion.round() as i64
}
fn bmi(weight: u32, height: f32) -> &'static str {
    let ibm = weight as f32 / (height.sqrt().round());
    println!("{}", ibm);
    match ibm {
        0.0..=18.5 => return "Underweight",
        18.6..=25.0 => return "Normal",
        25.1..=30.0 => return "Overweight",
        _ => return "Obese",
    }
}
