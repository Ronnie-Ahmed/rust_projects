use std::io::{self,Write};
use std::fs::{File,OpenOptions};
use std::prelude::*;




pub fn todo(){

    let input=get_input("Enter the input:  ");
    println!("{}",input);

}

pub fn get_input(prompt:&str)->String{
    println!("{}",prompt);
    io::stdout().flush().unwrap();


    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("INput error");
    input
}