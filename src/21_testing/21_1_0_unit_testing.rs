/*
 * Unit testing
 *
 * Tests are Ruest functions that verify that the non-test code is functioning
 * in the expected manner. The bodies of test functions typically perform some
 * setup, run the code we want to test, then assert whether the results are what
 * we expect.
 *
 * Most unit tests go into a `tests` mod with the `#[cfg(test)]` attribute. Test
 * functions are marked with the `#[test]` attribute.
 *
 * Tests fail when something in the test function panics. There are some helper
 * macros:
 * * `assert!(expression)` - panics if expression evaluates to `false`.
 * * `assert_eq!(left, right)` and `assert_ne!(left, right)` - testing left and
 *   right expressions for equality and inequality respectively.
 *
 * ```
 * pub fn add(a: i32, b: i32) -> i32 {
 *     a + b
 * }
 * 
 * // This is a really bad adding function, its purpose is to fail in this
 * // example.
 * #[allow(dead_code)]
 * fn bad_add(a: i32, b: i32) -> i32 {
 *     a - b
 * }
 * 
 * #[cfg(test)]
 * mod tests {
 *     // Note this useful idiom: importing names from outer (for mod tests) scope.
 *     use super::*;
 * 
 *     #[test]
 *     fn test_add() {
 *         assert_eq!(add(1, 2), 3);
 *     }
 * 
 *     #[test]
 *     fn test_bad_add() {
 *         // This assert would fire and test will fail.
 *         // Please note, that private functions can be tested too!
 *         assert_eq!(bad_add(1, 2), 3);
 *     }
 * }
 * ```
 *
 * Tests can be run with `cargo test`.
 *
 * ```
 * $ cargo test
 * 
 * running 2 tests
 * test tests::test_bad_add ... FAILED
 * test tests::test_add ... ok
 * 
 * failures:
 * 
 * ---- tests::test_bad_add stdout ----
 *         thread 'tests::test_bad_add' panicked at 'assertion failed: `(left == right)`
 *   left: `-1`,
 *  right: `3`', src/lib.rs:21:8
 * note: Run with `RUST_BACKTRACE=1` for a backtrace.
 * 
 * 
 * failures:
 *     tests::test_bad_add
 * 
 * test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
 * ```
 */
