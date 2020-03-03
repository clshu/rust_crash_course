// Conditionals - Used to check the condition of something and act

pub fn run() {
  let age: u8 = 18;
  let check_id: bool = true;
  let know_person_of_age = false;

  if age >= 21 && check_id || know_person_of_age {
    println!("What woud you like to drink?");
  } else if age < 21 && check_id {
    println!("Sorry you have to leave");
  } else {
    println!("I'll need to see your ID.");
  }

  // Shorthand If
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is of Age? {}", is_of_age);
}
