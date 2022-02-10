/*
 * File hierarchy
 *
 * Modules can be mapped to a file/directory hierarchy. Let's break down the
 * visibility example in files:
 *
 * ```
 * $ tree .
 * .
 * |-- my
 * |   |-- inaccessible.rs
 * |   |-- mod.rs
 * |   `-- nested.rs
 * `-- split.rs (uses file here instead `10_5_file_hierarchy.rs`)
 * ```
 */

// In `split.rs` (uses file here instead `10_5_file_hierarchy.rs`):

// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;

fn function() {
  println!("called `function()`");
}

fn main() {
  my::function();

  function();

  my::indirect_access();

  my::nested::function();
}
