/*
 * Testing
 *
 * As we know testing is integral to any piece of software! Rust has first class
 * support for unit an integration testing (see this chapter in TRPL).
 *
 * From the testing chapters linked above, we see how to write unit tests and
 * and integration tests. Organizationally, we can place unit tests in the
 * modules they test and integration tests in their own `tests/` directory:
 *
 * ```
 * foo
 * ├── Cargo.toml
 * ├── src
 * │   └── main.rs
 * │   └── lib.rs
 * └── tests
 *     ├── my_test.rs
 *     └── my_other_test.rs
 * ```
 * Each file in `tests` is a separate integration test, i.e. a test that is
 * meant to test your library as if it were being called from a dependent crate.
 *
 * The Testing chapter elaborates on the three different testing styles: Unit,
 * Doc, and Integration.
 *
 * `cargo` naturally provides an easy way to run all of you tests!
 *
 * ```
 * $ cargo test
 * ```
 * 
 * You should see output like this:
 *
 * ```
 * $ cargo test
 *    Compiling blah v0.1.0 (file:///nobackup/blah)
 *     Finished dev [unoptimized + debuginfo] target(s) in 0.89 secs
 *      Running target/debug/deps/blah-d3b32b97275ec472
 * 
 * running 3 tests
 * test test_bar ... ok
 * test test_baz ... ok
 * test test_foo_bar ... ok
 * test test_foo ... ok
 * 
 * test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 * ```
 *
 * You can also run tests whose name matches a patter:
 *
 * ```
 * $ cargo test test_foo
 * ```
 * 
 * ```
 * $ cargo test test_foo
 *    Compiling blah v0.1.0 (file:///nobackup/blah)
 *     Finished dev [unoptimized + debuginfo] target(s) in 0.35 secs
 *      Running target/debug/deps/blah-d3b32b97275ec472
 *
 * running 2 tests
 * test test_foo ... ok
 * test test_foo_bar ... ok
 *
 * test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
 * ```
 *
 * One word of caution: Cargo may run multiple tests concurrently, so make sure
 * that they don't race with each other.
 *
 * One example of this concurrency causing issues is if two tests output a file,
 * such as below:
 *
 * ```
 * #![allow(unused)]
 * fn main() {
 * #[cfg(test)]
 * mod tests {
 *     // Import the necessary modules
 *     use std::fs::OpenOptions;
 *     use std::io::Write;
 * 
 *     // This test writes to a file
 *     #[test]
 *     fn test_file() {
 *         // Opens the file ferris.txt or creates one if it doesn't exist.
 *         let mut file = OpenOptions::new()
 *             .append(true)
 *             .create(true)
 *             .open("ferris.txt")
 *             .expect("Failed to open ferris.txt");
 * 
 *         // Print "Ferris" 5 times.
 *         for _ in 0..5 {
 *             file.write_all("Ferris\n".as_bytes())
 *                 .expect("Could not write to ferris.txt");
 *         }
 *     }
 * 
 *     // This test tries to write to the same file
 *     #[test]
 *     fn test_file_also() {
 *         // Opens the file ferris.txt or creates one if it doesn't exist.
 *         let mut file = OpenOptions::new()
 *             .append(true)
 *             .create(true)
 *             .open("ferris.txt")
 *             .expect("Failed to open ferris.txt");
 * 
 *         // Print "Corro" 5 times.
 *         for _ in 0..5 {
 *             file.write_all("Corro\n".as_bytes())
 *                 .expect("Could not write to ferris.txt");
 *         }
 *     }
 * }
 * ```
 *
 * Although the intent is to get the following:
 *
 * ```
 * $ cat ferris.txt
 * Ferris
 * Ferris
 * Ferris
 * Ferris
 * Ferris
 * Corro
 * Corro
 * Corro
 * Corro
 * Corro
 * ```
 *
 * What actually gets put into `ferris.txt` is this:
 *
 * ```
 * $ cargo test test_foo
 * Corro
 * Ferris
 * Corro
 * Ferris
 * Corro
 * Ferris
 * Corro
 * Ferris
 * Corro
 * Ferris
 * ```
 */
