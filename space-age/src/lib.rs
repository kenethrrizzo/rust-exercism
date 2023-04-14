// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const SECONDS_IN_A_EARTH_YEAR: u64 = 31557600;
const ORBITAL_PERIODS: [f64; 8] = [
    0.2408467,  // Mercury
    0.61519726, // Venus
    1.00000000, // Earth
    1.8808158,  // Mars
    11.862615,  // Jupiter
    29.447498,  // Saturn
    84.016846,  // Uranus
    164.79132,  // Neptune
];

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        println!("Duration in earth years: {}", s as f64 / SECONDS_IN_A_EARTH_YEAR as f64);
        return Duration {
            earth_years: s as f64 / SECONDS_IN_A_EARTH_YEAR as f64,
        };
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

// Tipo de argumentos en macros
// Identificadores (palabras clave, nombres de variables, etc.).
// Literales (números, cadenas de texto, caracteres, etc.).
// Patrones (para hacer coincidir y desestructurar tipos de datos complejos).
// Bloques de código (que pueden contener cualquier cosa, incluyendo otras macros).
macro_rules! impl_planet {
    ($name:ident, $orbital_period:expr) => {
        pub struct $name;

        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                d.earth_years / $orbital_period
            }
        }
    };
}

// Uso de las macros
impl_planet!(Mercury, ORBITAL_PERIODS[0]);
impl_planet!(Venus, ORBITAL_PERIODS[1]);
impl_planet!(Earth, ORBITAL_PERIODS[2]);
impl_planet!(Mars, ORBITAL_PERIODS[3]);
impl_planet!(Jupiter, ORBITAL_PERIODS[4]);
impl_planet!(Saturn, ORBITAL_PERIODS[5]);
impl_planet!(Uranus, ORBITAL_PERIODS[6]);
impl_planet!(Neptune, ORBITAL_PERIODS[7]);
