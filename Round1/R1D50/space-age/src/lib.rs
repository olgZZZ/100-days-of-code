// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    pub secs: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { secs: s }
    }
}

pub trait Planet {
    const SECONDS_IN_YEAR: f64;
    fn years_during(d: &Duration) -> f64 {
        d.secs as f64 / Self::SECONDS_IN_YEAR
    }
}

const SECONDS_IN_EARTH_YEAR: f64 = 31557600_f64;

macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n; 
        impl Planet for $n { 
            const SECONDS_IN_YEAR: f64 = $p * SECONDS_IN_EARTH_YEAR; 
        }
    }
}
    

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

