use std::collections::Hashmap;

trait Item{
    fn name(&self)->&str;
    fn description(&self)->&str;
}


struct Inventory<T:Item>{
    items:Hashmap<String,T>
}


impl <T:Item>  Inventory<T>{
    fn new(&self)->Self{
        Self{
            items:Hashmap::new()
        }
    }

    fn add_items(&mut self,item:T){
        self.items.insert(item.name.to_string(),item)
    }

    fn get_name(&self,name:&str)->Option<&T>{
        self.items.get(name)
    }

    fn list_items(&self){
        println("\n Inventory");
        for item in self.items.values(){
            println("Name: {} Description: {}",item.name(),item.description());
        }
    }
}