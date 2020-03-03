// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scope language

pub fn run() {
  let name = "Brad";
  let mut age = 40;
  println!("My name is {} {}", name, age);
  age = 41;
  println!("My name is {} {}", name, age);

  //Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Peter", 42);
  println!("My name is {}. My age is {}.", my_name, my_age);
}
