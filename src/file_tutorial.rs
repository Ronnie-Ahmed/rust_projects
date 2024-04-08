use std::{fs::File, io::{Read, Write}};
use std::fs::{OpenOptions,write};

pub fn file_tutorial(){
    println!("Hello there");
    // write_create();
    // read_file();
    append_to_file();


}

pub fn write_create(){

    let mut file=File::create("example.txt").expect("File not created");
    file.write_all(b"Write something to the file").expect("Writing failed");
}

pub fn read_file(){
    let mut file=File::open("example.txt").expect("Reading file errored");
    let mut content=String::new();
    file.read_to_string(&mut content).expect("Reading Failed");
    println!("{}",content);
}

pub fn append_to_file()->std::io::Result<()>{

    let mut file=OpenOptions::new()
        .append(true)
        .create(true)
        .open("newfile.txt")?;
        

    file.write_all(b"this is going to be wild  \n")?;
    Ok(())
    
}