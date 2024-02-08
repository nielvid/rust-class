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

    pub fn new(first_name: String, last_name: String, age: u8, address: String, phone_number: String, email:String)-> Person{  
        Person{first_name, last_name, age, address, phone_number, email, hobbies: Vec::new()}
    }

    pub fn add_hobby(&mut self, hobby: String){
        self.hobbies.push(hobby);
    }
    pub fn get_hobbies(&self)-> Vec<String>{
        self.hobbies.clone()
    }
    
}