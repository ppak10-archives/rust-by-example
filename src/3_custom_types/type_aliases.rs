enum VeryVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Subtract,
}

// The most commmon place you'll see this is in `impl` blocks using the `Self`
// alias.
impl VeryVerboseEnumOfThingsToDoWithNumbers {
  fn run(&self, x: i32, y: i32) -> i32 {
    match self {
      Self::Add => x + y,
      Self::Subtract => x - y,
    }
  }
}

// Create a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
  // We can refer to each variant via its alias, not its long and inconvenient
  // name.
  let x = Operations::Add;
}
