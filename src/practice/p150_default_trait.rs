#[cfg(test)]
mod tests {

    // Default trait
    #[test]
    fn implement_default_trait_of_custom_type() {
        #[derive(Debug, PartialEq)]
        struct Point<T> {
            x: T,
            y: T,
        }

        impl Default for Point<i32> {
            fn default() -> Point<i32> {
                Point {
                    x: 42_i32,
                    y: 42_i32,
                }
            }
        }

        impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
            }
        }

        let point = Point { x: 42, y: 42 };

        // Default::default() inference type from `Point<i32>`
        let default_point_1: Point<i32> = Default::default();
        // Point::default() inference generic type i32 from `Point<i32>`
        let default_point_2: Point<i32> = Point::default();
        // specify type Point<i32>'s default function
        type PointI32 = Point<i32>;
        let default_point_3 = PointI32::default();
        let default_point_4 = Point::<i32>::default();
        assert_eq!(point, default_point_1);
        assert_eq!(point, default_point_2);
        assert_eq!(point, default_point_3);
        assert_eq!(point, default_point_4);
        println!(
            "point = {}, default 1 = {}, default 2 = {}, default 3 = {}, default 4 = {}",
            point, default_point_1, default_point_2, default_point_3, default_point_4
        );
    }

    // Implements trait default of generic type
    #[allow(dead_code)]
    #[test]
    fn implement_default_trait_of_generic_type() {
        use std::hash::BuildHasherDefault;
        use twox_hash::XxHash64;
        struct Pump<H> {
            h: H,
        }

        /// Implements Default trait for Pump<H> where H bound to Default trait.
        impl<H: Default> Default for Pump<H> {
            fn default() -> Self {
                Pump { h: H::default() }
            }
        }

        let _p: Pump<BuildHasherDefault<XxHash64>> = Default::default(); // totally type infer
        let _p: Pump<BuildHasherDefault<XxHash64>> = Pump::default(); // partially type infer
        let _p = Pump::<BuildHasherDefault<XxHash64>>::default(); // totally specify type by turbofish
        type BuildXxHash64 = BuildHasherDefault<XxHash64>; // type alias for shorthand
        let _p = Pump::<BuildXxHash64>::default();

        impl<H> From<H> for Pump<H> {
            fn from(h: H) -> Pump<H> {
                Pump { h }
            }
        }
        // Inference From::from() of type Pump<&str>
        let _p: Pump<&str> = From::from("trait");
        let _p = Pump::<&str>::from("type");
        let _p = <Pump<&str> as From<&str>>::from("from");
        let _p = <Pump<&str>>::from("from");
    }
}
