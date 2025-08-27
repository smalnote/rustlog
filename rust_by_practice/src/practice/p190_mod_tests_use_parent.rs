#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Number {
    value: i32,
}
// impl From<i32> for Number
// get impl Into<Number> fro i32 implicitly
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::Number; // use parent mod

    #[test]
    fn implement_trait_from_get_into_trait_for_given_type() {
        let n = Number::from(42);
        let t: Number = 42_i32.into();

        assert_eq!(t, n);
        assert_eq!(t.value, 42);
        assert_eq!(n.value, 42);
    }
}
