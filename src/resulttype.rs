pub fn result_type(){
    let result=divide(12, 0);
    println!("Result: is {:?}",result);
    // match result{
    //     Ok(value)=>println!("Result is {}",value),
    //     Err(e)=>println!("Error is : {}",e),
    // }


}


pub fn divide(x:u32,y:u32)->Result<u32,String>{
    if x==0{
        Err("Division by Zero".to_string())
    }else{
        Ok(x/y)
    }
}