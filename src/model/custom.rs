
#[derive(Debug)]
pub struct Person{
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub address: String,
    pub phone_number: String,
    pub email: String,
    pub hobbies: Vec<String>,
    // friends: Vec<Person>

}

impl Person {
    //static method that creates objects of the Person structure
    pub fn new(first_name: String, last_name: String, age: u8, address: String, phone_number: String, email:String)-> Person{  
        Person{first_name, last_name, age, address, phone_number, email, hobbies: Vec::new()}
    }


      //another static method that creates objects of the Person structure
  pub fn _get_instance(first_name: String, last_name: String) -> Person {
      Person{ first_name, last_name, age:0, address: String::new(), phone_number: String::new(), email: String::new(), hobbies: Vec::new() }
   }
    pub fn add_hobby(&mut self, hobby: String){
        self.hobbies.push(hobby);
    }
    pub fn get_hobbies(&self)-> Vec<String>{
        self.hobbies.clone()
    }

   
    
}