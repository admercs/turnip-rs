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
/// use turnip::ifelse;
/// 
/// // A single if-else statement.
/// ifelse!(1 < 0, true, false);
///
/// // Chained if-else statements.
/// ifelse!(1 < 0, true, 1 > 0, true, false);
/// ifelse!(1 < 0, true, 0 > 1, true, false);
/// ifelse!(1 < 0, 0, 0 > 1, 1, 0 != 0, 2, 3);
///
/// // A single input returns itself.
/// ifelse!(false);
/// ```
#[macro_export]
macro_rules! ifelse {
    ($cond:expr , $true_expr:expr , $($arg:tt)*) => {
        if $cond {
            $true_expr
        } else {
            ifelse!($($arg)*)
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
