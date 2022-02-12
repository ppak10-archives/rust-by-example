/*
 * Ignoring tests
 *
 * Tests can be marked with the `#[ignore]` attribute to exclude some tests. Or
 * to run them with command `cargo test -- --ignored`
 *
 * ```
 * #![allow(unused)]
 * fn main() {
 * pub fn add(a: i32, b: i32) -> i32 {
 *     a + b
 * }
 * 
 * #[cfg(test)]
 * mod tests {
 *     use super::*;
 * 
 *     #[test]
 *     fn test_add() {
 *         assert_eq!(add(2, 2), 4);
 *     }
 * 
 *     #[test]
 *     fn test_add_hundred() {
 *         assert_eq!(add(100, 2), 102);
 *         assert_eq!(add(2, 100), 102);
 *     }
 * 
 *     #[test]
 *     #[ignore]
 *     fn ignored_test() {
 *         assert_eq!(add(0, 0), 0);
 *     }
 * }
 * ```
 *
 * ```
 * $ cargo test
 * running 3 tests
 * test tests::ignored_test ... ignored
 * test tests::test_add ... ok
 * test tests::test_add_hundred ... ok
 * 
 * test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
 * 
 *    Doc-tests tmp-ignore
 * 
 * running 0 tests
 * 
 * test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 * 
 * $ cargo test -- --ignored
 * running 1 test
 * test tests::ignored_test ... ok
 * 
 * test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 * 
 *    Doc-tests tmp-ignore
 * 
 * running 0 tests
 * 
 * test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 * ```
 */
