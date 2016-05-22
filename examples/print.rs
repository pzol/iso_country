extern crate iso_country as country;

pub fn main() {
    // use std::ascii::AsciiExt;
    let mut countries = country::data::all();
    countries.sort_by_key(|country| country.alpha2);

    for country in countries {
        // let num : usize = country.num.parse().unwrap();
        // println!("(\"{}\",  Country::{}),", country.alpha2, country.alpha2);
        println!("{} => \"{}\",", country.alpha2, country.name);
    }
}
