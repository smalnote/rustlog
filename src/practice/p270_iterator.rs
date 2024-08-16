#[cfg(test)]
mod tests {
    /*
     * Iterator:
     * - Allows to perform task on sequence of items in turn
     * - Iterators are `lazy`, meaning no effect until methods are called that
     *   consume the iterator to use it up
     * - All iterators implement trait `Iterator` which has method `next()`,
     *   which gets called automatically when traversing over some data
     * - Some methods consume iterator while others produce a new iterator from
     *   the provided iterator.
     */

    #[test]
    fn iterate_vector() {
        let v = vec![1, 2, 3];

        // for-in get iterator of v by `v.into_iter()`
        // equivalent to: for x in v.into_iter()
        // According to Vec::into_iter(), v is moved after calling it, so v cannot be used anymore.
        for x in v {
            println!("x = {}", x);
        }

        let v = vec![4, 5, 6];
        for x in v.into_iter() {
            println!("x = {}", x);
        }
    }

    #[test]
    fn range_notion_as_iterator() {
        use std::ops::{Range, RangeInclusive};

        let mut v = Vec::new();

        for i in 0..100 {
            v.push(i);
        }

        assert_eq!(v.len(), 100);

        let range = Range {
            start: 100,
            end: 150,
        };
        for i in range {
            v.push(i)
        }

        assert_eq!(v.len(), 150);

        assert_eq!(
            100..150,
            Range {
                start: 100,
                end: 150
            }
        );

        assert_eq!(100..=150, RangeInclusive::new(100, 150))
    }

    #[test]
    fn manually_use_trait_iterator() {
        let v = vec![1, 2];

        let mut v1 = v.into_iter();

        assert_eq!(v1.next(), Some(1));
        assert_eq!(v1.next(), Some(2));
        assert_eq!(v1.next(), None);
    }

    #[test]
    fn into_iter_and_iter_and_iter_mut() {
        let mut v = vec![1, 2, 3];

        // iter() uses immutable reference of v
        for (i, x) in v.iter().enumerate() {
            assert_eq!(x, &(i as i32 + 1))
        }

        // iter_mut() uses mutable reference of v
        for x in v.iter_mut() {
            *x = *x * 2;
        }

        // into_iter() takes the ownership of v
        for (i, x) in v.into_iter().enumerate() {
            assert_eq!(x, (i as i32 + 1) * 2);
        }
    }

    #[test]
    fn custom_iterator_fibonacci() {
        let fib = Fibonacci::new(10);

        for f in fib {
            println!("{}", f);
        }
    }

    struct Fibonacci {
        count: u32,
        current: u32,
        first: u32,
        second: u32,
    }

    impl Fibonacci {
        fn new(count: u32) -> Fibonacci {
            assert!(count > 0);
            Fibonacci {
                count,
                current: 0,
                first: 0,
                second: 1,
            }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current >= self.count {
                return None;
            }
            self.current += 1;
            let next = self.first + self.second;
            self.first = self.second;
            self.second = next;
            Some(self.first)
        }
    }

    #[test]
    fn consume_iterator() {
        // trait Iterator has a set of default method that consume iterator.

        // consumer: sum()
        let fib5 = Fibonacci::new(5);
        let sum = fib5.sum::<u32>();
        assert_eq!(sum, 12);

        // consumer: collect()
        let fib10 = Fibonacci::new(10);
        let fib10 = fib10.collect::<Vec<u32>>();
        assert_eq!(fib10, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);

        // adaptor: map()
        let fib3 = Fibonacci::new(3);
        let multiply_10 = |x| x * 10;
        let fib3 = fib3.map(multiply_10).collect::<Vec<u32>>();
        assert_eq!(fib3, vec![10, 10, 20]);

        use std::collections::HashMap;
        let fib20 = Fibonacci::new(20);
        let fib20: HashMap<u32, u32> = fib20.map(|x| ((x, x))).collect();
        for (i, v) in fib20 {
            println!("key = {}, value = {}", i, v);
        }
    }
}
