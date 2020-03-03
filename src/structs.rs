// Struct -- Used to create data types

struct Color {
  red: u8,
  green: u8,
  blue: u8,
}
// Tuple struct
struct NewColor(u8, u8, u8);

// Struct with functions

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Construct person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }
  // Get full name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // Set lastname
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  // Name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}
pub fn run() {
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };

  c.red = 200;

  println!("Color: {} {} {}", c.red, c.green, c.blue);

  let mut nc = NewColor(255, 0, 0);
  nc.0 = 210;
  println!("New Color: {} {} {}", nc.0, nc.1, nc.2);

  let mut p = Person::new("John", "Doe");
  println!("Firstname: {} Lastname: {}", p.first_name, p.last_name);
  println!("Fullname: {}", p.full_name());
  p.set_last_name("Puppy");
  println!("Fullname: {}", p.full_name());
  println!("Tuple {:?}", p.to_tuple());
}
