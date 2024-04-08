use std::io::{self, Read, Write};
use std::fs::{File,OpenOptions};
use std::prelude::*;




pub fn todo(){
    println!("......................................................................................");
    println!("......................................................................................");
    println!("......................................................................................");
    println!("...........................Welcome to Todo List.......................................");
    println!("......................................................................................");
    println!("......................................................................................");
    println!("......................................................................................");

    loop {
        println!("......................................................................................");
        println!("......................................................................................");
        println!("\nMenu:");
        println!("......................................................................................");
        println!("......................................................................................");
        println!("......................................................................................");


        println!("1. View tasks");
        println!("2. Add task");
        println!("3. Remove task");
        println!("4. Exit");
        println!("......................................................................................");
        println!("......................................................................................");
        println!("......................................................................................");

        let mut choices=get_input("Enter your choices: ");
        println!("......................................................................................");




        match  choices.trim() {
            "1"=>view_task(),
            "2"=>add_task(),
            "3"=>task_remove(),
            "4"=>break,
            _ => println!("INvalid Choices")
            
        }
    }


}

pub fn get_input(prompt:&str)->String{
    println!("{}",prompt);
    println!("......................................................................................");
    println!("......................................................................................");

    io::stdout().flush().unwrap();


    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Input error");
    input
}

pub fn add_task(){
    let task=get_input("Enter the task: ");
    let mut file=OpenOptions::new()
            .append(true)
            .create(true)
            .open("task.txt")
            .expect("Error opening and writing to the file");
    writeln!(file,"{}",task.trim()).expect("Error");
    println!("......................................................................................");
    println!("Task Added Successfully");
}

pub fn view_task(){
    println!("......................................................................................");
    println!("......................................................................................");

    let mut file=match File::open("task.txt"){
        Ok(file)=>file,
        Err(_)=>{
            println!("no task found");
            return;
        }
    };

    let mut content=String::new();
    file.read_to_string(&mut content).expect("Error reading the file");

    if content.is_empty(){
        println!("no task found")
    }else{
        println!("Tasks");
        println!("{}",content);
    }
}

pub fn task_remove(){
    let task_index:usize=get_input("Enter the Task Index to remove: ").trim().parse().expect("Invalid Index");

    let mut file=match File::open("task.txt"){
        Ok(file)=>file,
        Err(_)=>{
            println!("There are no tasks");
            return;
        }
    };
    let mut content=String::new();
    file.read_to_string(&mut content).expect("Error Reading to the file");
    let tasks:Vec<&str>=content.lines().collect();
    if task_index>tasks.len(){
        println!("Invalid Index");
        return;
    }

    let mut updated_task=String::new();
    for (i ,task) in tasks.iter().enumerate(){
        if i !=task_index{
            updated_task.push_str(task);
            updated_task.push_str("\n");
        }
    }
    let mut file=File::create("task.txt").expect("error");
    file.write_all(&mut updated_task.as_bytes()).expect("error");
    println!("......................................................................................");
    println!("Task Remove Successfully");

}