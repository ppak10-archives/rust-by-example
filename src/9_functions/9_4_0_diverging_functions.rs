/*
 * Diverging functions
 *
 * Diverging functions never return. They are marked using `!`, which is an
 * empty type.
 *
 * ```
 * fn foo() -> ! {
 *   panic!("This call never returns.");
 * }
 * ```
 *
 * As opposed to all the other types, this one cannot be instantiated, because
 * the set of all possible values this type can have is empty. Note that, it is
 * different from the `()` type, which has exactly one possible value.
 */

// For example, this function returns as usual, although there is no information
// in the return value.

fn some_fn() {
  ()
}

fn main() {
  let a: () = some_fn();
  println!("This function returns and you can see this line.")
}
