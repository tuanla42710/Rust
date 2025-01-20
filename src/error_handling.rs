use std::fmt::Error;
/*
    The '?' Operator is used for propagating errors or early returning from a function when an error
    occurs. It works by unwrapping the Result or Option value returning early with an error if it contains Err or None, respectively
    This also , avloids the verbosity of match statement when handling error in rust
    The ? operator in Rust can be used to convert type of error
    
    let c = concat_string(s1, s2)?; // 

    ---------------------------------------------------------------------------------------------------------------------

    unwrap() and expect() :
    - Both expect and unwrap are methods provided by the Option and Result enums for extractiong their value
    - unwrap() simply return the contained value of Some or Ok, if the value is None or Err it will panic and terminate program
    - expect() work in the same way as unwrap but allow the programmer to provide a custom error message as a parameter to the method, this error msg is
      printed to a console when a panic occur
    let c = concat_string(s1, s2).unwrap();
    let c = concat_string(s1, s2).expect("String canot be empty"); 

    unwrap_err()
    - unwrap_err() will panic if the Result is not an Err

*/
/*
    Standard library error types :
    There are several types of errors that commonly occur while codeing, regardless of the programming language being used

    - I/O Erros
    - Numeric and Conversion Error
    - Formatting and data conversion Error
    - Network Error

    Handling I/O errors 
    in rust std::io::Error error type is used for I/O-related errors , such as file operation or network errors

*/
use std::io;
use std::fs;
use std::i32;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum AddStringError {
    LengthMissMatch ,
    EmptyString 
}

impl AddStringError {
    fn describe(&self) {
        match self {
            AddStringError::EmptyString => {
                println!("error code 404");
                println!("empty string is detected");
            }
            AddStringError::LengthMissMatch => {
                println!("error code is 500");
                println!("length between two string must be equal");
            }
        }
    }
}




fn concat_string (s1: &str, s2 : &str)-> Result<String, AddStringError> {
    if s1.is_empty() || s2.is_empty() {
        return Err(AddStringError::EmptyString);
    } 
    if s1.len() != s2.len() {
        return Err(AddStringError::LengthMissMatch);
    }
    let c = format!("{} {}", s1, s2);
    Ok(c)
}

fn rename_file(from : &str, to : &str) -> Result<(), io::Error>{
     match fs::rename(from, to){
        Ok(_) => Ok(()),
        Err(e) => { 
            eprintln!("{}",e);
            Err(e)
    } 
}
//    fs::rename(from, to)?;
}

fn parse_integer(s: &str) -> Result<i32, ParseIntError>{
    let rc = i32::from_str(s);
    rc
}

fn parse_integer_from_string(input : &str) ->Result<i32, String>{
    match i32::from_str(input){
        Ok(num) => Ok(num),
        Err(e)=> match e.kind(){
            std::num::IntErrorKind::Empty => Err("Empty string".to_string()),
            std::num::IntErrorKind::InvalidDigit => Err("Invalid digit".to_string()),
            std::num::IntErrorKind::Zero => Err("Zero".to_string()),
            _ => Err("Unknown error".to_string())
        }
    }
}

fn convert_udf() -> Result<i32, io::Error>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to read line");
    match i32::from_str(&input.trim()){
        Ok(num) => Ok(num),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("{}", e))) //Err(std::io::ErrorKind::InvalidData.into())
    }
}

/*    
fn main() {
    let rc = convert_udf();
    match rc {
        Ok(num) => println!("Number is {}", num),
        Err(e) => eprintln!("Error occur {}", e)
    }
    /* let result = parse_integer_from_string("99x");
    match result {
        Ok(num) => println!("parse number {}", num),
        Err(e) => println!("Error occur {}", e),
    } */
}
*/