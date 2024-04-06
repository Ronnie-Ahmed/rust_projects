use error_chain::example_generated::Error;
use rand::Rng;
use std::{io, str::Chars};


pub fn password_generator(){

    println!("Advanced Random Password Generator");

    let length = get_password_length();

    let include_lowercase = get_bool_input("Include lowercase letters? (yes/no)");
    let include_uppercase = get_bool_input("Include uppercase letters? (yes/no)");
    let include_digits = get_bool_input("Include digits? (yes/no)");
    let include_special_chars = get_bool_input("Include special characters? (yes/no)");

    let password = generate_password(length.unwrap(), include_lowercase, include_uppercase, include_digits, include_special_chars);
    println!("Generated Password: {}", password);

    



}

fn get_password_length()->Result<usize,String>{
    let mut length=String::new();
    io::stdin()
        .read_line(&mut length)
        .expect("Reading line Failed");
    match length.trim().parse(){
        Ok(num) if num > 0 => Ok(num),
        Err(e) => {
            println!("Invalid Input {}", e);
            Err("This is invalid".to_string())
        },
        _ => Err("INvalid".to_string()),
        
        
    }


}
fn get_bool_input(promt:&str)->bool{
    println!("Promt: {}",promt);
    loop{
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Failed to read Line");
        match input.trim().to_lowercase().as_str(){
            "yes"=>return true,
            "no"=>return false,
            _ => println!("INvalid Choices"),
        }
    }
}
fn generate_password(length:usize,include_lowercase:bool,include_uppercase:bool,include_numeric:bool,include_special:bool)->String{
    let mut chars=Vec::new();
    if include_lowercase{
        chars.extend('a'..'z');
    }
    if include_uppercase{
        chars.extend('A'..'Z');
    }
    if include_numeric{
        chars.extend('0'..'9');
    }
    if include_special{
        chars.extend("!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?".chars());
    }if chars.is_empty(){
        println!("No character set selected. Using default character set: lowercase letters, uppercase letters, digits, and special characters.");
        chars.extend('a'..'z');
        chars.extend('A'..'Z');
        chars.extend('0'..'9');
        chars.extend("!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?".chars());
    }
    let mut rng=rand::thread_rng();
    let password=(0..length)
        .map(|_|{
            let index=rng.gen_range(0..chars.len());
            chars[index]
        }).collect();

    password


}
