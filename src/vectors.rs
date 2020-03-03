// Vectors - Resizable array

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
  println!("{:?}", numbers);

  // Re-assign value
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Add on to vector
  numbers.push(40);
  numbers.push(50);
  println!("{:?}", numbers);
  // Pop value
  let last = numbers.pop();

  println!("{:?}", numbers);
  println!("Poped value: {:?}", last);
  // Get single value
  println!("Value of index 1: {}", numbers[1]);

  // Get vector length
  println!("Vector Length: {}", numbers.len());

  // Vector are stack allocated
  println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

  // Get Silice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // Loop thru vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop thru vector values and mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("{:?}", numbers);
}
