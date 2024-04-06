mod calculator;
mod rust_get_request;
mod password_generator;
use std::io;

fn main(){
   println!("RUST ClI APPS");

   loop{
      println!("Calculator || press=> 1");
      println!("Get Request || press=> 2");

      println!("Password Generator || press=> 3");
      println!("Quit || press=> 4");

      let mut input=String::new();
      io::stdin().read_line(&mut input).expect("Invalid Reading the input");
      let number=input.trim().parse().expect("Please enter a valid integer");
      match number{
         1 => {
            println!("Loading Calculator...");
            calculator::calculator();
         },
         2 => {
            println!("Loading Get Request....");
            match rust_get_request::rust_request(){
               Ok(_)=>println!("request successfull"),
               Err(e)=>eprintln!("ERROR: {}",e)
            }      },
         3 =>{
            println!("Loading password Generator....");
            password_generator::password_generator();
         }
         6 => break,
         _ => {
            println!("INvalid choices");
            continue;
         },
        

      }


   }
}