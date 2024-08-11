#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use std::hash::{BuildHasherDefault, DefaultHasher, Hash, Hasher};
    use twox_hash::XxHash64;

    // HashMap<&str, i32>
    #[test]
    fn use_hash_map() {
        let mut scores: HashMap<&str, i32> = HashMap::new();
        scores.insert("Sunface", 98);
        scores.insert("Daniel", 95);
        scores.insert("Ashley", 69);
        scores.insert("Katie", 58);

        let score = scores.get("Katie"); // access by method `get`
        assert_eq!(score, Some(&58_i32));

        if scores.contains_key("Ashley") {
            let score = scores["Ashley"]; // access by brackets
            assert_eq!(score, 69);
            scores.remove("Ashley");
        }

        assert_eq!(scores.len(), 3);

        for (name, score) in scores {
            println!("The score of {} is {}", name, score);
        }
    }

    // array of tuples to HashMap
    #[test]
    fn convert_array_of_tuples_to_hash_map() {
        let teams: [(&str, i32); 3] = [
            ("Chinese Team", 100),
            ("American Team", 10),
            ("France Team", 50),
        ];

        let m1 = HashMap::<&str, i32>::from(teams);
        let m2: HashMap<&str, i32> = teams.into_iter().collect();

        assert_eq!(m1, m2);
    }

    // HashMap entry
    #[test]
    fn use_hash_map_entry() {
        let mut player_stats = HashMap::new();

        player_stats.entry("health").or_insert(100);
        assert_eq!(player_stats["health"], 100);

        // insert with a pointer to a function
        fn random_stat() -> u8 {
            42
        }
        player_stats.entry("health").or_insert_with(random_stat);
        assert_eq!(player_stats["health"], 100);

        // .entry().or_insert() return a mutable reference to the value
        let health: &mut u8 = player_stats.entry("health").or_insert(50);
        assert_eq!(*health, 100);
        *health -= 50;
        assert_eq!(player_stats["health"], 50);
    }

    /*
     * Requirements of HashMap key
     * Any type taht implements the Eq and Hash traits can be a key in HashMap. This include:
     *   - bool
     *   - int, uint, and all variations thereof
     *   - String and &str (tips: you can have a HashMap keyed by String and call .get() with an &str)
     *
     * Note: f32 and f64 do not implement Hash, likely because floating-point precesion erros would make
     * using the as hashmap keys horribly error-prone.
     *
     * Note: All collection classes implement Eq and Hash if there contained type also respectively
     * implements Eq and Hash.
     * For example, Vec<T> implements Hash if the contained type T implements Hash.
     */
    #[test]
    fn implement_custom_hash_map_key_type() {
        #[derive(Debug, Hash, PartialEq, Eq)]
        struct Viking {
            name: String,
            country: String,
        }

        impl Viking {
            fn new(name: &str, country: &str) -> Self {
                Viking {
                    name: name.to_string(),
                    country: country.to_string(),
                }
            }
        }

        let vikings = HashMap::from([
            (Viking::new("Olaf", "Norway"), 25),
            (Viking::new("Olaf", "Denmark"), 4),
            (Viking::new("Harald", "Denmark"), 12),
        ]);

        fn hash<T: Hash>(t: &T) -> u64 {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            hasher.finish()
        }

        for (viking, age) in vikings {
            println!(
                "Viking: {:?}, age: {}, hash: {}",
                viking,
                age,
                hash(&viking)
            );
        }
    }

    // HashMap capacity
    #[test]
    fn test_hash_map_capacity() {
        let mut m = HashMap::<i32, i32>::with_capacity(5);

        let mut cap = m.capacity();

        println!("len = {}, cap = {}", m.len(), m.capacity());
        for i in 0..70 {
            m.insert(i, i);
            if m.capacity() != cap {
                cap = m.capacity();
                println!("len = {}, cap = {}", m.len(), m.capacity());
            }
        }
        m.shrink_to(50);
        println!(
            "len = {}, shrink_to = {}, cap = {}",
            m.len(),
            50,
            m.capacity()
        );
    }

    /*
     * Ownership of HashMap
     * For types that implement the Copy trait, like i32, the values are copied into HashMap.
     * For owned values like String, the values will be moved and HashMap will be the owner of
     * those values.
     */
    #[allow(unused_mut)]
    #[test]
    fn test_hash_map_elements_ownership() {
        // Values of type i32 are copied into HashMap
        let mut numbers = HashMap::<i32, i32>::new();
        let mut n = 42;
        numbers.insert(n, n);
        n *= 2; // i32 still usable after insert
        for (k, v) in numbers {
            println!("n = {}, k = {}, v = {}", n, k, v,);
            assert_ne!(n, v);
        }

        // Values of type String are moved into HashMap
        let mut strings = HashMap::<String, String>::new();
        let mut k = "country".to_string();
        let mut v = "China".to_string();
        strings.insert(k, v);
        // error: value borrowed after move
        // k.push('1');
        // v.push('1');

        // Unlike String, &str is copied into HashMap
        let mut strs = HashMap::<&str, &str>::new();
        let mut sk = "name";
        let mut sv = "Olaf";
        strs.insert(sk, sv);
        sk = "age";
        sv = "28";
        for (k, v) in strs {
            println!("sk = {}, sv = {}, k = {}, v = {}", sk, sv, k, v);
            assert_ne!(sk, k);
            assert_ne!(sv, v);
        }
    }

    // Third-party Hash libs
    #[test]
    fn use_third_party_hash_algorithm() {
        let mut map: HashMap<&str, &str, BuildHasherDefault<XxHash64>> = Default::default();
        map.insert("version", "alpha");
        assert_eq!(map.get("version"), Some(&"alpha"));

        // Default::default() <==> HashMap::default()
        // HashMap implements trait Default
        let mut map: HashMap<&str, &str, BuildHasherDefault<XxHash64>> = HashMap::default();
        map.insert("version", "alpha");
        assert_eq!(map.get("version"), Some(&"alpha"));

        // HashMap::default() <==> HashMap::with_hasher(BuildHasherDefault::default())
        // HashMap implements trait Default: HashMap::with_hasher(Default::default())
        // Where the `Default::default()` is inference to BuildHasherDefault<XxHash64>'s Default trait implementation
        // Where inference to XxHash64's Default trait implementation
        let mut map: HashMap<&str, &str, BuildHasherDefault<XxHash64>> =
            HashMap::with_hasher(BuildHasherDefault::default());
        map.insert("version", "alpha");
        assert_eq!(map.get("version"), Some(&"alpha"));

        // Creates hasher with generic(type XxHash64) associated function(default).
        // Use ::<type> to specify type of generic associated function.
        let hasher = BuildHasherDefault::<XxHash64>::default();
        let mut map = HashMap::<u32, i32, BuildHasherDefault<XxHash64>>::with_hasher(hasher);
        map.insert(42, 42);
        assert_eq!(map.get(&42), Some(&42));

        // Defines type alias for short.
        type BuildXxHash64 = BuildHasherDefault<XxHash64>;
        // BuildXxHash64::default() <==> BuildHasherDefault::<XxHash64>>::default()
        let mut map = HashMap::<u32, i32, BuildXxHash64>::with_hasher(BuildXxHash64::default());
        map.insert(42, 42);
        assert_eq!(map.get(&42), Some(&42));
    }
}
