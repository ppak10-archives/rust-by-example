/*
 * Running specific tests
 *
 * To run specific tests one may specify the test name to cargo test command.
 *
 * ```
 * $ cargo test test_any_panic
 * running 1 test
 * test tests::test_any_panic ... ok
 * 
 * test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
 * 
 *    Doc-tests tmp-test-should-panic
 * 
 * running 0 tests
 * 
 * test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 * ```
 *
 * To run multiple tests on may specify part of a test name that matches all the
 * tests that should be run.
 *
 * ```
 * $ cargo test panic
 * running 2 tests
 * test tests::test_any_panic ... ok
 * test tests::test_specific_panic ... ok
 * 
 * test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
 * 
 *    Doc-tests tmp-test-should-panic
 * 
 * running 0 tests
 * 
 * test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 * ```
 */
