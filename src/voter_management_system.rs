use core::hash;
use std::{collections::HashMap, io::Write};
use ring::digest::{self, Digest,SHA256};
use std::io::{self};

struct Voter{
    name:String,
    age:u32,
    voter_id:Digest,
  
}


impl Voter{
    fn new(name:String,age:u32)->(Self,Digest){
        if age<18{
            println!("Age should be greater than 18");
        }
        assert!(age>=18);
        let data=format!("{}{}",name.clone(),age);
        let hash=digest::digest(&digest::SHA256, data.as_bytes());
        (Voter { name: name.clone(), age: age, voter_id: hash },hash)
    }

 
}


pub fn voting_system(){
    println!("____________Welcome to Voter Management System!_____________");
    let mut voter_id:HashMap<String,Digest>=HashMap::new();
    let mut voter_map:HashMap<String,Voter>=HashMap::new();
    loop{
        println!("Menu");
        println!("1. Register ");
        println!("2. See your Voter Id ");
        println!("3. Check your Info ");

        println!("________________________________________________");
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Error reading Line");

        let mut choice:u32=match  input.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Invalid Input");
                continue;
            }
            
        };

        match choice{
            1=>{
                println!("___ Register as Voter ___");
                println!("Enter your name: ");
                let mut name=String::new();
                io::stdout().flush().expect("Error flushing the io");
                io::stdin().read_line(&mut name).expect("Error entering the name");
                println!("Enter your age: ");
                let mut input_age=String::new();
                io::stdout().flush().expect("Error flushing the io");
                io::stdin().read_line(&mut input_age).expect("Error at reading line");
                let  age:u32=match input_age.trim().parse(){
                    Ok(age)=>age,
                    Err(_)=>{
                        println!("Error");
                        continue;
                    }
                };
                if age<18{
                    println!("Age Should be greater or equal 18");
                    continue;
                }
                let (new_voter,new_voter_id)=Voter::new(name.clone(), age);
                voter_id.insert(name, new_voter_id);
                let hex_digest = new_voter_id.as_ref()
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>(); 
                voter_map.insert(hex_digest, new_voter);
            },
            2=>{
                let mut name=String::new();
                io::stdout().flush().expect("Error");
                println!("Enter you Name");
                io::stdin().read_line(&mut name).expect("Error reading the line");
                let name=name.trim();

                if let Some(check)=voter_id.get(name){
                   println!("Voter id for Voter: {} is {:?}",name,check);
                }else{
                    println!("Voter not Found");
                    continue;
                }


            },
            3=>{
                let mut id=String::new();
                println!("Enter you Voter ID");
                io::stdout().flush().expect("Error");
                io::stdin().read_line(&mut id).expect("Error reading the line");
                let id=id.trim();
                if let Some(val)=voter_map.get(id){
                    println!("Name: {} age: {} Id: {:?}",val.name,val.age,val.voter_id);
                }
                else{
                    println!("Invalid Voter Id");
                    continue;
                }
            },
            4=>break,
            _ =>{

            }
        }

    }
}



