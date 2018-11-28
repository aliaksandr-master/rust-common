extern crate csv;

mod some;
mod aitline;

use aitline::load_passengers;
use some::submodule::{calc_sum/*, ...*/};

use std::path::PathBuf;
use std::env;

use std::fmt::{Display,Formatter,Error};


// Result<T, E> - T успешно, E ошибка

//#[derive(Display)]
pub struct ZeroDivisionError;
impl Display for ZeroDivisionError {
    fn fmt (&self, fmt :&mut Formatter) -> Result<(), Error>{
        fmt.write_str("ZeroDivisionError !!!")
    }
}

pub fn calc_division(a: f32, b: f32) -> Result<f32, ZeroDivisionError> {
    if b != 0.0 {
        return Ok(a / b)
    }

    return Err(ZeroDivisionError)
}

fn main() {
    let mut sum = calc_sum(1.32, 1.55);
    let cwd = env::current_dir().unwrap();


    let div = calc_division(3.3, 5.3).unwrap_or(0.0);
    let div2 = calc_division(3.3, 0.0).unwrap_or_else(|err|5.5);
    let div3 = calc_division(3.3, 0.0).unwrap_or_else(|err|{
        println!("{}", err.to_string());
        return 5.5;
    });
    let div4 = match calc_division(3.3, 0.0){
        Ok(3.3) => 3.4,
        Ok(value) => value,
        Err(err) => 0.0,
        _ => 0.0,
    };


    sum += 3.0;

    let some_string = format!("sum {}", sum);

    println!("Hello, world ({})!", some_string);

    passengers_file_path = cwd.join("data").join("airline_passengers.csv");

    let passgers = load_passengers(passengers_file_path);

//    panic!("KERNEL PANIC!")
//    unreachable!("some ") // что бы пометить что этот кейс не должен пройти
//    unimplemented!("some ") // что бы пометить что эта функция еще не реализована
//    assert!(some_string.len() < 3)

}
