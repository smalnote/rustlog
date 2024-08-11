#[cfg(test)]
mod tests {
    // Vectors are resizable arrays, they can grow or shrink at runtime.
    // vec![] or vec!() is marco for Vec<T>
    #[test]
    fn create_vectors() {
        fn is_vec_u8(_: &Vec<u8>) {}

        let arr: [u8; 3] = [1, 2, 3];

        let v: Vec<u8> = Vec::from(arr);
        is_vec_u8(&v);

        let v: Vec<u8> = vec![1, 2, 3];
        is_vec_u8(&v);

        let v: Vec<u8> = vec![1, 2, 3];
        is_vec_u8(&v);

        let mut v1: Vec<u8> = Vec::new();
        for v in arr {
            v1.push(v);
        }

        assert_eq!(v, v1);
    }

    // vector, extend with other vectors
    #[test]
    fn change_vector_size() {
        let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
        v1.pop();
        v1.push(3);

        let mut v2: Vec<i32> = Vec::new();
        v2.extend(&v1);

        assert_eq!(v1, v2);
    }

    // array or slice -> vector
    #[test]
    fn convert_array_or_slice_to_vector() {
        let arr = [1, 2, 3];
        let slice = &arr;
        let v1 = Vec::from(arr);
        let v2 = Vec::from(slice);
        let mut v3: Vec<i32> = arr.to_vec();
        let v4: Vec<i32> = arr.into();

        assert_eq!(v1, v2);
        assert_eq!(v1, v3);
        assert_eq!(v1, v4);

        // vector is a copy of arrry
        v3[2] = 6;
        assert_ne!(arr[2], v3[2]);
    }

    // String -> Vec
    #[test]
    fn convert_string_to_vector() {
        let s = "hello".to_string();
        let v1: Vec<u8> = Vec::from(&s[..]);

        let v2: Vec<u8> = s.into_bytes();
        assert_eq!(v1, v2);

        // impl <'_> From<&'_ str> for Vec
        let s: &str = "hello";
        let v3: Vec<u8> = Vec::from(s);
        assert_eq!(v2, v3);
    }

    // iterators can be collected into vectors
    #[test]
    fn convert_iterator_to_vector() {
        let v4: Vec<i32> = [0; 10].into_iter().collect();
        assert_eq!(v4, vec![0; 10]);
    }

    // implement vector trait `From` for custome type
    #[test]
    fn implement_trait_from_of_custom_type_for_vector() {
        struct Point<T> {
            x: T,
            y: T,
            z: T,
        }

        impl<T: Copy> From<&Point<T>> for Vec<T> {
            fn from(value: &Point<T>) -> Self {
                vec![value.x, value.y, value.z]
            }
        }

        let p = Point { x: 1, y: 2, z: 3 };
        let v = Vec::from(&p);
        assert_eq!(v, vec![1, 2, 3]);
    }

    // vector indexing
    #[test]
    fn vector_indexing() {
        let mut v = vec![1, 2, 3];

        for i in 0..3 {
            assert_eq!(v[i], i + 1);
        }

        for i in 0..5 {
            match v.get(i) {
                Some(e) => v[i] = e + 1,
                None => v.push(i + 2),
            }
        }

        assert_eq!(v, vec![2, 3, 4, 5, 6]);
    }

    // vector slice(&[]) is readonly, like &str fro String
    #[test]
    fn vector_slice() {
        let mut v = vec![1, 2, 3];

        let slice1 = &v[..];
        let slice2 = &v[0..v.len()];
        assert_eq!(slice1, slice2);

        // Slices are readonly
        // Note: slice and &Vec are different
        let vec_ref: &mut Vec<i32> = &mut v;
        vec_ref.push(4);
        let slice3 = &mut v[0..4];
        slice3[3] = 5;

        assert_eq!(slice3, &[1, 2, 3, 5]);
    }

    // vector capacity
    #[test]
    fn vector_capacity() {
        let mut vec = Vec::<usize>::with_capacity(10);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);

        for i in 0..10 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);

        // extend vec, this may make the vector reallocate
        vec.push(11);
        assert_eq!(vec.len(), 11);
        assert!(vec.capacity() >= 11);
    }

    // use enum or trait to store distinct types in vector
    #[test]
    fn vector_with_distinct_element_types() {
        #[derive(Debug, PartialEq)]
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let v: Vec<IpAddr> = vec![
            IpAddr::V4("127.0.0.1".to_string()),
            IpAddr::V6("::1".to_string()),
        ];

        assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
        assert_eq!(v[1], IpAddr::V6("::1".to_string()));

        impl std::fmt::Display for IpAddr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    IpAddr::V4(ip) => write!(f, "IPv4({})", ip),
                    IpAddr::V6(ip) => write!(f, "IPv6({})", ip),
                }
            }
        }

        // Vec<Box<dyn T>> or Vec<&dyn T>
        let formatables: Vec<Box<dyn std::fmt::Display>> = vec![
            Box::new(1),
            Box::new(2_u8),
            Box::new(3.0_f32),
            Box::new(4_usize),
            Box::new("hello"),
            Box::new('中'),
            Box::new(true),
            Box::new(IpAddr::V4("127.0.0.1".to_string())),
            Box::new(IpAddr::V6("::1".to_string())),
        ];
        for v in formatables {
            println!("{}", v);
        }
    }
}
