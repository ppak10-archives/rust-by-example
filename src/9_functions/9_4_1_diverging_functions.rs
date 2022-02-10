// As opposed to this function, which will never return the control back to the
// caller.

#![feature(never_type)]

fn main() {
  let x: ! = panic!("This call never returns");
  println!("You will never see this line!");
}
