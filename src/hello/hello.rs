// fn main() {
//   println!("Hello World!");
//   println!("I'm a Rustacean!");
//   let x = 5 + /* 90 + */ 5;
//   println!("Is `x` 10 or 100? x = {}", x);
//   println!("{} days", 31);
//   println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
//   println!(
//     "{subject} {verb} {object}",
//     object="the lazy dog",
//     subject="the quick brown fox",
//     verb="jumps over"
//   );
//   println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

//   // "     1". 5 white spaces and a "1" .
//   println!("{number:>width$}", number=1, width=6);

//   // You can pad numbers with extra zeroes. This will output "000001".
//   println!("{number:0>width$}", number=1, width=6);

//   println!("My name is {0}, {1} {0}", "Bond", "James");

//   // Create a structure named `Structure` which contains an `i32`.
//   #[allow(dead_code)]
//   struct Structure(i32);

//   // However, custom types such as this structure require more complicated
//   // handling. This will not work.
//   // println!("This struct `{}` wont print...", Structure(3));

//   // This structure cannot be printed either with `fmt::Display` or with
//   // `fmt::Debug`.
//   struct UnPrintable(i32);

//   // The `derive` attribute automatically creates the implementation required
//   // to make this `struct` printable with `fmt::Debug`.
//   #[derive(Debug)]
//   struct DebugPrintable(i32);
// }

// // Derive the `fmt::Debug` implementation for `Structure`. `Structure` is a
// // structure which contains a single `i32`.
// #[derive(Debug)]
// struct Structure(i32);

// // Put a `Structure` inside of the struture `Deep`. Make it printable also.
// #[derive(Debug)]
// struct Deep(Structure);

// fn main() {
//   // Printing with `{:?}` is similar to with `{}`.
//   println!("{:?} months in a year.", 12);
//   println!(
//     "{1:?} {0:?} is the {actor:?} name.",
//     "Slater",
//     "Christian",
//     actor="actor's"
//   );

//   // `Structure` is printable!
//   println!("Now {:?} will print!", Structure(3));

//   // The Problem with `derive` is there is no control over how the results look.
//   // What if I want this to just show a `7`?
//   println!("Now {:?} will print!", Deep(Structure(7)));
// }

// #[derive(Debug)]
// struct Person<'a> {
//   name: &'a str,
//   age: u8
// }

// fn main() {
//   let name = "Peter";
//   let age = 27;
//   let peter = Person { name, age };

//   // Pretty print
//   println!("{:#?}", peter);
// }

// // Import (via `use`) the `fmt` module to make it available.
// use std::fmt;

// // Define a structure for which `fmt::Display` will be implemented. This is a
// // tuple struct named `Structure` that contains an `i32`.
// struct Structure(i32);

// // To use the `{}` marker, the trait `fmt::Display` must be implemented manually
// // for the type.
// impl fmt::Display for Structure {
//   // This trait requires `fmt` with this exact signature.
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     // Write strictly the first element into the supplied output stream: `f`.
//     // Returns `fmt::Result` which indicates whether the operation succeeded or
//     // failed. Note that `write!` uses syntax which is very similar to
//     // `println!`.
//     write!(f, "{}", self.0)
//   }
// }

// use std::fmt; // Import `fmt`

// // A structure holding two numbers. `Debug` will be derived so the results can
// // be contrasted with `Display`.
// #[derive(Debug)]
// struct MinMax(i64, i64);

// // Implement `Display` for `MinMax`.
// impl fmt::Display for MinMax {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     // Use `self.number` to refer to each positional data point.
//     write!(f, "({}, {})", self.0, self.1)
//   }
// }

// // Define a structure where the fields are nameable for comparison.
// #[derive(Debug)]
// struct Point2D {
//   x: f64,
//   y: f64,
// }

// // Similarly, implement `Display` for `Point2D`
// impl fmt::Display for Point2D {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     // Customize so only `x` and `y` are denoted.
//     write!(f, "x: {}, y: {}", self.x, self.y)
//   }
// }

// #[derive(Debug)]
// struct Complex {
//   real: f64,
//   imag: f64,
// }

// impl fmt::Display for Complex {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{} + {}i", self.real, self.imag)
//   }
// }

// fn main() {
//   let minmax = MinMax(0, 14);
  
//   println!("Compare structures:");
//   println!("Display: {}", minmax);
//   println!("Debug: {:?}", minmax);

//   let big_range = MinMax(-300, 300);
//   let small_range = MinMax(-3, 3);

//   println!(
//     "The big range is {big} and the small is {small}",
//     small = small_range,
//     big = big_range,
//   );

//   let point = Point2D { x: 3.3, y: 7.2 };

//   println!("Compare points:");
//   println!("Display: {}", point);
//   println!("Debug: {:?}", point);

//   // Error. Both `Debug` and `Display` were implemented, but `{:b}` requires
//   // `fmt::Binary` to be implemented. This will not work.
//   // println!("What does Point2D look like in binary: {:b}?", point);

//   // Activity

//   let complex = Complex { real: 3.3, imag: 7.2 };

//   println!("Compare complexs:");
//   println!("Display: {}", complex);
//   println!("Debug: {:?}", complex);
// }

// Testcase: List
// Try `write!` to see if it errors. If it errors, return the error. Otherwise
// continue.
// write!(f, "{}", value)?;

// use std::fmt; // Import the `fmt` module.

// // Define a structure named `List` containing a `Vec`.
// struct List(Vec<i32>);

// impl fmt::Display for List {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     // Extract the value using tuple indexing, and create a reference to `vec`.
//     let vec = &self.0;

//     write!(f, "[")?;

//     // Iterate over `v` in `vec` while enumerating the iteration count in
//     // `count`.
//     for (count, v) in vec.iter().enumerate() {
//       // For every element except the first, add a comma.
//       // Use the ? operator to return on errors.
//       if count != 0 { write!(f, ", ")?; }

//       // Activity
//       write!(f, "{0}: {1}", count, v)?;

//       // write!(f, "{}", v)?;
//     }

//     // Close the opened bracket and return a fmt::Result value.
//     write!(f, "]")
//   }
// }

// fn main () {
//   let v = List(vec![1, 2, 3]);
//   println!("{}", v);
// }

use std::fmt::{self, Formatter, Display};

struct City {
  name: &'static str,
  // Latitude
  lat: f32,
  // Longitude
  lon: f32,
}

impl Display for City {
  // `f` is a buffer, and this method must write the formatted string into it
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
    let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

    // `write!` is like `format!`, but it will write the formatted string into a
    // buffer (the first argument)
    write!(
      f,
      "{}: {:.3}°{} {:.3}°{}",
      self.name,
      self.lat.abs(),
      lat_c,
      self.lon.abs(),
      lon_c,
    )
  }
}

#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// Activity
impl Display for Color {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {

    write!(
      f,
      "RGB ({}, {}, {}) 0x{:0>2X}{:0>2X}{:0>2X}",
      self.red,
      self.green,
      self.blue,
      self.red,
      self.green,
      self.blue,
    )
  }
}

fn main() {
  for city in [
    City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
    City { name: "Oslo", lat: 59.95, lon: 10.75 },
    City { name: "Vancouver", lat: 49.25, lon: -123.1 },
  ].iter() {
    println!("{}", *city);
  }
  for color in [
    Color { red: 128, green: 255, blue: 90 },
    Color { red: 0, green: 3, blue: 254 },
    Color { red: 0, green: 0, blue: 0},
  ].iter() {
    println!("{}", *color);
  }
}
