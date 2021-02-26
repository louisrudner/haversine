#![feature(test)]
extern crate test;
use std::collections::HashMap;

fn haversine(unit: &str, from: (f32, f32), to: (f32, f32)) -> f32 {
    let earth = 6371.0088;

    let mut units = HashMap::new();
    units.insert("km", 1.00 as f32);
    units.insert("m", 1000.00 as f32);
    units.insert("mi", 0.621371192 as f32);
    units.insert("nmi", 0.539956803 as f32);
    units.insert("ft", 3280.839895013 as f32);
    units.insert("in", 39370.078740158 as f32);

    let r = earth * units.get(unit).unwrap();

    let a = from.0.to_radians();
    let b = from.1.to_radians();
    let c = to.0.to_radians();
    let d = to.1.to_radians();

    let lat_difference = a - c;
    let lon_difference = b - d;

    let hav = (lat_difference * 0.5).sin().powi(2) + a.cos() * c.cos() * (lon_difference * 0.5).sin().powi(2);

    2.0 * r * hav.sqrt().asin() as f32
}

#[cfg(test)]

mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(haversine("km", (41.507483, -99.436554), (38.504048, -98.315949)), 347.3288277897736);
    }

    #[bench]
    fn bench_haversine(b: &mut Bencher) {
        b.iter(|| haversine("km", (41.507483, -99.436554), (38.504048, -98.315949)))
    }
}
