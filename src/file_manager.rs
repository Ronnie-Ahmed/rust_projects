use std::fs;
use std::io::{self, Write};

pub fn file_manager(){
    println!("Welcome to File Manager!");

    loop {
        println!("\nMenu:");
        println!("1. Create File");
        println!("2. Read File");
        println!("3. Update File");
        println!("4. Delete File");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse user input
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                print!("Enter file name: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut file_name = String::new();
                io::stdin().read_line(&mut file_name).expect("Failed to read line");
                create_file(file_name.trim());
            }
            2 => {
                print!("Enter file name: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut file_name = String::new();
                io::stdin().read_line(&mut file_name).expect("Failed to read line");
                read_file(file_name.trim());
            }
            3 => {
                print!("Enter file name: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut file_name = String::new();
                io::stdin().read_line(&mut file_name).expect("Failed to read line");
                print!("Enter text to append: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut text = String::new();
                io::stdin().read_line(&mut text).expect("Failed to read line");
                update_file(file_name.trim(), text.trim());
            }
            4 => {
                print!("Enter file name: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut file_name = String::new();
                io::stdin().read_line(&mut file_name).expect("Failed to read line");
                delete_file(file_name.trim());
            }
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a number from 1 to 5."),
        }
    }
}


fn create_file(file_name:&str){
    match fs::write(file_name, ""){
        Ok(_)=>println!("File Created Successfully. {}",file_name),
        Err(e)=>println!("Error Creating the file. {}",e)
    }
}

fn read_file(file_name:&str){
    match fs::read_to_string(file_name){
        Ok(content)=> println!("Read to the file. {}",content),
        Err(e)=> println!("Error reading the file. {}",e)
    }
}

fn update_file(file_name: &str, text: &str) {
    match fs::write(file_name, text) {
        Ok(_) => println!("Text appended to '{}'.", file_name),
        Err(err) => println!("Error: {}", err),
    }
}
fn delete_file(file_name: &str) {
    match fs::remove_file(file_name) {
        Ok(_) => println!("File '{}' deleted successfully.", file_name),
        Err(err) => println!("Error: {}", err),
    }
}