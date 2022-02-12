/*
 * Integration testing
 *
 * Unit tests are testing on module in isolation at a time: they're small and
 * can test private code. Integration tests are external to your crate an use
 * only its public interface in the same way any other code would. Their purpose
 * is to test that many parts of your library work correctly together.
 *
 * Cargo looks for integration tests in `tests` directory next to `src`.
 *
 * Each Rust source file in the `tests` directory is compiled as a separate
 * crate. One way of sharing some code between integration tests is making a
 * module with public functions, importing and using it within tests.
 *
 * Modules with common code follow the ordinary modules rules, so its ok to
 * create common module as `tests/common/mod.rs`.
 */
