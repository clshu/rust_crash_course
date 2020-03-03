pub fn run() {
  // Print to console
  println!("Hello from print.rs file");
  // Basic formating
  println!("Number: {} Name: {}", 1, "Brass");
  // Psitional Arguments
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Brad", "Mass", "code"
  );
  // Named Arguments
  println!(
    "{name} likes to play {activity}",
    name = "John",
    activity = "Baseball"
  );
  // Placeholder Trait
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
  //Placeholder for debug trait
  println!("{:?}", (12, true, "Hello"));
  //Basic math
  println!("10 + 10 = {}", 10 + 10);
}
