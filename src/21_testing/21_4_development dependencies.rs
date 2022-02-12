/*
 * Development dependencies
 *
 * Sometimes there is a need to have dependencies for tests (or examples, or 
 * benchmarks) only. Such dependencies are added to `Cargo.toml` in the
 * `[dev-dependencies]` secion. These dependencies are not propagated to other
 * packages which depend on this package.
 *
 * One such example is `pretty_assertions`, which extends standard `assert_eq!`
 * and `assert_ne!` macros, to provide colorful diff.
 */
