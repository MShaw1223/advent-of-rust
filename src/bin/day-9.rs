fn main() {
    Snowball::new(10);
}

const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl From<SnowKg> for Snowball {
    fn from(value: SnowKg) -> Self {
        let value = (value.0 / SNOWBALL_WEIGHT_KG).round() as i64;
        Snowball(value)
    }
}
impl From<SnowLb> for Snowball {
    fn from(value: SnowLb) -> Self {
        let value = (value.0 / SNOWBALL_WEIGHT_LB).round() as i64;
        Snowball(value)
    }
}