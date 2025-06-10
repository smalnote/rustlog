#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        collections::{LinkedList, VecDeque},
        sync::RwLock,
    };

    use async_std::task;

    #[test]
    fn async_closure() {
        use std::future::ready;
        let mut vec = vec![];

        // async block: `|| async {}`
        // cannot capture outer variables
        let _async_block = || async {
            // captured variable cannot escape `FnMut` closure body
            // vec.push(ready(String::from("Hello")).await);
            println!("{:?}", vec);
        };

        // async closure: `async || {}`
        let mut closure = async || {
            vec.push(ready(String::from("Hello")).await);
        };

        task::block_on(closure());
        task::block_on(closure());
        task::block_on(closure());

        println!("{:?}", vec);
    }

    #[test]
    fn asnyc_closure_trait() {
        async fn take_async_closure(mut c: impl for<'a> AsyncFnMut(&'a Vec<String>)) {
            let v = vec![];
            c(&v).await;
        }

        async fn take_async_generic<F>(_: impl for<'a> Fn(&'a Vec<String>) -> F)
        where
            F: Future<Output = ()>,
        {
        }

        let async_block = |_: &Vec<String>| async {};
        let async_closure = async |_: &Vec<String>| {};

        task::block_on(take_async_closure(async_block));
        task::block_on(take_async_closure(async_closure));

        task::block_on(take_async_generic(async_block));
        // mismatched types
        // task::block_on(take_async_generic(async_closure));
    }

    #[test]
    fn tuple_iterator() {
        let (squares, cubes, tesseracts): (Vec<_>, VecDeque<_>, LinkedList<_>) =
            (0i32..10).map(|i| (i * i, i.pow(3), i.pow(4))).collect();

        println!("{squares:?}");
        println!("{cubes:?}");
        println!("{tesseracts:?}");
    }

    #[test]
    fn if_let_borrow_checker() {
        fn get_or_default(value: &RwLock<Option<bool>>, default_value: bool) -> bool {
            if let Some(v) = *value.read().unwrap() {
                v
                // *value.read() borrow dropped here
                // value.write in else is possible
            } else {
                let mut lock = value.write().unwrap();
                if lock.is_none() {
                    *lock = Some(default_value)
                }
                default_value
            }
        }

        let lock = RwLock::<Option<bool>>::new(None);

        let value = get_or_default(&lock, true);
        assert!(value);
    }

    #[test]
    fn temporary_borrow() {
        fn temporary_borrow_len() -> usize {
            let c = RefCell::new("Hello");

            c.borrow().len()
        }

        assert_eq!(temporary_borrow_len(), 5);
    }

    #[test]
    fn unsafe_extern_c_block() {
        unsafe extern "C" {
            pub safe fn sqrt(x: f64) -> f64;

            pub unsafe fn strlen(p: *const std::ffi::c_char) -> usize;
        }

        assert_eq!(sqrt(9.0), 3.0);
        let message = String::from("Hello");
        assert_eq!(unsafe { strlen(message.as_ptr() as *const i8) }, 5);
    }
}
