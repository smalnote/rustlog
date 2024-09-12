#[cfg(test)]
mod tests {

    #[test]
    fn custom_reciprocal_trait() {
        trait Come<T>
        where
            Self: Sized,
        {
            fn come(t: T) -> Self;
        }

        trait Go<T>
        where
            Self: Sized,
        {
            fn go(self) -> T;
        }

        // Implements Go<U> for T (T->U), for all U that implements From(T) (T->U)
        impl<T, U> Go<U> for T
        where
            U: Come<T>,
        {
            fn go(self) -> U {
                U::come(self)
            }
        }

        #[derive(Debug, PartialEq)]
        struct Complexity {
            real: f32,
            virt: f32,
        }

        struct Tuplexity(f32, f32);

        impl Come<Tuplexity> for Complexity {
            fn come(Tuplexity(real, virt): Tuplexity) -> Self {
                Self { real, virt }
            }
        }

        // use the Go<Complexity> for Tuplexity implements by generic above
        let c: Complexity = Tuplexity(2.2, 3.4).go();

        assert_eq!(
            c,
            Complexity {
                real: 2.2,
                virt: 3.4
            }
        );
    }
}
