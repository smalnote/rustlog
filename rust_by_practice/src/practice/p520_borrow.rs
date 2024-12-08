#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    trait Equivalent<K: ?Sized> {
        fn equivalent(&self, key: &K) -> bool;
    }

    impl<Q: ?Sized, K: ?Sized> Equivalent<K> for Q
    where
        Q: Eq,
        // K impl Borrow<Q>, means K.borrow() -> &Q
        K: Borrow<Q>,
    {
        #[inline]
        fn equivalent(&self, key: &K) -> bool {
            let lhs: &Q = self;
            let rhs: &Q = key.borrow(); // K.borrow() -> &Q
            // Eq is a super trait of PartialEq
            // Q: PartialEq::eq take types &Q
            PartialEq::eq(lhs, rhs)
        }
    }

    #[test]
    fn test_borrow_equivalent() {
        let lhs = "hello";
        let rhs = String::from("hello");

        fn check_eq<Q: Eq>(_: Q) {}
        check_eq(lhs);

        // String: Borrow<str>
        // str: Eq
        // PartialEq::eq(&str, &Str);
        assert!(lhs.equivalent(&rhs));
    }
}
