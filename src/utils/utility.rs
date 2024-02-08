pub fn add_numbers(first:i32, second:i32)->i32{
    first + second

}


pub fn concat_str(s1: &str, s2: &str) -> String {
    s1.to_string() + s2
}

pub fn multiply(first:f32,second:f32 ){
    let product = first * second;
 println!("the product of the numbers is: {}", product);
}