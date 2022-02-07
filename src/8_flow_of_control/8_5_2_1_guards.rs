/*
 * Note that the compiler does not check arbitrary expressions for whether all
 * possible conditions have been checked. Therefore, you must use the `_`
 * pattern at the end.
 */

fn main() {
  let number: u8 = 4;

  match number {
    i if i == 0 => println!("Zero"),
    i if i > 0 => println!("Greater than zero"),
    _ => println!("Fell through"), // This should not be possible to reach
  }
}
