use std::{io, result};

pub fn calculator(){
    println!("Welcome to RUST Calculator");

    loop {
        println!("Enter the first number");
        let num1=match  read_input(){
            Ok(num)=>num,
            Err(msg)=>{
                println!("Error : {}",msg);
                continue;
            }
            
        };

        println!("Enter the second Number");
        let num2=match read_input() {
            Ok(num)=>num,
            Err(msg)=>{
                println!("Error is : {}",msg);
                continue;
            }
            
        };

        println!("Select operation:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exponentiation");
        println!("6. Quit");

        let choice=match read_input() {
            Ok(num)=>num,
            Err(msg)=>{
                println!("Invalid error: {}",msg);
                continue;
            }
            
        };

        let result=match choice as i32 {
            1=>num1+num2,
            2=>num1-num2,
            3=>num1*num2,
            4=>{
                if num2==0.0{
                    println!("Error num2 , Can not Divide by Zero");
                    continue;
                }else{
                    num1/num2
                }
            },
            5=>num1.powf(num2),
            6=>break,
            _ =>{
                println!("Invalid Choices");
                continue;
            }
            
        };
        println!("Result is : {}",result);
        println!("Do you want another round or quit yes/no");
        let mut answer=String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("error reading line");
        if answer.trim().eq_ignore_ascii_case("yes") {
            continue;
        }
        if answer.trim().eq_ignore_ascii_case("no"){
            break;
        }

    }
   

}

pub fn read_input()->Result<f64,String>{
    let mut input=String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");
    match input.trim().parse(){
        Ok(num)=>Ok(num),
        Err(_)=>Err("Invalid User Input".to_string())
    }
}

