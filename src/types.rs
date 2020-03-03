/*
Primitive types --
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean: bool
Character: char
Tuples
Arrays
*/

/*
Rust is a statistically typed language, which means that is must know the types of all variables
at compile time. However, the compiler can usually infer ehat type type we want to use based on the value
and how we use it.
*/

pub fn run() {
  // Defaukt is "i32"
  let x = 1;

  // Default is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 3141599;
  println!("x: {} y: {} z: {}", x, y, z);

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active = true;
  // Get boolean from expression
  let is_greater: bool = 10 > 5;
  println!("{:?}", (is_active, is_greater));

  // Char
  let a1 = 'a';
  let smile = '\u{1f600}';
  println!("{:?}", (a1, smile));
}
