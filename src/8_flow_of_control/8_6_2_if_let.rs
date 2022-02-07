/*
 * Another benefit is that `if let` allows use to match non-parameterized enum
 * variants. This is true even in cases where the enum doesn't implement or
 * derive `PartialEq`. In such cases `if Foo::Bar == a` would fail to compile,
 * because instances of the enum cannot be equated, however `if let` will
 * continue to work.
 *
 * Would you like a challenge? Fix the following example to use `if let:`
 */

// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.

enum Foo {Bar}

fn main() {
  let a = Foo::Bar;

  // Variable a matches Foo::Bar
  if let Foo::Bar = a {
  // if Foo::Bar == a {
  // ^-- this causes a compile-time error. Use `if let` instead.
    println!("a is foobar");
  }
}
