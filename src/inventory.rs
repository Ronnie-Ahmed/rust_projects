use std::collections::HashMap;

trait Item{
    fn name(&self)->&str;
    fn description(&self)->&str;
}


struct Inventory<T:Item>{
    items:HashMap<String,T>
}


impl <T:Item>  Inventory<T>{
    fn new()->Self{
        Self{
            items:HashMap::new()
        }
    }

    fn add_items(&mut self,item:T){
        self.items.insert(item.name().to_string(), item);
    }

    fn get_name(&self,name:&str)->Option<&T>{
        self.items.get(name)
    }

    fn list_items(&self){
        println!("\n Inventory");
        for item in self.items.values(){
            println!("Name: {} Description: {}",item.name(),item.description());
        }
    }
}
struct Tool{
    name:String,
    category:String,
    description:String
}

impl Tool{
    fn new(name:&str,category:&str,description:&str)->Self{
        Self{
            name:name.to_string(),
            category:category.to_string(),
            description:description.to_string()
        }
    }
}


impl Item for Tool{
    fn name(&self)->&str {
        &self.name
    }
    fn description(&self)->&str {
        &self.description
    }

}



pub fn inventory(){
    let mut book_inventory = Inventory::new();
    let book1 = Tool::new("The Rust Programming Language", "Steve Klabnik", "A book about Rust programming language");
    let book2 = Tool::new("Programming Rust", "Jim Blandy", "A comprehensive guide to Rust programming");
    book_inventory.add_items(book1);
    book_inventory.add_items(book2);

    // Create an inventory for tools
    let mut tool_inventory = Inventory::new();
    let tool1 = Tool::new("Hammer", "Hand Tool", "A tool used to drive nails into wood or other materials");
    let tool2 = Tool::new("Screwdriver", "Hand Tool", "A tool used to tighten or loosen screws");
    tool_inventory.add_items(tool1);
    tool_inventory.add_items(tool2);

    // List items in the inventories
    println!("Books:");
    book_inventory.list_items();
    println!("\nTools:");
    tool_inventory.list_items();
}