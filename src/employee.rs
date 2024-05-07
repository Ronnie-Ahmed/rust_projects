use std::borrow::Borrow;
use std::io;

use tokio::io::stdin;

use crate::employee;

#[derive(Debug)]
enum Department{
    It,
    Hr,
    Finance,
    Sales
}

trait EmplyeeBehavior{
    fn greet(&self) -> String;
    fn get_department(&self)-> &Department;
    fn get_age(&self)->u32;
}

#[derive(Debug)]
struct Employee{
    name:String,
    age:u32,
    department:Department
}

impl EmplyeeBehavior for Employee{
    fn greet(&self) -> String {
        format!("Hello my name is {} , I work in the {:?} Department",self.name,self.department)
    }

    fn get_department(&self)->&Department {
        self.department.borrow()
    }
    fn get_age(&self)->u32 {
        self.age
    }
}

impl Employee{
    

    pub fn create(name:String,age:u32,department:Department)->Result<Self, &'static str>{
        if  age < 18{
            Err("You are not eligible to be employee")
        }else{
            Ok(Self { name: name, age: age, department: department })
        }
    }
}

trait CompanyBehavior{
    fn add_employee(&mut self,employee:Employee);
    fn find_employee(&self,name:&str)->Option<&Employee>;
    // fn calculate_average_age(&self,department:Department)->Option<f64>;
}

struct Company{
    employees:Vec<Employee>
}

impl Company{
    fn new()->Self{
        Self { employees: Vec::new() }
    }

}


impl CompanyBehavior for Company{
    fn add_employee(&mut self,employee:Employee) {
        self.employees.push(employee)
    }
    fn find_employee(&self,name:&str)->Option<&Employee> {
        self.employees.iter().find(|&e| e.name==name)
    }
    // fn calculate_average_age(&self,department:Department)->Option<f64> {
    //     let department_employee:Vec<&Employee>=self.employees.iter().filter(|&e| e.department==department).collect();
    //     let total_age:u32=department_employee.iter().map(|&e| e.age).sum();
    //     let count=department_employee.len() as f64;
         
    //      if count>0.0{
    //         Some(total_age as f64/count)
    //      }else{
    //         None
    //      }
    // }
}


pub fn employee(){
    println!("This is Employee page");

    let mut company=Company::new();
    loop{
        println!("1. Create Employee");
        println!("2. Find Employee");
        println!("3. Calculate Average Age by Department");
        println!("4. Exit");
        println!("Enter your choice:");

        let mut input=String::new();

        io::stdin().read_line(&mut input).expect("Error Reading line");
       
        let mut choice:u32=match  input.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Invalid Input");
                continue;
            }
            
        };
        match choice{
            1=>{
                println!("Creating the Employee");
                println!("_____________________");
                println!("\n Enter your name: ");
                let mut name=String::new();
                io::stdin().read_line(&mut name).expect("Error Reading line");
                let name=name.trim().to_string();
                println!("_____________________");
                println!("\n Enter your Age: ");

                let mut age=String::new();
                io::stdin().read_line(&mut age).expect("Error reading line");
                let age:u32=match age.trim().parse(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Enter a valid age");
                        continue;
                    } 
                };
                println!("_____________________");
                println!("\n Enter your Department: ");


                println!("_____________________");
                println!("Availabe Department:: IT SALES FINANCE HR \n\n");
                println!("\n Enter your Department: ");



                let mut department=String::new();
                io::stdin().read_line(&mut department).expect("Error reading line");

                let department= match department.trim().to_lowercase().as_str(){
                    "it"=>Department::It,
                    "hr"=>Department::Hr,
                    "finance"=>Department::Finance,
                    "sales"=>Department::Sales,
                    _=>{
                        println!("Enter a valid Department name");
                        continue;
                    }
                };

                match Employee::create(name, age, department){
                    Ok(employee)=>{
                        company.add_employee(employee)
                    }
                    Err(_)=>{
                        println!("Error Adding a new Employee");
                        continue;

                    }
                }

                
            },
            2 => {
                println!("Enter employee name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_lowercase();

                if let Some(employee) = company.find_employee(&name) {
                    println!("{:?}", employee);
                } else {
                    println!("Employee not found.");
                }
            },
            4 => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid choice!"),
        }
    }
}