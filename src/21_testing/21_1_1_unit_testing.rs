/*
 * Tests and ?
 *
 * None of the previous unit test examples had a return type. But in Rust 2018,
 * your unit tests can return `Result<{}>`, which lets you use `?` in them! This
 * can make them much more concise.
 *
 * ```
 * fn sqrt(number: f64) -> Result<f64, String> {
 *     if number >= 0.0 {
 *         Ok(number.powf(0.5))
 *     } else {
 *         Err("negative floats don't have square roots".to_owned())
 *     }
 * }
 * 
 * #[cfg(test)]
 * mod tests {
 *     use super::*;
 * 
 *     #[test]
 *     fn test_sqrt() -> Result<(), String> {
 *         let x = 4.0;
 *         assert_eq!(sqrt(x)?.powf(2.0), x);
 *         Ok(())
 *     }
 * }
 * ```
 *
 * See "The Edition Guide" for more details.
 */
