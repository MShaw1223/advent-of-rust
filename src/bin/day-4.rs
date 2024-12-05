fn main() {
    let foo = Kid::new("john".to_string(), 10, 6);
    println!("{}", foo.name);
    println!("{:?}", foo.niceness);
    let bar = Kid::new("carl".to_string(), 10, 1);
    println!("{}", bar.name);
    println!("{:?}", bar.niceness);
}
const GOOD_WEIGHT: f32 = 1.0;
const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
enum Niceness {
    Nice(u32),
    Naughty,
}

struct Kid {
    name: String,
    niceness: Niceness,
}

// HINT: Use Self::is_nice or Kid::is_nice to call the associated function from within the impl block.
impl Kid {
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
