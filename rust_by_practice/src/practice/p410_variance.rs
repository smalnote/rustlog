/// Subtypeing and Variance
///
/// Subtype: if a type `Sub` can be coerced to another type `Super`, donated as `Sub` <: `Super`
/// Example: 'static <: 'a, meaning lifetime 'static can be use as a shorter lifetime 'a
///
/// Variant: F is covariant if, F<Sub> is a subtype of F<Super>
/// Example: if 'a <: 'b, then &'a T <: &'b T, so we can say &'a T is covariant over 'a
/// Note: Treats `F` as a function, F('a) = &'a T, F('b) = &'b T
///
/// Contravariant: F is contravariant if, F<Super> is subtype of F<Sub>
/// Example: fn(P), for P of &'a T and &'b T, where 'a <: 'b, fn(&'b T) <: fn(&'a T)
/// Explanation: fn(&'b T) accept a shorter lifetime 'b, can be use to accept a longer lifetime 'a

#[cfg(test)]
mod tests {

    #[test]
    fn test_lifetime_subtype() {
        {
            let longer = "longer";
            {
                let mut shorter = "shorter";
                assert_eq!(shorter, "shorter");
                shorter = longer;
                assert_eq!(shorter, "longer");
            }
        }
    }

    #[test]
    fn test_ref_lifetime_covariant() {
        fn debug<'a>(_: &'a str, _: &'a str) {}

        {
            let longer = "longer";
            {
                let shorter = "shorter";
                debug(shorter, longer); // 'longer <: 'shorter
            }
        }
    }

    #[test]
    fn test_contravariant_fn_t() {
        fn take_fn_static(_: fn(_: &'static str)) {}

        fn fn_static(_: &'static str) {}
        take_fn_static(fn_static);

        // &'static str <: &'a str
        // fn<'a> <: fn<'static>
        fn fn_tick_a<'a>(_: &'a str) {}
        take_fn_static(fn_tick_a);
    }
}
