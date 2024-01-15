
/// Recursive ternary operation for terse if-else statements.
///
/// # Uses
/// 
/// Provides brief if-else statement syntax similar to other languages.
///
/// Reducing the lines-of-code (LOC) of chained if-else conditional
/// statements while improving the overall readability.
///
/// Other use-cases of `ifelse!` include providing natural recursion of chained
/// conditional branches.
/// 
/// If only a value is entered, that value is returned. This is necessary to
/// capture false statement remainders.
///
/// # Examples
///
/// ```
/// // A single if-else statement.
/// ifelse!(1 < 0, true, false);
/// 
/// // Chained if-else statements.
/// ifelse!(1 < 0, true, 1 > 0, true, false);
/// ifelse!(1 < 0, true, 0 > 1, true, false);
/// ifelse!(1 < 0, true, 0 > 1, false, 0 != 0, 1, 0);
/// 
/// // A single input returns itself.
/// assert!(ifelse!(false), false);
/// ```
macro_rules! ifelse {
    ($condition:expr , $true_expr:expr , $($opt:tt)*) => {
        if $condition {
            $true_expr
        } else {
            ifelse!($($opt)*)
        }
    };
    ($false_expr:expr) => ($false_expr);
}

#[cfg(test)]
mod tests {

    #[test]
    fn ternary_macro() {
        assert_eq!(ifelse!(1 < 0, true, false), false);
        assert_eq!(ifelse!(1 < 0, true, 1 > 0, true, false), true);
        assert_eq!(ifelse!(1 < 0, true, 1 < 0, true, false), false);
        assert_eq!(ifelse!(false), false);
    }

}
