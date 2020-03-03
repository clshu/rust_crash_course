// Enums are types which hace a few definite values

enum Movement {
  // Variants
  Up,
  Down,
  Left,
  Right,
}
fn move_avatar(m: Movement) {
  match m {
    Movement::Up => println!("Avatat moving up"),
    Movement::Down => println!("Avatat moving down"),
    Movement::Left => println!("Avatat moving left"),
    Movement::Right => println!("Avatat moving right"),
  }
}
pub fn run() {
  let ava1 = Movement::Up;
  let ava2 = Movement::Down;
  let ava3 = Movement::Left;
  let ava4 = Movement::Right;

  move_avatar(ava1);
  move_avatar(ava2);
  move_avatar(ava3);
  move_avatar(ava4);
}
