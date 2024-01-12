#[allow(unused_macros)]

// Chained ternary function generator (recursive)
macro_rules! ifelse {
    ($x:expr) => ($x);
    ($condition:expr , $true_expr:expr , $($opt:tt)*) => {
        if $condition {
            $true_expr
        } else {
            ifelse!($($opt)*)
        }
    };
    ($condition:expr => $true_expr:expr) => {
        if $condition {
            $true_expr
        } else {
            panic!("Failed to evaluate expression: {}", $condition)
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn ternary_macro() {
        assert_eq!(ifelse!(1 < 0, true, false), false);
        assert_eq!(ifelse!(1 < 0, true, 1 > 0, true, false), true);
        assert_eq!(ifelse!(1 < 0, true, 1 < 0, true, false), false);
    }

}
