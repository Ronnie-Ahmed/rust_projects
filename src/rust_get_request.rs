use error_chain::error_chain;
use std::io::Read;


#[allow(dead_code)]
pub fn rust_request()->Result<(),reqwest::Error>{
    let mut res=reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body=String::new();
    res.read_to_string(&mut body);

    println!("Status is : {} ",res.status());
    println!("Headers {:#?}",res.headers());
    println!("Body is : {}",body);
    Ok(())
}