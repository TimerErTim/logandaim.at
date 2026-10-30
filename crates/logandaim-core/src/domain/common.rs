//! Small value types shared by more than one aggregate.

use std::fmt;
use std::num::NonZeroU32;
use std::str::FromStr;

/// A string with at least one visible character; surrounding whitespace is trimmed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NonEmptyString(String);

#[derive(Debug, thiserror::Error)]
#[error("der Text darf nicht leer sein")]
pub struct NonEmptyStringFehler;

impl NonEmptyString {
    pub fn neu<S: AsRef<str>>(wert: S) -> Result<Self, NonEmptyStringFehler> {
        let wert = wert.as_ref().trim();
        if wert.is_empty() {
            return Err(NonEmptyStringFehler);
        }
        Ok(Self(wert.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NonEmptyString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for NonEmptyString {
    type Err = NonEmptyStringFehler;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::neu(value)
    }
}

/// A colour held as three rgb floats, parsed from and rendered as `#rrggbb`.
///
/// Caller data, not a design token: the product owns no palette. The hex spelling is
/// lowercase, so two spellings of one colour compare equal.
#[derive(Debug, Clone, Copy)]
pub struct Farbe {
    r: f32,
    g: f32,
    b: f32,
}

#[derive(Debug, thiserror::Error)]
#[error("die Farbe „{0}“ ist keine Hex-Farbe der Form #rrggbb")]
pub struct FarbeFehler(pub String);

fn kanal_aus_byte(wert: f32) -> u8 {
    (wert.clamp(0.0, 1.0) * 255.0).round() as u8
}

impl Farbe {
    pub fn neu<S: AsRef<str>>(wert: S) -> Result<Self, FarbeFehler> {
        let wert = wert.as_ref().trim();
        let hex = wert.strip_prefix('#').unwrap_or(wert);
        if hex.len() != 6 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(FarbeFehler(wert.to_owned()));
        }
        // Only ASCII hex digits reach here, so byte slicing is safe.
        let kanal = |start: usize| {
            u8::from_str_radix(&hex[start..start + 2], 16).expect("geprüfte Hex-Ziffern") as f32
                / 255.0
        };
        Ok(Self {
            r: kanal(0),
            g: kanal(2),
            b: kanal(4),
        })
    }

    pub fn rgb(self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }

    /// The colour as lowercase `#rrggbb`.
    pub fn to_hex(self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            kanal_aus_byte(self.r),
            kanal_aus_byte(self.g),
            kanal_aus_byte(self.b)
        )
    }
}

impl PartialEq for Farbe {
    fn eq(&self, other: &Self) -> bool {
        self.r.to_bits() == other.r.to_bits()
            && self.g.to_bits() == other.g.to_bits()
            && self.b.to_bits() == other.b.to_bits()
    }
}

impl Eq for Farbe {}

impl std::hash::Hash for Farbe {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
    }
}

impl fmt::Display for Farbe {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

impl FromStr for Farbe {
    type Err = FarbeFehler;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::neu(value)
    }
}

/// A configured distance in metres, positive by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Abstand(NonZeroU32);

#[derive(Debug, thiserror::Error)]
#[error("der Abstand muss positiv sein, war aber 0 m")]
pub struct AbstandFehler;

impl Abstand {
    /// Parsing entry point: a plain `u32` so a stored or submitted `0` is an error, not a
    /// silently defaulted distance.
    pub fn neu(meter: u32) -> Result<Self, AbstandFehler> {
        NonZeroU32::new(meter).map(Self).ok_or(AbstandFehler)
    }

    pub fn meter(self) -> u32 {
        self.0.get()
    }

    pub fn als_nonzero(self) -> NonZeroU32 {
        self.0
    }
}

impl From<Abstand> for u32 {
    fn from(abstand: Abstand) -> Self {
        abstand.meter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn non_empty_string_trims_and_refuses_blank() {
        assert_eq!(NonEmptyString::neu("  Keiler ").unwrap().as_str(), "Keiler");
        assert!(NonEmptyString::neu("   ").is_err());
        assert!(NonEmptyString::neu("").is_err());
    }

    #[test_log::test]
    fn farbe_roundtrips_hex() {
        let farbe = Farbe::neu("#C0392B").unwrap();
        assert_eq!(farbe.to_hex(), "#c0392b");
        assert_eq!(Farbe::neu(" #c0392b ").unwrap(), farbe);
        assert_eq!("c0392b".parse::<Farbe>().unwrap(), farbe);
    }

    #[test_log::test]
    fn farbe_keeps_channels_after_roundtrip() {
        for hex in ["#000000", "#ffffff", "#1b4f9c", "#e8b81c"] {
            assert_eq!(Farbe::neu(hex).unwrap().to_hex(), hex);
        }
        let (r, g, b) = Farbe::neu("#ffffff").unwrap().rgb();
        assert_eq!((r, g, b), (1.0, 1.0, 1.0));
    }

    #[test_log::test]
    fn farbe_refuses_non_hex() {
        for wert in ["", "Rot", "#c0392", "#c0392bb", "#c0392g"] {
            assert!(Farbe::neu(wert).is_err(), "{wert:?} ist keine Hex-Farbe");
        }
    }

    #[test_log::test]
    fn abstand_is_nonzero() {
        assert_eq!(Abstand::neu(40).unwrap().meter(), 40);
        assert!(Abstand::neu(0).is_err());
    }
}
