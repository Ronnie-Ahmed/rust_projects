use core::num;
use std::collections::HashMap;
use std::io::{self, Read, Write};

pub struct Account{
    account_id:u32,
    account_name:String,
    balance:f32
}

impl Account{
    pub fn new(account_id:u32,account_name:&str)->Self{
        Account { account_id: account_id, account_name: account_name.to_string(), balance: 0.0 }
    }

    pub fn get_account(self)->String{
        format!("Account Id: {} \n, Account name : {} \n, Balance: {}" ,self.account_id,self.account_name,self.balance)
    }

    pub fn deposit(&mut self,amount:f32){
        self.balance+=amount
    }



    pub fn get_balance(&self)->f32{
        self.balance
    }

    pub fn withdraw(&mut self,amount:f32)->Result<(),&'static str>{
        if amount>self.balance{
            Err("Insufficient Fund")
        }else{
            self.balance-=amount;
            Ok(())
        }
    }
}

pub fn bank_management(){
    println!("Bank Management System");


    let mut accounts:HashMap<u32,Account>=HashMap::new();

    let mut account_id=1;
    loop{
        println!("\n Menu ");
        println!("1. Create Account");
        println!("2. Deposit Amount");
        println!("3. Withdraw Balance");
        println!("4. Check Balance");
        println!("5. Get Account info");
        println!("6. Exit");
        println!("......................................................................................");
        println!("......................................................................................");

        println!("Enter your choice \n ");
        println!("......................................................................................");
        println!("......................................................................................");

        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("error Reading the line");

        let mut choice:u32 = match input.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Invalid input, Please enter a right number");
                continue;
            }
        };


        match choice{
            1 =>{
                let mut name=String::new();
                println!("Enter you account name: \n");
                println!("......................................................................................");
                io::stdin().read_line(&mut name).expect("Read line Error");
                println!("......................................................................................");
                accounts.insert(account_id, Account::new(account_id, &name));
                println!("Account Created Successfully");
                println!("......................................................................................");
                account_id+=1;
            },
            2=>{
                let mut account_number=get_account_number();
                let account=accounts.get_mut(&account_number);
                match account{
                    Some(acc)=>{
                        let mut amount=get_amount("Enter the amount you want to deposit");
                        acc.deposit(amount);
                        println!("Amount Deposited Successfully");
                    }
                    None=>{
                        println!("Invalid Account")
                    }
                }

            }
            _=>println!("Redo")
        }



    }
}


fn get_amount(promt:&str)->f32{
    println!("{}",promt);
    io::stdout().flush().expect("Failed to flush out accounts");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse(){
        Ok(bal)=>bal,
        Err(_)=>{
            println!("Invalid Account info,Enter a valid Amount");
            get_amount(promt)
        }
    }
}

fn get_account_number()->u32{
    println!("Enter you account number: ");
    let mut account=String::new();
    io::stdin().read_line(&mut account).expect("Read ling Failed");

    match account.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Input Invalid");
            get_account_number()
        }
    }
}

