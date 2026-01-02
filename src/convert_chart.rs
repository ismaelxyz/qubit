#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Temperature {
    Kelvin,
    Celsius,
    Fahrenheit,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Acceleration {
    MetrePerSecondSquared,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Angle {
    Turn,
    Radian,
    Degree,
    Gradian,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Length {
    Millimetre,
    Centimetre,
    Metre,
    Kilometre,
    Inch,
    Foot,
    Yard,
    Mile,
    NauticalMile,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mass {
    Microgram,
    Milligram,
    Gram,
    Kilogram,
    MetricTon,
    Ounce,
    Pound,
    Stone,
    ShortTon,
    LongTon,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Time {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Decade,
    Century,
    Millenium,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Area {
    SquareMetre,
    Hectare,
    SquareKilometre,
    SquareInch,
    SquareFeet,
    SquareYard,
    Acre,
    SquareMile,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Speed {
    MetrePerSecond,
    KilometresPerHour,
    FeetPerSecond,
    MilesPerHour,
    Knot,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DigitalInformation {
    Bit,
    Byte,
    Kilobit,
    Kilobyte,
    Megabit,
    Megabyte,
    Gigabit,
    Gigabyte,
    Terabit,
    Terabyte,
    Petabit,
    Petabyte,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UnitType {
    Temperature(Temperature),
    Acceleration(Acceleration),
    Angle(Angle),
    Length(Length),
    Mass(Mass),
    Time(Time),
    Area(Area),
    Speed(Speed),
    Digitalinformation(DigitalInformation),
}

// pub struct InvalidConversion;

pub fn convert(value: f64, from: UnitType, to: UnitType) -> f64 {
    if from == to {
        return value;
    }
    if std::mem::discriminant(&from) != std::mem::discriminant(&to) {
        return f64::NAN;
    }
    match (find_conversion_factor(from), find_conversion_factor(to)) {
        (Ok(from), Ok(to)) => value * from / to,
        (Err(from), Err(to)) => match (from, to) {
            (Temperature::Kelvin, Temperature::Celsius) => value - 273.15,
            (Temperature::Kelvin, Temperature::Fahrenheit) => value.mul_add(1.8, -459.67),
            (Temperature::Kelvin, Temperature::Kelvin) => value,
            // CELSIUS
            (Temperature::Celsius, Temperature::Celsius) => value,
            (Temperature::Celsius, Temperature::Fahrenheit) => value.mul_add(1.8, 32.),
            (Temperature::Celsius, Temperature::Kelvin) => value + 273.15,
            // FAHRENHEIT
            (Temperature::Fahrenheit, Temperature::Celsius) => (value - 32f64) / 1.8,
            (Temperature::Fahrenheit, Temperature::Fahrenheit) => value,
            (Temperature::Fahrenheit, Temperature::Kelvin) => (value + 459.67) * 5f64 / 9f64,
        },
        _ => f64::NAN,
    }
}

/// Finds conversion factor if applicable, otherwise return which
/// actual unit does not have a fixed conversion factor.
pub fn find_conversion_factor(u: UnitType) -> Result<f64, Temperature> {
    Ok(match u {
        UnitType::Temperature(v) => return Err(v),
        UnitType::Acceleration(v) => match v {
            Acceleration::MetrePerSecondSquared => 1_f64,
        },
        UnitType::Angle(v) => match v {
            // 6.28318531,
            Angle::Turn => std::f64::consts::TAU,
            Angle::Radian => 1_f64,
            Angle::Degree => 0.0174532925,
            Angle::Gradian => 0.015707963267949,
        },
        UnitType::Length(v) => match v {
            Length::Millimetre => 0.001,
            Length::Centimetre => 0.01,
            Length::Metre => 1_f64,
            Length::Kilometre => 1000_f64,
            Length::Inch => 0.0254,
            Length::Foot => 0.3048,
            Length::Yard => 0.9144,
            Length::Mile => 1609.34,
            Length::NauticalMile => 1852_f64,
        },
        UnitType::Mass(v) => match v {
            Mass::Microgram => 1e-7_f64,
            Mass::Milligram => 1e-6_f64,
            Mass::Gram => 0.001,
            Mass::Kilogram => 1_f64,
            Mass::MetricTon => 1000_f64,
            Mass::Ounce => 0.0283495,
            Mass::Pound => 0.453592,
            Mass::Stone => 6.35029,
            Mass::ShortTon => 907.185,
            Mass::LongTon => 1016.0469088,
        },
        UnitType::Time(v) => match v {
            Time::Nanosecond => 1e-9,
            Time::Microsecond => 1e-6,
            Time::Millisecond => 0.001,
            Time::Second => 1_f64,
            Time::Minute => 60_f64,
            Time::Hour => 3600_f64,
            Time::Day => 86400_f64,
            Time::Week => 604800_f64,
            Time::Month => 2.62974e6,
            Time::Year => 3.15569e7,
            Time::Decade => 3.15569e8,
            Time::Century => 3.15569e9,
            Time::Millenium => 3.1556926e10,
        },
        UnitType::Area(v) => match v {
            Area::SquareMetre => 1_f64,
            Area::Hectare => 10000_f64,
            Area::SquareKilometre => 1000000_f64,
            Area::SquareInch => 0.00064516,
            Area::SquareFeet => 0.09290304,
            Area::SquareYard => 0.83612736,
            Area::Acre => 4046.8564224,
            Area::SquareMile => 2589988.110336,
        },
        UnitType::Speed(v) => match v {
            Speed::MetrePerSecond => 1_f64,
            Speed::KilometresPerHour => 0.277778,
            Speed::FeetPerSecond => 0.3048,
            Speed::MilesPerHour => 0.44704,
            Speed::Knot => 0.514444,
        },
        UnitType::Digitalinformation(v) => match v {
            DigitalInformation::Bit => 0.00012207,
            DigitalInformation::Byte => 0.000976563,
            DigitalInformation::Kilobit => 0.125,
            DigitalInformation::Kilobyte => 1_f64,
            DigitalInformation::Megabit => 128_f64,
            DigitalInformation::Megabyte => 1024_f64,
            DigitalInformation::Gigabit => 131072_f64,
            DigitalInformation::Gigabyte => 1.049e+6,
            DigitalInformation::Terabit => 1.342e+8,
            DigitalInformation::Terabyte => 1.074e+9,
            DigitalInformation::Petabit => 1.374e+11,
            DigitalInformation::Petabyte => 1.1e+12,
        },
    })
}

impl std::str::FromStr for UnitType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Temprature
            "TEMPERATURE::KELVIN" => Ok(UnitType::Temperature(Temperature::Kelvin)),
            "TEMPERATURE::FAHRENHEIT" => Ok(UnitType::Temperature(Temperature::Fahrenheit)),
            "TEMPERATURE::CELSIUS" => Ok(UnitType::Temperature(Temperature::Celsius)),
            // Angle
            "ACCELERATION::MetrePerSecondSquared" => {
                Ok(UnitType::Acceleration(Acceleration::MetrePerSecondSquared))
            }
            // Angle
            "ANGLE::TURN" => Ok(UnitType::Angle(Angle::Turn)),
            "ANGLE::RADIAN" => Ok(UnitType::Angle(Angle::Radian)),
            "ANGLE::DEGREE" => Ok(UnitType::Angle(Angle::Degree)),
            "ANGLE::GRADIAN" => Ok(UnitType::Angle(Angle::Gradian)),
            // Length
            "LENGTH::MILLIMETRE" => Ok(UnitType::Length(Length::Millimetre)),
            "LENGTH::CENTIMETRE" => Ok(UnitType::Length(Length::Centimetre)),
            "LENGTH::METRE" => Ok(UnitType::Length(Length::Metre)),
            "LENGTH::KILOMETRE" => Ok(UnitType::Length(Length::Kilometre)),
            "LENGTH::INCH" => Ok(UnitType::Length(Length::Inch)),
            "LENGTH::FOOT" => Ok(UnitType::Length(Length::Foot)),
            "LENGTH::YARD" => Ok(UnitType::Length(Length::Yard)),
            "LENGTH::MILE" => Ok(UnitType::Length(Length::Mile)),
            "LENGTH::NAUTICAL_MILE" => Ok(UnitType::Length(Length::NauticalMile)),
            // Mass
            "MASS::MICROGRAM" => Ok(UnitType::Mass(Mass::Microgram)),
            "MASS::MILLIGRAM" => Ok(UnitType::Mass(Mass::Milligram)),
            "MASS::GRAM" => Ok(UnitType::Mass(Mass::Gram)),
            "MASS::KILOGRAM" => Ok(UnitType::Mass(Mass::Kilogram)),
            "MASS::METRIC_TON" => Ok(UnitType::Mass(Mass::MetricTon)),
            "MASS::OUNCE" => Ok(UnitType::Mass(Mass::Ounce)),
            "MASS::POUND" => Ok(UnitType::Mass(Mass::Pound)),
            "MASS::STONE" => Ok(UnitType::Mass(Mass::Stone)),
            "MASS::SHORT_TON" => Ok(UnitType::Mass(Mass::ShortTon)),
            "MASS::LONG_TON" => Ok(UnitType::Mass(Mass::LongTon)),
            // Time
            "TIME::NANOSECOND" => Ok(UnitType::Time(Time::Nanosecond)),
            "TIME::MICROSECOND" => Ok(UnitType::Time(Time::Microsecond)),
            "TIME::MILLISECOND" => Ok(UnitType::Time(Time::Millisecond)),
            "TIME::SECOND" => Ok(UnitType::Time(Time::Second)),
            "TIME::MINUTE" => Ok(UnitType::Time(Time::Minute)),
            "TIME::HOUR" => Ok(UnitType::Time(Time::Hour)),
            "TIME::DAY" => Ok(UnitType::Time(Time::Day)),
            "TIME::WEEK" => Ok(UnitType::Time(Time::Week)),
            "TIME::MONTH" => Ok(UnitType::Time(Time::Month)),
            "TIME::YEAR" => Ok(UnitType::Time(Time::Year)),
            "TIME::DECADE" => Ok(UnitType::Time(Time::Decade)),
            "TIME::CENTURY" => Ok(UnitType::Time(Time::Century)),
            "TIME::MILLENIUM" => Ok(UnitType::Time(Time::Millenium)),
            // Area
            "AREA::SQUARE_METRE" => Ok(UnitType::Area(Area::SquareMetre)),
            "AREA::HECTARE" => Ok(UnitType::Area(Area::Hectare)),
            "AREA::SQUARE_KILOMETRE" => Ok(UnitType::Area(Area::SquareKilometre)),
            "AREA::SQUARE_INCH" => Ok(UnitType::Area(Area::SquareInch)),
            "AREA::SQUARE_FEET" => Ok(UnitType::Area(Area::SquareFeet)),
            "AREA::SQUARE_YARD" => Ok(UnitType::Area(Area::SquareYard)),
            "AREA::ACRE" => Ok(UnitType::Area(Area::Acre)),
            "AREA::SQUARE_MILE" => Ok(UnitType::Area(Area::SquareMile)),

            // Speed
            "SPEED::METRE_PER_SECOND" => Ok(UnitType::Speed(Speed::MetrePerSecond)),
            "SPEED::KILOMETRES_PER_HOUR" => Ok(UnitType::Speed(Speed::KilometresPerHour)),
            "SPEED::FEET_PER_SECOND" => Ok(UnitType::Speed(Speed::FeetPerSecond)),
            "SPEED::MILES_PER_HOUR" => Ok(UnitType::Speed(Speed::MilesPerHour)),
            "SPEED::KNOT" => Ok(UnitType::Speed(Speed::Knot)),

            // DigitalInformation
            "DIGITALINFORMATION::BIT" => Ok(UnitType::Digitalinformation(DigitalInformation::Bit)),
            "DIGITALINFORMATION::BYTE" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Byte))
            }
            "DIGITALINFORMATION::KILOBIT" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Kilobit))
            }
            "DIGITALINFORMATION::KILOBYTE" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Kilobyte))
            }
            "DIGITALINFORMATION::MEGABIT" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Megabit))
            }
            "DIGITALINFORMATION::MEGABYTE" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Megabyte))
            }
            "DIGITALINFORMATION::GIGABIT" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Gigabit))
            }
            "DIGITALINFORMATION::GIGABYTE" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Gigabyte))
            }
            "DIGITALINFORMATION::TERABIT" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Terabit))
            }
            "DIGITALINFORMATION::TERABYTE" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Terabyte))
            }
            "DIGITALINFORMATION::PETABIT" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Petabit))
            }
            "DIGITALINFORMATION::PETABYTE" => {
                Ok(UnitType::Digitalinformation(DigitalInformation::Petabyte))
            }

            _ => Err(format!("'{}' is not a valid value for UnitType", s)),
        }
    }
}
