// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own
pub fn run() {
  // let hello = "Hello";
  let mut beta = String::from("Beta ");

  // Get length
  println!("Length: {}", beta.len());

  // Push char
  beta.push('G');
  // Push string
  beta.push_str("arma");
  println!("{}", beta);
  // Capacity in bytes
  println!("Capacity: {}", beta.capacity());
  // Is empty
  println!("Is Empty: {}", beta.is_empty());
  // Contains
  println!("Contains Grama: {}", beta.contains("Garma"));
  // Loop through string by whitespace
  for word in beta.split_whitespace() {
    println!("{}", word);
  }
  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  // Testing
  assert_eq!(2, s.len());
  println!("{}", s);
}
