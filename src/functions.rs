// Functions -- Used to store blocks of code for re-use

pub fn run() {
  greeting("Hello", "Jane");

  // Bind function value to variables
  let sum = add(6, 4);
  println!("Sum: {}", sum);

  // Closure
  let n3: i32 = 10;
  let add_num = |n1: i32, n2: i32| n1 + n2 + n3;

  println!("Add Num {}", add_num(10, 20));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}
