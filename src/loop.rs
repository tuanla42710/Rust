// use regex::Regex;
use std::io;

fn get_pwd() -> String {
    let mut pw = String::new();
    println!("Please enter password: ");
    io::stdin().read_line(&mut pw).expect("Fail to read line");
    let parse_ps = pw.trim();
    return parse_ps.to_string();
}

fn is_contain_lower_case(_psw : &str) -> bool{
    for ch in _psw.chars(){
        if ch.is_ascii_lowercase(){
            return true;
        }
    }
    return false;
}

fn is_contain_upper_case(_psw : &str)-> bool{
    for ch in _psw.chars(){
        if ch.is_ascii_uppercase(){
            return true;
        }
    }
    return false;
}

fn is_contain_digit(_psw : &str) -> bool{
    for ch in _psw.chars(){
        if ch.is_ascii_digit(){
            return true;
        }
    }
    return false;
}

fn is_contain_special_char(_psw : &str) -> bool{
    for ch in _psw.chars(){
        if ch == '$' || ch == '@' || ch == '#'{
            return true;
        }
    }
    return false;
}


fn check_pw_strength(_psw : &str) ->bool {
    let contain_lower_case = is_contain_lower_case(_psw);
    let contain_upper_case = is_contain_upper_case(_psw);
    let contain_digit = is_contain_digit(_psw);
    let contain_special_char = is_contain_special_char(_psw);
    return contain_lower_case && contain_special_char && contain_upper_case && contain_digit;   
}


fn test2<'b>() -> &'b str {
    let  a = "hello";

    return a;
}

/* 
fn main() {
    let a = get_pwd();
    println!("{}",check_pw_strength(&a));
    let a = test2();
    println!("{a}");
    
}
*/