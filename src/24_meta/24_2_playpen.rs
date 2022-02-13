/*
 * Playpen
 *
 * The Rust Playpen is a way to experiment with Rust code through a web
 * interface. This project is now commonly referred to as Rust Playground.
 */

// Using it with `mdbook`
// In `mdbook`, you can make code examples playable and editable.

// This allows the reader to both run your code sample, but also modify and
// tweak it. The key here is the adding the word `editable` to your codefence
// block separated by a comma.

// Additionally, you can add `ignore` if you want `mdbook` to skip your code
// when it builds and tests.

/// ```rust,editable,ignore
/// //...place your code here
/// ```
fn main() {
  println!("Hello World!");
}

// Using it with docs
// You may have noticed in some of the official Rust docs a button that says
// "Run", which opens the code sample up in a new tab in Rust Playground. This
// feature is enabled if you use the #[doc] attribute called
// `html_playground_url`.
