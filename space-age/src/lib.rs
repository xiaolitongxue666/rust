// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        // Return a Duration object with the given number of seconds
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! planet_impl {
    ($planet:ident, $years:expr) => {
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                // Return the number of years during which $planet has
                // been around the sun for the given duration
                d.seconds as f64 / 31557600.0 / $years
            }
        }
    };
    () => {};
}

planet_impl!(Mercury, 0.2408467);
planet_impl!(Venus, 0.61519726);
planet_impl!(Earth, 1.0);
planet_impl!(Mars, 1.8808158);
planet_impl!(Jupiter, 11.862615);
planet_impl!(Saturn, 29.447498);
planet_impl!(Uranus, 84.016846);
planet_impl!(Neptune, 164.79132);


// impl Planet for Mercury {
//     fn years_during(d: &Duration) -> f64 {
//         // Return the number of years during the given duration
//         (d.seconds as f64 / 31557600.0) / 0.2408467
//     }
// }
//
// impl Planet for Venus {
//     fn years_during(d: &Duration) -> f64 {
//         // Return the number of years during the given duration
//         d.seconds as f64 / 31557600.0 / 0.61519726
//     }
// }
//
// impl Planet for Earth {
//     fn years_during(d: &Duration) -> f64 {
//         // Return the number of Earth years during the given duration
//         d.seconds as f64 / 31557600.0 / 1.0
//     }
// }
//
// impl Planet for Mars {
//     fn years_during(d: &Duration) -> f64 {
//         // Return the number of Mars years during the given duration
//         d.seconds as f64 / 31557600.0 / 1.8808158
//     }
// }
//
// impl Planet for Jupiter {
//     fn years_during(d: &Duration) -> f64 {
//         // Return the number of Jupiter years during the given duration
//         d.seconds as f64 / 31557600.0 / 11.862615
//     }
// }
//
// impl Planet for Saturn {
//     fn years_during(d: &Duration) -> f64 {
//         // Return the number of Saturn years during the given duration
//         d.seconds as f64 / 31557600.0 / 29.447498
//     }
// }
//
// impl Planet for Uranus {
//     fn years_during(d: &Duration) -> f64 {
//         // Return the number of Uranus years during the given duration
//         d.seconds as f64 / 31557600.0 / 84.016846
//     }
// }
//
// impl Planet for Neptune {
//     fn years_during(d: &Duration) -> f64 {
//         // Return the number of Neptune years during the given duration
//         d.seconds as f64 / 31557600.0 / 164.79132
//     }
// }
