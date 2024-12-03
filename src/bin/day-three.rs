fn main() {
    match is_nice(10, 2) {
        true => println!("Fred is good"),
        false => println!("Fred is on the naughty list"),
    }
    match is_nice(15, 1) {
        true => println!("John is good"),
        false => println!("John is on the naughty list"),
    }
    match is_nice(0, 0) {
        true => println!("Gert is good"),
        false => println!("Gert is on the naughty list"),
    }
}

const BAD_WEIGHT: f32 = 2.0;
const GOOD_WEIGHT: f32 = 1.0;

fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    let good_deeds = good_deeds as f32;
    let bad_deeds = bad_deeds as f32;
    if good_deeds == 0.0 && bad_deeds == 0.0 {
        return false;
    }

    let ratio = (good_deeds * GOOD_WEIGHT) / (good_deeds + (BAD_WEIGHT * bad_deeds));
    if ratio >= 0.75 {
        return true;
    }
    return false;
}
