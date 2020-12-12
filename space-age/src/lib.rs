// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    const DURATION_IN_YEAR: f64;
    fn years_during(d: &Duration) -> f64 {
            let &Duration(s) = d;
        s / Self::DURATION_IN_YEAR

    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const DURATION_IN_YEAR: f64 =
        Earth::DURATION_IN_YEAR
 * 0.2408467;
}
impl Planet for Venus {
    const DURATION_IN_YEAR: f64 =
        Earth::DURATION_IN_YEAR
 * 0.61519726;
}
impl Planet for Earth {
    const DURATION_IN_YEAR: f64 = 31_557_600_f64;
}
impl Planet for Mars {
    const DURATION_IN_YEAR: f64 =
        Earth::DURATION_IN_YEAR
 * 1.8808158;
}
impl Planet for Jupiter {
    const DURATION_IN_YEAR: f64 =
        Earth::DURATION_IN_YEAR
 * 11.862615;
}
impl Planet for Saturn {
    const DURATION_IN_YEAR: f64 =
        Earth::DURATION_IN_YEAR
 * 29.447498;
}
impl Planet for Uranus {
    const DURATION_IN_YEAR: f64 =
        Earth::DURATION_IN_YEAR
 * 84.016846;
}
impl Planet for Neptune {
    const DURATION_IN_YEAR: f64 =
        Earth::DURATION_IN_YEAR
 * 164.79132;
}