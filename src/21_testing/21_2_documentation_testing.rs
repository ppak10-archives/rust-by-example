/*
 * Documentation testing
 *
 * The primary way of documenting a Rust project is through annotating the sourc
 * code. Documentation comments are written in markdown and support code blocks 
 * in them. Rust takes care about correctness, so these code blocks are compiled
 * and used as tests.
 *
 * Motivation behind documentation tests
 *
 * The main purpose of documentation tests is to serve as examples that exercise
 * the functionality, which is one of the most important guidelines. It allows
 * using examples from docs as complete code snippets. But using `?` makes
 * compilations fail since `main` returns `unit`. The ability to hide some
 * source lines from documentation comes to the rescue: one may write
 * `fn try_main() -> Result<(), ErrorType>`, hide it and `unwrap` it in hidden
 * `main`. Sounds complicated? Here's an example:
 */
