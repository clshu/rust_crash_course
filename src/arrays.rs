// Arrays - Fixed list where elements are the same data type

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{:?}", numbers);

  // Re-assign value
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Get single value
  println!("Value of index 1: {}", numbers[1]);

  // Get array length
  println!("Array Length: {}", numbers.len());

  // Array are stack allocated
  println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

  // Get Silice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
}
