use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// use Tab separated so we can easily split on a rarely used character
static TSV_TABLE_PATH: &'static str = "isodata.tsv";

type Alpha2 = String;
type Alpha3 = String;
type Name = String;
type NumCode = String;

type IsoData = Vec<(Alpha2, Alpha3, Name, NumCode)>;

fn read_table() -> IsoData {
    let reader = BufReader::new(File::open(TSV_TABLE_PATH).expect("Couldn't read isodata table"));

    reader.lines()
        .skip(1)
        .map(|line| {
            let line = line.expect("Problems reading line from ISO data CSV file");

            let columns: Vec<&str> = line.split("\t").collect();

            let alpha2 = columns[0].into();
            let alpha3 = columns[1].into();
            let name = columns[2].into();
            let num = columns[3].into();

            (alpha2, alpha3, name, num)
        })
        .collect()
}

fn write_enum(file: &mut BufWriter<File>, data: &IsoData) {
    writeln!(file, "#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]").unwrap();
    writeln!(file, "pub enum Country {{").unwrap();
    writeln!(file, "    Unspecified = 0,").unwrap();

    for &(ref alpha2, _, _, ref numcode) in data.iter() {
        writeln!(file, "    #[doc(hidden)]").unwrap();
        writeln!(file, "    {} = {},", alpha2, numcode).unwrap();
    }

    writeln!(file, "}}\n").unwrap();
}

fn write_country_code_search_table(file: &mut BufWriter<File>, data: &IsoData) {
    writeln!(file, "const COUNTRY_CODE_SEARCH_TABLE : &[(&str, Country)] = &[").unwrap();
    writeln!(file, "   (\"\",    Country::Unspecified),").unwrap();

    for &(ref alpha2, _, _, _) in data.iter() {
        writeln!(file, "    (\"{}\",  Country::{}),", alpha2, alpha2).unwrap()
    }

    writeln!(file, "];\n").unwrap();
}

fn write_enum_impl(file: &mut BufWriter<File>, data: &IsoData) {
    writeln!(file, "impl Country {{").unwrap();
    writeln!(file, "    pub fn name(self) -> &'static str {{").unwrap();
    writeln!(file, "        match self {{").unwrap();
    writeln!(file, "            Country::Unspecified => \"\",").unwrap();
    for &(ref alpha2, _, ref name, _) in data.iter() {
        writeln!(file, "            Country::{} => \"{}\",", alpha2, name).unwrap();
    }
    writeln!(file, "        }}").unwrap();
    writeln!(file, "    }}").unwrap();

    writeln!(file, "    pub fn from_name(s: &str) -> Option<Country> {{").unwrap();
    writeln!(file, "        Some(match s {{").unwrap();
    for &(ref alpha2, _, ref name, _) in data.iter() {
        writeln!(file, "            \"{}\" => Country::{},", name, alpha2).unwrap();
    }
    writeln!(file, "            _ => return None,").unwrap();
    writeln!(file, "        }})").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();
}

fn main() {
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("isodata.rs");

    let isodata = read_table();

    {
        let mut file = BufWriter::new(File::create(&out_path).expect("Couldn't write to output file"));
        write_enum(&mut file, &isodata);
        write_country_code_search_table(&mut file, &isodata);
        write_enum_impl(&mut file, &isodata);
    }
}
