/*
 * Unsafe Operations
 *
 * As an introduction to this section, to borrow from the official docs, "one
 * should try to minimize the amout of unsafe code in a code base." With that in
 * mind, let's get started! Unsafe annotations in Rust are used to bypass
 * protections put in place by the compiler; specifically, there are four
 * primary things that unsafe is used for:
 * * dereferencing raw pointers
 * * calling functions or methods which are `unsafe` (including calling a
 *   function over FFI, see a previous chapter of the book)
 * * accessing or modifying static mutable variables
 * * implementing unsafe traits
 *
 * Raw Pointers
 *
 * Raw pointers `*` and references `&T` function similarly, but references are
 * always safe because they are guaranteed to point to valid data due to the
 * borrow checker. Dereferencing a raw pointer can only be done through an
 * unsafe block.
 */

fn main() {
  let raw_p: *const u32 = &10;

  unsafe {
    assert!(*raw_p == 10);
  }
}
