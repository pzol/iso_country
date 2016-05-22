pub struct CountryCode<'a> {
    pub alpha2: &'a str,
    pub alpha3: &'a str,
    pub name: &'a str,
    pub num: &'a str,
}

pub fn all<'a>() -> Vec<CountryCode<'a>> {
    let mut codes: Vec<CountryCode> = vec![];

    // Begin
    codes.push(CountryCode {
        alpha2: "AF",
        alpha3: "AFG",
        name: "Afghanistan",
        num: "004",
    });
    codes.push(CountryCode {
        alpha2: "AL",
        alpha3: "ALB",
        name: "Albania",
        num: "008",
    });
    codes.push(CountryCode {
        alpha2: "AQ",
        alpha3: "ATA",
        name: "Antarctica",
        num: "010",
    });
    codes.push(CountryCode {
        alpha2: "DZ",
        alpha3: "DZA",
        name: "Algeria",
        num: "012",
    });
    codes.push(CountryCode {
        alpha2: "AS",
        alpha3: "ASM",
        name: "American Samoa",
        num: "016",
    });
    codes.push(CountryCode {
        alpha2: "AD",
        alpha3: "AND",
        name: "Andorra",
        num: "020",
    });
    codes.push(CountryCode {
        alpha2: "AO",
        alpha3: "AGO",
        name: "Angola",
        num: "024",
    });
    codes.push(CountryCode {
        alpha2: "AG",
        alpha3: "ATG",
        name: "Antigua and Barbuda",
        num: "028",
    });
    codes.push(CountryCode {
        alpha2: "AZ",
        alpha3: "AZE",
        name: "Azerbaijan",
        num: "031",
    });
    codes.push(CountryCode {
        alpha2: "AR",
        alpha3: "ARG",
        name: "Argentina",
        num: "032",
    });
    codes.push(CountryCode {
        alpha2: "AU",
        alpha3: "AUS",
        name: "Australia",
        num: "036",
    });
    codes.push(CountryCode {
        alpha2: "AT",
        alpha3: "AUT",
        name: "Austria",
        num: "040",
    });
    codes.push(CountryCode {
        alpha2: "BS",
        alpha3: "BHS",
        name: "Bahamas",
        num: "044",
    });
    codes.push(CountryCode {
        alpha2: "BH",
        alpha3: "BHR",
        name: "Bahrain",
        num: "048",
    });
    codes.push(CountryCode {
        alpha2: "BD",
        alpha3: "BGD",
        name: "Bangladesh",
        num: "050",
    });
    codes.push(CountryCode {
        alpha2: "AM",
        alpha3: "ARM",
        name: "Armenia",
        num: "051",
    });
    codes.push(CountryCode {
        alpha2: "BB",
        alpha3: "BRB",
        name: "Barbados",
        num: "052",
    });
    codes.push(CountryCode {
        alpha2: "BE",
        alpha3: "BEL",
        name: "Belgium",
        num: "056",
    });
    codes.push(CountryCode {
        alpha2: "BM",
        alpha3: "BMU",
        name: "Bermuda",
        num: "060",
    });
    codes.push(CountryCode {
        alpha2: "BT",
        alpha3: "BTN",
        name: "Bhutan",
        num: "064",
    });
    codes.push(CountryCode {
        alpha2: "BO",
        alpha3: "BOL",
        name: "Bolivia (Plurinational State of)",
        num: "068",
    });
    codes.push(CountryCode {
        alpha2: "BA",
        alpha3: "BIH",
        name: "Bosnia and Herzegovina",
        num: "070",
    });
    codes.push(CountryCode {
        alpha2: "BW",
        alpha3: "BWA",
        name: "Botswana",
        num: "072",
    });
    codes.push(CountryCode {
        alpha2: "BV",
        alpha3: "BVT",
        name: "Bouvet Island",
        num: "074",
    });
    codes.push(CountryCode {
        alpha2: "BR",
        alpha3: "BRA",
        name: "Brazil",
        num: "076",
    });
    codes.push(CountryCode {
        alpha2: "BZ",
        alpha3: "BLZ",
        name: "Belize",
        num: "084",
    });
    codes.push(CountryCode {
        alpha2: "IO",
        alpha3: "IOT",
        name: "British Indian Ocean Territory",
        num: "086",
    });
    codes.push(CountryCode {
        alpha2: "SB",
        alpha3: "SLB",
        name: "Solomon Islands",
        num: "090",
    });
    codes.push(CountryCode {
        alpha2: "VG",
        alpha3: "VGB",
        name: "Virgin Islands (British)",
        num: "092",
    });
    codes.push(CountryCode {
        alpha2: "BN",
        alpha3: "BRN",
        name: "Brunei Darussalam",
        num: "096",
    });
    codes.push(CountryCode {
        alpha2: "BG",
        alpha3: "BGR",
        name: "Bulgaria",
        num: "100",
    });
    codes.push(CountryCode {
        alpha2: "MM",
        alpha3: "MMR",
        name: "Myanmar",
        num: "104",
    });
    codes.push(CountryCode {
        alpha2: "BI",
        alpha3: "BDI",
        name: "Burundi",
        num: "108",
    });
    codes.push(CountryCode {
        alpha2: "BY",
        alpha3: "BLR",
        name: "Belarus",
        num: "112",
    });
    codes.push(CountryCode {
        alpha2: "KH",
        alpha3: "KHM",
        name: "Cambodia",
        num: "116",
    });
    codes.push(CountryCode {
        alpha2: "CM",
        alpha3: "CMR",
        name: "Cameroon",
        num: "120",
    });
    codes.push(CountryCode {
        alpha2: "CA",
        alpha3: "CAN",
        name: "Canada",
        num: "124",
    });
    codes.push(CountryCode {
        alpha2: "CV",
        alpha3: "CPV",
        name: "Cabo Verde",
        num: "132",
    });
    codes.push(CountryCode {
        alpha2: "KY",
        alpha3: "CYM",
        name: "Cayman Islands",
        num: "136",
    });
    codes.push(CountryCode {
        alpha2: "CF",
        alpha3: "CAF",
        name: "Central African Republic",
        num: "140",
    });
    codes.push(CountryCode {
        alpha2: "LK",
        alpha3: "LKA",
        name: "Sri Lanka",
        num: "144",
    });
    codes.push(CountryCode {
        alpha2: "TD",
        alpha3: "TCD",
        name: "Chad",
        num: "148",
    });
    codes.push(CountryCode {
        alpha2: "CL",
        alpha3: "CHL",
        name: "Chile",
        num: "152",
    });
    codes.push(CountryCode {
        alpha2: "CN",
        alpha3: "CHN",
        name: "China",
        num: "156",
    });
    codes.push(CountryCode {
        alpha2: "TW",
        alpha3: "TWN",
        name: "Taiwan, Province of China[a]",
        num: "158",
    });
    codes.push(CountryCode {
        alpha2: "CX",
        alpha3: "CXR",
        name: "Christmas Island",
        num: "162",
    });
    codes.push(CountryCode {
        alpha2: "CC",
        alpha3: "CCK",
        name: "Cocos (Keeling) Islands",
        num: "166",
    });
    codes.push(CountryCode {
        alpha2: "CO",
        alpha3: "COL",
        name: "Colombia",
        num: "170",
    });
    codes.push(CountryCode {
        alpha2: "KM",
        alpha3: "COM",
        name: "Comoros",
        num: "174",
    });
    codes.push(CountryCode {
        alpha2: "YT",
        alpha3: "MYT",
        name: "Mayotte",
        num: "175",
    });
    codes.push(CountryCode {
        alpha2: "CG",
        alpha3: "COG",
        name: "Congo",
        num: "178",
    });
    codes.push(CountryCode {
        alpha2: "CD",
        alpha3: "COD",
        name: "Congo (Democratic Republic of the)",
        num: "180",
    });
    codes.push(CountryCode {
        alpha2: "CK",
        alpha3: "COK",
        name: "Cook Islands",
        num: "184",
    });
    codes.push(CountryCode {
        alpha2: "CR",
        alpha3: "CRI",
        name: "Costa Rica",
        num: "188",
    });
    codes.push(CountryCode {
        alpha2: "HR",
        alpha3: "HRV",
        name: "Croatia",
        num: "191",
    });
    codes.push(CountryCode {
        alpha2: "CU",
        alpha3: "CUB",
        name: "Cuba",
        num: "192",
    });
    codes.push(CountryCode {
        alpha2: "CY",
        alpha3: "CYP",
        name: "Cyprus",
        num: "196",
    });
    codes.push(CountryCode {
        alpha2: "CZ",
        alpha3: "CZE",
        name: "Czech Republic",
        num: "203",
    });
    codes.push(CountryCode {
        alpha2: "BJ",
        alpha3: "BEN",
        name: "Benin",
        num: "204",
    });
    codes.push(CountryCode {
        alpha2: "DK",
        alpha3: "DNK",
        name: "Denmark",
        num: "208",
    });
    codes.push(CountryCode {
        alpha2: "DM",
        alpha3: "DMA",
        name: "Dominica",
        num: "212",
    });
    codes.push(CountryCode {
        alpha2: "DO",
        alpha3: "DOM",
        name: "Dominican Republic",
        num: "214",
    });
    codes.push(CountryCode {
        alpha2: "EC",
        alpha3: "ECU",
        name: "Ecuador",
        num: "218",
    });
    codes.push(CountryCode {
        alpha2: "SV",
        alpha3: "SLV",
        name: "El Salvador",
        num: "222",
    });
    codes.push(CountryCode {
        alpha2: "GQ",
        alpha3: "GNQ",
        name: "Equatorial Guinea",
        num: "226",
    });
    codes.push(CountryCode {
        alpha2: "ET",
        alpha3: "ETH",
        name: "Ethiopia",
        num: "231",
    });
    codes.push(CountryCode {
        alpha2: "ER",
        alpha3: "ERI",
        name: "Eritrea",
        num: "232",
    });
    codes.push(CountryCode {
        alpha2: "EE",
        alpha3: "EST",
        name: "Estonia",
        num: "233",
    });
    codes.push(CountryCode {
        alpha2: "FO",
        alpha3: "FRO",
        name: "Faroe Islands",
        num: "234",
    });
    codes.push(CountryCode {
        alpha2: "FK",
        alpha3: "FLK",
        name: "Falkland Islands",
        num: "238",
    });
    codes.push(CountryCode {
        alpha2: "GS",
        alpha3: "SGS",
        name: "South Georgia and the South Sandwich Islands",
        num: "239",
    });
    codes.push(CountryCode {
        alpha2: "FJ",
        alpha3: "FJI",
        name: "Fiji",
        num: "242",
    });
    codes.push(CountryCode {
        alpha2: "FI",
        alpha3: "FIN",
        name: "Finland",
        num: "246",
    });
    codes.push(CountryCode {
        alpha2: "AX",
        alpha3: "ALA",
        name: "Åland Islands",
        num: "248",
    });
    codes.push(CountryCode {
        alpha2: "FR",
        alpha3: "FRA",
        name: "France",
        num: "250",
    });
    codes.push(CountryCode {
        alpha2: "GF",
        alpha3: "GUF",
        name: "French Guiana",
        num: "254",
    });
    codes.push(CountryCode {
        alpha2: "PF",
        alpha3: "PYF",
        name: "French Polynesia",
        num: "258",
    });
    codes.push(CountryCode {
        alpha2: "TF",
        alpha3: "ATF",
        name: "French Southern Territories",
        num: "260",
    });
    codes.push(CountryCode {
        alpha2: "DJ",
        alpha3: "DJI",
        name: "Djibouti",
        num: "262",
    });
    codes.push(CountryCode {
        alpha2: "GA",
        alpha3: "GAB",
        name: "Gabon",
        num: "266",
    });
    codes.push(CountryCode {
        alpha2: "GE",
        alpha3: "GEO",
        name: "Georgia",
        num: "268",
    });
    codes.push(CountryCode {
        alpha2: "GM",
        alpha3: "GMB",
        name: "Gambia",
        num: "270",
    });
    codes.push(CountryCode {
        alpha2: "PS",
        alpha3: "PSE",
        name: "Palestine, State of",
        num: "275",
    });
    codes.push(CountryCode {
        alpha2: "DE",
        alpha3: "DEU",
        name: "Germany",
        num: "276",
    });
    codes.push(CountryCode {
        alpha2: "GH",
        alpha3: "GHA",
        name: "Ghana",
        num: "288",
    });
    codes.push(CountryCode {
        alpha2: "GI",
        alpha3: "GIB",
        name: "Gibraltar",
        num: "292",
    });
    codes.push(CountryCode {
        alpha2: "KI",
        alpha3: "KIR",
        name: "Kiribati",
        num: "296",
    });
    codes.push(CountryCode {
        alpha2: "GR",
        alpha3: "GRC",
        name: "Greece",
        num: "300",
    });
    codes.push(CountryCode {
        alpha2: "GL",
        alpha3: "GRL",
        name: "Greenland",
        num: "304",
    });
    codes.push(CountryCode {
        alpha2: "GD",
        alpha3: "GRD",
        name: "Grenada",
        num: "308",
    });
    codes.push(CountryCode {
        alpha2: "GP",
        alpha3: "GLP",
        name: "Guadeloupe",
        num: "312",
    });
    codes.push(CountryCode {
        alpha2: "GU",
        alpha3: "GUM",
        name: "Guam",
        num: "316",
    });
    codes.push(CountryCode {
        alpha2: "GT",
        alpha3: "GTM",
        name: "Guatemala",
        num: "320",
    });
    codes.push(CountryCode {
        alpha2: "GN",
        alpha3: "GIN",
        name: "Guinea",
        num: "324",
    });
    codes.push(CountryCode {
        alpha2: "GY",
        alpha3: "GUY",
        name: "Guyana",
        num: "328",
    });
    codes.push(CountryCode {
        alpha2: "HT",
        alpha3: "HTI",
        name: "Haiti",
        num: "332",
    });
    codes.push(CountryCode {
        alpha2: "HM",
        alpha3: "HMD",
        name: "Heard Island and McDonald Islands",
        num: "334",
    });
    codes.push(CountryCode {
        alpha2: "VA",
        alpha3: "VAT",
        name: "Holy See",
        num: "336",
    });
    codes.push(CountryCode {
        alpha2: "HN",
        alpha3: "HND",
        name: "Honduras",
        num: "340",
    });
    codes.push(CountryCode {
        alpha2: "HK",
        alpha3: "HKG",
        name: "Hong Kong",
        num: "344",
    });
    codes.push(CountryCode {
        alpha2: "HU",
        alpha3: "HUN",
        name: "Hungary",
        num: "348",
    });
    codes.push(CountryCode {
        alpha2: "IS",
        alpha3: "ISL",
        name: "Iceland",
        num: "352",
    });
    codes.push(CountryCode {
        alpha2: "IN",
        alpha3: "IND",
        name: "India",
        num: "356",
    });
    codes.push(CountryCode {
        alpha2: "ID",
        alpha3: "IDN",
        name: "Indonesia",
        num: "360",
    });
    codes.push(CountryCode {
        alpha2: "IR",
        alpha3: "IRN",
        name: "Iran (Islamic Republic of)",
        num: "364",
    });
    codes.push(CountryCode {
        alpha2: "IQ",
        alpha3: "IRQ",
        name: "Iraq",
        num: "368",
    });
    codes.push(CountryCode {
        alpha2: "IE",
        alpha3: "IRL",
        name: "Ireland",
        num: "372",
    });
    codes.push(CountryCode {
        alpha2: "IL",
        alpha3: "ISR",
        name: "Israel",
        num: "376",
    });
    codes.push(CountryCode {
        alpha2: "IT",
        alpha3: "ITA",
        name: "Italy",
        num: "380",
    });
    codes.push(CountryCode {
        alpha2: "CI",
        alpha3: "CIV",
        name: "Côte d'Ivoire",
        num: "384",
    });
    codes.push(CountryCode {
        alpha2: "JM",
        alpha3: "JAM",
        name: "Jamaica",
        num: "388",
    });
    codes.push(CountryCode {
        alpha2: "JP",
        alpha3: "JPN",
        name: "Japan",
        num: "392",
    });
    codes.push(CountryCode {
        alpha2: "KZ",
        alpha3: "KAZ",
        name: "Kazakhstan",
        num: "398",
    });
    codes.push(CountryCode {
        alpha2: "JO",
        alpha3: "JOR",
        name: "Jordan",
        num: "400",
    });
    codes.push(CountryCode {
        alpha2: "KE",
        alpha3: "KEN",
        name: "Kenya",
        num: "404",
    });
    codes.push(CountryCode {
        alpha2: "KP",
        alpha3: "PRK",
        name: "Korea (Democratic People's Republic of)",
        num: "408",
    });
    codes.push(CountryCode {
        alpha2: "KR",
        alpha3: "KOR",
        name: "Korea (Republic of)",
        num: "410",
    });
    codes.push(CountryCode {
        alpha2: "KW",
        alpha3: "KWT",
        name: "Kuwait",
        num: "414",
    });
    codes.push(CountryCode {
        alpha2: "KG",
        alpha3: "KGZ",
        name: "Kyrgyzstan",
        num: "417",
    });
    codes.push(CountryCode {
        alpha2: "LA",
        alpha3: "LAO",
        name: "Lao People's Democratic Republic",
        num: "418",
    });
    codes.push(CountryCode {
        alpha2: "LB",
        alpha3: "LBN",
        name: "Lebanon",
        num: "422",
    });
    codes.push(CountryCode {
        alpha2: "LS",
        alpha3: "LSO",
        name: "Lesotho",
        num: "426",
    });
    codes.push(CountryCode {
        alpha2: "LV",
        alpha3: "LVA",
        name: "Latvia",
        num: "428",
    });
    codes.push(CountryCode {
        alpha2: "LR",
        alpha3: "LBR",
        name: "Liberia",
        num: "430",
    });
    codes.push(CountryCode {
        alpha2: "LY",
        alpha3: "LBY",
        name: "Libya",
        num: "434",
    });
    codes.push(CountryCode {
        alpha2: "LI",
        alpha3: "LIE",
        name: "Liechtenstein",
        num: "438",
    });
    codes.push(CountryCode {
        alpha2: "LT",
        alpha3: "LTU",
        name: "Lithuania",
        num: "440",
    });
    codes.push(CountryCode {
        alpha2: "LU",
        alpha3: "LUX",
        name: "Luxembourg",
        num: "442",
    });
    codes.push(CountryCode {
        alpha2: "MO",
        alpha3: "MAC",
        name: "Macao",
        num: "446",
    });
    codes.push(CountryCode {
        alpha2: "MG",
        alpha3: "MDG",
        name: "Madagascar",
        num: "450",
    });
    codes.push(CountryCode {
        alpha2: "MW",
        alpha3: "MWI",
        name: "Malawi",
        num: "454",
    });
    codes.push(CountryCode {
        alpha2: "MY",
        alpha3: "MYS",
        name: "Malaysia",
        num: "458",
    });
    codes.push(CountryCode {
        alpha2: "MV",
        alpha3: "MDV",
        name: "Maldives",
        num: "462",
    });
    codes.push(CountryCode {
        alpha2: "ML",
        alpha3: "MLI",
        name: "Mali",
        num: "466",
    });
    codes.push(CountryCode {
        alpha2: "MT",
        alpha3: "MLT",
        name: "Malta",
        num: "470",
    });
    codes.push(CountryCode {
        alpha2: "MQ",
        alpha3: "MTQ",
        name: "Martinique",
        num: "474",
    });
    codes.push(CountryCode {
        alpha2: "MR",
        alpha3: "MRT",
        name: "Mauritania",
        num: "478",
    });
    codes.push(CountryCode {
        alpha2: "MU",
        alpha3: "MUS",
        name: "Mauritius",
        num: "480",
    });
    codes.push(CountryCode {
        alpha2: "MX",
        alpha3: "MEX",
        name: "Mexico",
        num: "484",
    });
    codes.push(CountryCode {
        alpha2: "MC",
        alpha3: "MCO",
        name: "Monaco",
        num: "492",
    });
    codes.push(CountryCode {
        alpha2: "MN",
        alpha3: "MNG",
        name: "Mongolia",
        num: "496",
    });
    codes.push(CountryCode {
        alpha2: "MD",
        alpha3: "MDA",
        name: "Moldova (Republic of)",
        num: "498",
    });
    codes.push(CountryCode {
        alpha2: "ME",
        alpha3: "MNE",
        name: "Montenegro",
        num: "499",
    });
    codes.push(CountryCode {
        alpha2: "MS",
        alpha3: "MSR",
        name: "Montserrat",
        num: "500",
    });
    codes.push(CountryCode {
        alpha2: "MA",
        alpha3: "MAR",
        name: "Morocco",
        num: "504",
    });
    codes.push(CountryCode {
        alpha2: "MZ",
        alpha3: "MOZ",
        name: "Mozambique",
        num: "508",
    });
    codes.push(CountryCode {
        alpha2: "OM",
        alpha3: "OMN",
        name: "Oman",
        num: "512",
    });
    codes.push(CountryCode {
        alpha2: "NA",
        alpha3: "NAM",
        name: "Namibia",
        num: "516",
    });
    codes.push(CountryCode {
        alpha2: "NR",
        alpha3: "NRU",
        name: "Nauru",
        num: "520",
    });
    codes.push(CountryCode {
        alpha2: "NP",
        alpha3: "NPL",
        name: "Nepal",
        num: "524",
    });
    codes.push(CountryCode {
        alpha2: "NL",
        alpha3: "NLD",
        name: "Netherlands",
        num: "528",
    });
    codes.push(CountryCode {
        alpha2: "CW",
        alpha3: "CUW",
        name: "Curaçao",
        num: "531",
    });
    codes.push(CountryCode {
        alpha2: "AW",
        alpha3: "ABW",
        name: "Aruba",
        num: "533",
    });
    codes.push(CountryCode {
        alpha2: "SX",
        alpha3: "SXM",
        name: "Sint Maarten (Dutch part)",
        num: "534",
    });
    codes.push(CountryCode {
        alpha2: "BQ",
        alpha3: "BES",
        name: "Bonaire, Sint Eustatius and Saba",
        num: "535",
    });
    codes.push(CountryCode {
        alpha2: "NC",
        alpha3: "NCL",
        name: "New Caledonia",
        num: "540",
    });
    codes.push(CountryCode {
        alpha2: "VU",
        alpha3: "VUT",
        name: "Vanuatu",
        num: "548",
    });
    codes.push(CountryCode {
        alpha2: "NZ",
        alpha3: "NZL",
        name: "New Zealand",
        num: "554",
    });
    codes.push(CountryCode {
        alpha2: "NI",
        alpha3: "NIC",
        name: "Nicaragua",
        num: "558",
    });
    codes.push(CountryCode {
        alpha2: "NE",
        alpha3: "NER",
        name: "Niger",
        num: "562",
    });
    codes.push(CountryCode {
        alpha2: "NG",
        alpha3: "NGA",
        name: "Nigeria",
        num: "566",
    });
    codes.push(CountryCode {
        alpha2: "NU",
        alpha3: "NIU",
        name: "Niue",
        num: "570",
    });
    codes.push(CountryCode {
        alpha2: "NF",
        alpha3: "NFK",
        name: "Norfolk Island",
        num: "574",
    });
    codes.push(CountryCode {
        alpha2: "NO",
        alpha3: "NOR",
        name: "Norway",
        num: "578",
    });
    codes.push(CountryCode {
        alpha2: "MP",
        alpha3: "MNP",
        name: "Northern Mariana Islands",
        num: "580",
    });
    codes.push(CountryCode {
        alpha2: "UM",
        alpha3: "UMI",
        name: "United States Minor Outlying Islands",
        num: "581",
    });
    codes.push(CountryCode {
        alpha2: "FM",
        alpha3: "FSM",
        name: "Micronesia (Federated States of)",
        num: "583",
    });
    codes.push(CountryCode {
        alpha2: "MH",
        alpha3: "MHL",
        name: "Marshall Islands",
        num: "584",
    });
    codes.push(CountryCode {
        alpha2: "PW",
        alpha3: "PLW",
        name: "Palau",
        num: "585",
    });
    codes.push(CountryCode {
        alpha2: "PK",
        alpha3: "PAK",
        name: "Pakistan",
        num: "586",
    });
    codes.push(CountryCode {
        alpha2: "PA",
        alpha3: "PAN",
        name: "Panama",
        num: "591",
    });
    codes.push(CountryCode {
        alpha2: "PG",
        alpha3: "PNG",
        name: "Papua New Guinea",
        num: "598",
    });
    codes.push(CountryCode {
        alpha2: "PY",
        alpha3: "PRY",
        name: "Paraguay",
        num: "600",
    });
    codes.push(CountryCode {
        alpha2: "PE",
        alpha3: "PER",
        name: "Peru",
        num: "604",
    });
    codes.push(CountryCode {
        alpha2: "PH",
        alpha3: "PHL",
        name: "Philippines",
        num: "608",
    });
    codes.push(CountryCode {
        alpha2: "PN",
        alpha3: "PCN",
        name: "Pitcairn",
        num: "612",
    });
    codes.push(CountryCode {
        alpha2: "PL",
        alpha3: "POL",
        name: "Poland",
        num: "616",
    });
    codes.push(CountryCode {
        alpha2: "PT",
        alpha3: "PRT",
        name: "Portugal",
        num: "620",
    });
    codes.push(CountryCode {
        alpha2: "GW",
        alpha3: "GNB",
        name: "Guinea-Bissau",
        num: "624",
    });
    codes.push(CountryCode {
        alpha2: "TL",
        alpha3: "TLS",
        name: "Timor-Leste",
        num: "626",
    });
    codes.push(CountryCode {
        alpha2: "PR",
        alpha3: "PRI",
        name: "Puerto Rico",
        num: "630",
    });
    codes.push(CountryCode {
        alpha2: "QA",
        alpha3: "QAT",
        name: "Qatar",
        num: "634",
    });
    codes.push(CountryCode {
        alpha2: "RE",
        alpha3: "REU",
        name: "Réunion",
        num: "638",
    });
    codes.push(CountryCode {
        alpha2: "RO",
        alpha3: "ROU",
        name: "Romania",
        num: "642",
    });
    codes.push(CountryCode {
        alpha2: "RU",
        alpha3: "RUS",
        name: "Russian Federation",
        num: "643",
    });
    codes.push(CountryCode {
        alpha2: "RW",
        alpha3: "RWA",
        name: "Rwanda",
        num: "646",
    });
    codes.push(CountryCode {
        alpha2: "BL",
        alpha3: "BLM",
        name: "Saint Barthélemy",
        num: "652",
    });
    codes.push(CountryCode {
        alpha2: "SH",
        alpha3: "SHN",
        name: "Saint Helena, Ascension and Tristan da Cunha",
        num: "654",
    });
    codes.push(CountryCode {
        alpha2: "KN",
        alpha3: "KNA",
        name: "Saint Kitts and Nevis",
        num: "659",
    });
    codes.push(CountryCode {
        alpha2: "AI",
        alpha3: "AIA",
        name: "Anguilla",
        num: "660",
    });
    codes.push(CountryCode {
        alpha2: "LC",
        alpha3: "LCA",
        name: "Saint Lucia",
        num: "662",
    });
    codes.push(CountryCode {
        alpha2: "MF",
        alpha3: "MAF",
        name: "Saint Martin (French part)",
        num: "663",
    });
    codes.push(CountryCode {
        alpha2: "PM",
        alpha3: "SPM",
        name: "Saint Pierre and Miquelon",
        num: "666",
    });
    codes.push(CountryCode {
        alpha2: "VC",
        alpha3: "VCT",
        name: "Saint Vincent and the Grenadines",
        num: "670",
    });
    codes.push(CountryCode {
        alpha2: "SM",
        alpha3: "SMR",
        name: "San Marino",
        num: "674",
    });
    codes.push(CountryCode {
        alpha2: "ST",
        alpha3: "STP",
        name: "Sao Tome and Principe",
        num: "678",
    });
    codes.push(CountryCode {
        alpha2: "SA",
        alpha3: "SAU",
        name: "Saudi Arabia",
        num: "682",
    });
    codes.push(CountryCode {
        alpha2: "SN",
        alpha3: "SEN",
        name: "Senegal",
        num: "686",
    });
    codes.push(CountryCode {
        alpha2: "RS",
        alpha3: "SRB",
        name: "Serbia",
        num: "688",
    });
    codes.push(CountryCode {
        alpha2: "SC",
        alpha3: "SYC",
        name: "Seychelles",
        num: "690",
    });
    codes.push(CountryCode {
        alpha2: "SL",
        alpha3: "SLE",
        name: "Sierra Leone",
        num: "694",
    });
    codes.push(CountryCode {
        alpha2: "SG",
        alpha3: "SGP",
        name: "Singapore",
        num: "702",
    });
    codes.push(CountryCode {
        alpha2: "SK",
        alpha3: "SVK",
        name: "Slovakia",
        num: "703",
    });
    codes.push(CountryCode {
        alpha2: "VN",
        alpha3: "VNM",
        name: "Viet Nam",
        num: "704",
    });
    codes.push(CountryCode {
        alpha2: "SI",
        alpha3: "SVN",
        name: "Slovenia",
        num: "705",
    });
    codes.push(CountryCode {
        alpha2: "SO",
        alpha3: "SOM",
        name: "Somalia",
        num: "706",
    });
    codes.push(CountryCode {
        alpha2: "ZA",
        alpha3: "ZAF",
        name: "South Africa",
        num: "710",
    });
    codes.push(CountryCode {
        alpha2: "ZW",
        alpha3: "ZWE",
        name: "Zimbabwe",
        num: "716",
    });
    codes.push(CountryCode {
        alpha2: "ES",
        alpha3: "ESP",
        name: "Spain",
        num: "724",
    });
    codes.push(CountryCode {
        alpha2: "SS",
        alpha3: "SSD",
        name: "South Sudan",
        num: "728",
    });
    codes.push(CountryCode {
        alpha2: "SD",
        alpha3: "SDN",
        name: "Sudan",
        num: "729",
    });
    codes.push(CountryCode {
        alpha2: "EH",
        alpha3: "ESH",
        name: "Western Sahara",
        num: "732",
    });
    codes.push(CountryCode {
        alpha2: "SR",
        alpha3: "SUR",
        name: "Suriname",
        num: "740",
    });
    codes.push(CountryCode {
        alpha2: "SJ",
        alpha3: "SJM",
        name: "Svalbard and Jan Mayen",
        num: "744",
    });
    codes.push(CountryCode {
        alpha2: "SZ",
        alpha3: "SWZ",
        name: "Swaziland",
        num: "748",
    });
    codes.push(CountryCode {
        alpha2: "SE",
        alpha3: "SWE",
        name: "Sweden",
        num: "752",
    });
    codes.push(CountryCode {
        alpha2: "CH",
        alpha3: "CHE",
        name: "Switzerland",
        num: "756",
    });
    codes.push(CountryCode {
        alpha2: "SY",
        alpha3: "SYR",
        name: "Syrian Arab Republic",
        num: "760",
    });
    codes.push(CountryCode {
        alpha2: "TJ",
        alpha3: "TJK",
        name: "Tajikistan",
        num: "762",
    });
    codes.push(CountryCode {
        alpha2: "TH",
        alpha3: "THA",
        name: "Thailand",
        num: "764",
    });
    codes.push(CountryCode {
        alpha2: "TG",
        alpha3: "TGO",
        name: "Togo",
        num: "768",
    });
    codes.push(CountryCode {
        alpha2: "TK",
        alpha3: "TKL",
        name: "Tokelau",
        num: "772",
    });
    codes.push(CountryCode {
        alpha2: "TO",
        alpha3: "TON",
        name: "Tonga",
        num: "776",
    });
    codes.push(CountryCode {
        alpha2: "TT",
        alpha3: "TTO",
        name: "Trinidad and Tobago",
        num: "780",
    });
    codes.push(CountryCode {
        alpha2: "AE",
        alpha3: "ARE",
        name: "United Arab Emirates",
        num: "784",
    });
    codes.push(CountryCode {
        alpha2: "TN",
        alpha3: "TUN",
        name: "Tunisia",
        num: "788",
    });
    codes.push(CountryCode {
        alpha2: "TR",
        alpha3: "TUR",
        name: "Turkey",
        num: "792",
    });
    codes.push(CountryCode {
        alpha2: "TM",
        alpha3: "TKM",
        name: "Turkmenistan",
        num: "795",
    });
    codes.push(CountryCode {
        alpha2: "TC",
        alpha3: "TCA",
        name: "Turks and Caicos Islands",
        num: "796",
    });
    codes.push(CountryCode {
        alpha2: "TV",
        alpha3: "TUV",
        name: "Tuvalu",
        num: "798",
    });
    codes.push(CountryCode {
        alpha2: "UG",
        alpha3: "UGA",
        name: "Uganda",
        num: "800",
    });
    codes.push(CountryCode {
        alpha2: "UA",
        alpha3: "UKR",
        name: "Ukraine",
        num: "804",
    });
    codes.push(CountryCode {
        alpha2: "MK",
        alpha3: "MKD",
        name: "Macedonia (the former Yugoslav Republic of)",
        num: "807",
    });
    codes.push(CountryCode {
        alpha2: "EG",
        alpha3: "EGY",
        name: "Egypt",
        num: "818",
    });
    codes.push(CountryCode {
        alpha2: "GB",
        alpha3: "GBR",
        name: "United Kingdom of Great Britain and Northern Ireland",
        num: "826",
    });
    codes.push(CountryCode {
        alpha2: "GG",
        alpha3: "GGY",
        name: "Guernsey",
        num: "831",
    });
    codes.push(CountryCode {
        alpha2: "JE",
        alpha3: "JEY",
        name: "Jersey",
        num: "832",
    });
    codes.push(CountryCode {
        alpha2: "IM",
        alpha3: "IMN",
        name: "Isle of Man",
        num: "833",
    });
    codes.push(CountryCode {
        alpha2: "TZ",
        alpha3: "TZA",
        name: "Tanzania, United Republic of",
        num: "834",
    });
    codes.push(CountryCode {
        alpha2: "US",
        alpha3: "USA",
        name: "United States of America",
        num: "840",
    });
    codes.push(CountryCode {
        alpha2: "VI",
        alpha3: "VIR",
        name: "Virgin Islands (U.S.)",
        num: "850",
    });
    codes.push(CountryCode {
        alpha2: "BF",
        alpha3: "BFA",
        name: "Burkina Faso",
        num: "854",
    });
    codes.push(CountryCode {
        alpha2: "UY",
        alpha3: "URY",
        name: "Uruguay",
        num: "858",
    });
    codes.push(CountryCode {
        alpha2: "UZ",
        alpha3: "UZB",
        name: "Uzbekistan",
        num: "860",
    });
    codes.push(CountryCode {
        alpha2: "VE",
        alpha3: "VEN",
        name: "Venezuela (Bolivarian Republic of)",
        num: "862",
    });
    codes.push(CountryCode {
        alpha2: "WF",
        alpha3: "WLF",
        name: "Wallis and Futuna",
        num: "876",
    });
    codes.push(CountryCode {
        alpha2: "WS",
        alpha3: "WSM",
        name: "Samoa",
        num: "882",
    });
    codes.push(CountryCode {
        alpha2: "YE",
        alpha3: "YEM",
        name: "Yemen",
        num: "887",
    });
    codes.push(CountryCode {
        alpha2: "ZM",
        alpha3: "ZMB",
        name: "Zambia",
        num: "894",
    });
    // End

    codes
}
