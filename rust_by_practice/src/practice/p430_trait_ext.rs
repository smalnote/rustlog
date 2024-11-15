#[cfg(test)]
mod tests {
    use std::{collections::HashSet, hash::Hash};

    use axum::http::Request;
    trait HttpRequestExt {
        fn get_jwt(&self) -> Option<String>;
    }

    impl<T> HttpRequestExt for Request<T> {
        fn get_jwt(&self) -> Option<String> {
            self.headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.strip_prefix("Bearer "))
                .map(|value| value.to_string())
        }
    }

    #[test]
    fn test_http_request_ext() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let req = Request::head("http://www.rust-lang.org")
            .header("Authorization", "Bearer ".to_string() + token)
            .body(());
        assert_eq!(req.unwrap().get_jwt().unwrap(), token);
    }

    // super trait
    trait IteratorExt: Iterator {
        fn unique(self) -> UniqueIterator<Self>
        where
            Self: Sized,
            Self::Item: Eq + Hash + Clone,
        {
            UniqueIterator {
                iter: self,
                seen: HashSet::new(),
            }
        }
    }

    // generic implementation
    // for I, where I is a generic type rather than a concrete type
    impl<I: Iterator> IteratorExt for I {}

    struct UniqueIterator<I>
    where
        I: Iterator,
        I::Item: Eq + Hash + Clone,
    {
        iter: I,
        seen: HashSet<I::Item>,
    }

    impl<I> Iterator for UniqueIterator<I>
    where
        I: Iterator,
        I::Item: Eq + Hash + Clone,
    {
        type Item = I::Item;
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.find(|item| self.seen.insert(item.clone()))
        }
    }

    #[test]
    fn test_unique_iterator() {
        let numbers = vec![1, 2, 3, 4, 2, 1, 0, -1];
        let unique_numbers = numbers.into_iter().unique().collect::<Vec<i32>>();
        assert_eq!(unique_numbers, vec![1, 2, 3, 4, 0, -1]);
    }
}
