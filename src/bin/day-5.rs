fn main() {
    let foo = Kid::parse_row("Alice,3,1");
    println!("{foo:?}");
    let bar = Kid::parse_row("Bob,,2");
    println!("{bar:?}");
    let baz = Kid::parse_row("Don,10,1");
    println!("{baz:?}");
}
const GOOD_WEIGHT: f32 = 1.0;
const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
enum Niceness {
    Nice(u32),
    Naughty,
}

#[derive(Debug, PartialEq)]
struct Kid {
    name: String,
    niceness: Niceness,
}
// - - - hint from day four - - -
// HINT: Use Self::is_nice or Kid::is_nice to call the associated function from within the impl block.
impl Kid {
    fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        let row: Vec<&str> = csv_row.split_terminator(',').collect();

        if row.len() < 3 {
            return Err("Error in row: Missing fields");
        }

        let name: String = row[0].to_string();
        let mut good_deeds: u32 = 0;
        let mut bad_deeds: u32 = 0;

        match row[1].parse::<u32>() {
            Ok(val) => {
                good_deeds = val;
            }
            Err(_) => {
                return Err("Parsing good_deeds error");
            }
        };
        match row[2].parse::<u32>() {
            Ok(val) => {
                bad_deeds = val;
            }
            Err(_) => {
                return Err("Parsing bad_deeds error");
            }
        };

        Ok(Self::new(name, good_deeds, bad_deeds))
    }
    fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        match Kid::is_nice(good_deeds, bad_deeds) {
            true => Kid {
                name,
                niceness: Niceness::Nice(good_deeds),
            },
            false => Kid {
                name,
                niceness: Niceness::Naughty,
            },
        }
    }
    // Move yesterday's function to an associated function in the struct
    // (is_nice() is from day three)
    fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}
