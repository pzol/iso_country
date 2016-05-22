#![feature(test)]
extern crate test;
use test::Bencher;

extern crate iso_country as country;
use country::Country;

#[bench]
pub fn parse(b : &mut Bencher) {

    b.iter(|| {
        let _c : Country = "PL".parse().unwrap();
    });
}


#[bench]
pub fn parse_name(b : &mut Bencher) {

    b.iter(|| {
        let c : Country = "PL".parse().unwrap();
        let _s = c.name();
    });
}
