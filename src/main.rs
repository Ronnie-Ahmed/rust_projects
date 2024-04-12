mod calculator;
mod rust_get_request;
mod password_generator;
mod todo;
mod simple_chat;
use std::io;
mod file_tutorial;
mod bank_management;
mod resulttype;
mod file_manager;

fn main(){

   println!("......................................................................................");
   println!("......................................................................................");
   println!(".......................................RUST ClI APPS..................................");
   println!("......................................................................................");
   println!("......................................................................................");
   // resulttype::result_type();



   



   loop{
      println!("......................................................................................");
      println!("......................................................................................");
      println!("Calculator || press=> 1");
      println!("Get Request || press=> 2");
      println!("Password Generator || press=> 3");
      println!("Todo List || press=> 4");
      println!("Simple_Chat || press => 5");
      println!("Bank_management_system || press => 6");
      println!("File Manager || press 7");
      println!("Quit || press=> 8");
      println!("......................................................................................");
      println!("......................................................................................");


      let mut input=String::new();
      println!("\n\nEnter Your Choices");
      println!("......................................................................................\n\n");

      io::stdin().read_line(&mut input).expect("Invalid Reading the input");
      let number=input.trim().parse().expect("Please enter a valid integer");
      match number{
         1 => {
            println!("\n\nLoading Calculator...\n\n");
            calculator::calculator();
         },
         2 => {
            println!("\n\nLoading Get Request....\n\n");
            match rust_get_request::rust_request(){
               Ok(_)=>println!("request successfull"),
               Err(e)=>eprintln!("ERROR: {}",e)
            }      
         },
         3 =>{
            println!("\n\nLoading password Generator....\n\n");
            password_generator::password_generator();
         },
         4 =>{
            println!("\n\nLoading Todo List....\n\n");
            todo::todo();
            },
         5 => {
            println!("\n\nSimple Chatting Loading......\n\n");
            simple_chat::Simple_Chat();
         },
         6=>{
            println!("\n\n Bank Management System is Loading......\n\n");
            bank_management::bank_management();

         },
         7=>{
            println!("\n\n File Manager is Loading......\n\n");
            file_manager::file_manager()

         }
         8 => break,
         _ => {
            println!("INvalid choices");
            continue;
         },
        

      }


   }
}