use std::io::stdin;

mod utils {
pub mod utility;
}
mod model;

use crate::model::custom::Person;



#[allow(dead_code)]
fn main() {
 println!("Writing rust"); 
  let mut text = String::new();
let r = stdin().read_line(&mut text).unwrap();
println!("text entered {:?}", text);
println!(" number of bytes read {:?}", r);

 let product_name  = "Location";   
 let provider = String::from("James");
 let price  = 10.2;
 let is_availabble = true;
  let list: [u8; 3] =[1,2,3];
 let amenities = vec!["air conditioner", "fridge", "bathroom"];
//  let food: [uint:2] = [2,3];
println!("{}", product_name);
println!("{provider}");
println!("{:?}", amenities);
println!("{}", amenities[0]);
println!("{}", price);
println!("{}",is_availabble );
println!("{}", product_name.to_string());

let _replace = product_name.replace("Location", "Rust");
let replace_part = product_name.replace("tion", "8");

println!("{}",  replace_part);
println!("{:?}",  list);
let data = utils::utility::concat_str(&provider, product_name);
println!("{}", data);
println!("{}", provider);
utils::utility::multiply(price, 20.4);
let cost  = &provider;
println!("cost is {}", cost);
println!("provider after moving {}", provider);
let sum = utils::utility::add_numbers(3,4);
println!("sum of the numbers is {}", sum);


let color = "red";

enum COLOR{
    RED,
    GREEN,
    BLUE,
    OTHER
}

match color {
    "red" => println!("the color is red"),
    "green" => println!("the color is green"),
    "blue" => println!("the color is blue"),
    _ => println!("the color is other"),
    
}

match COLOR::RED {
    COLOR::RED => println!("the color is red"),
   COLOR::GREEN=> println!("the color is green"),
   COLOR::BLUE => println!("the color is blue"),
    _ => println!("the color is other"),
    
}

let mut my_number = 10;
match my_number {
    10 => {
        my_number *= 2;
      print_result(my_number);
    },
    11 => {
          my_number *= 3;
      print_result(my_number);
    },
    12 => {
          my_number *= 4;
      print_result(my_number);
    },
    13 => {
          my_number *= 4;
      print_result(my_number);
    },
    _ => {
          my_number *= 5;
      print_result(my_number);
    },

}
let mut number = 10;
loop {
    println!("number is {}", number);
     
    if number >= 15{
        break;
}
number += 1;
}
while number < 20 {
    println!("number is {}", number *2);
    number += 1;
}
for digit in 0..10 {
    println!("digit is {}", digit*digit*digit);

}

let mut person1 = Person::new(String::from("James"), String::from("Kirk"), 18, String::from("12, MOleye Street, Alagomeji, Yaba, Lagos"), String::from("08034567890"), String::from("james.clark@gmail.com"));
println!("person1 is {}", person1.first_name);
person1.add_hobby(String::from("reading"));
person1.add_hobby(String::from("dancing"));
let mut hobbies = person1.get_hobbies();
hobbies.push(String::from("volleying"));

println!("person1 hobbies are {:?}", hobbies)
}
fn print_result(my_number: i32) {
    println!("my number after multiplication is {my_number}")
}