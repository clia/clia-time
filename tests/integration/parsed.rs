use std::num::{NonZeroU16, NonZeroU8};

use time::format_description::modifier::WeekNumberRepr;
use time::format_description::{Component, FormatItem};
use time::parsing::Parsed;
use time::{error, Month, Time, Weekday};

#[test]
fn getters_setters() {
    macro_rules! getters_setters {
        ($($setter:ident $getter:ident $value:expr;)*) => {$(
            let mut parsed = Parsed::new();
            parsed.$setter($value);
            assert_eq!(parsed.$getter(), Some($value));
        )*};
    }

    getters_setters! {
        set_year year 5;
        set_year_last_two year_last_two 5;
        set_iso_year iso_year 5;
        set_iso_year_last_two iso_year_last_two 5;
        set_month month Month::May;
        set_sunday_week_number sunday_week_number 5;
        set_monday_week_number monday_week_number 5;
        set_iso_week_number iso_week_number NonZeroU8::new(5).expect("valid value");
        set_weekday weekday Weekday::Monday;
        set_ordinal ordinal NonZeroU16::new(5).expect("valid value");
        set_day day NonZeroU8::new(5).expect("valid value");
        set_hour_24 hour_24 5;
        set_hour_12 hour_12 NonZeroU8::new(5).expect("valid value");
        set_hour_12_is_pm hour_12_is_pm true;
        set_minute minute 5;
        set_second second 5;
        set_subsecond subsecond 5;
        set_offset_hour offset_hour 5;
        set_offset_minute offset_minute 5;
        set_offset_second offset_second 5;
    }
}

#[test]
fn builder_methods() {
    let parsed = Parsed::new()
        .with_year(5)
        .and_then(|parsed| parsed.with_year_last_two(5))
        .and_then(|parsed| parsed.with_iso_year(5))
        .and_then(|parsed| parsed.with_iso_year_last_two(5))
        .and_then(|parsed| parsed.with_month(Month::May))
        .and_then(|parsed| parsed.with_sunday_week_number(5))
        .and_then(|parsed| parsed.with_monday_week_number(5))
        .and_then(|parsed| parsed.with_iso_week_number(NonZeroU8::new(5).expect("valid value")))
        .and_then(|parsed| parsed.with_weekday(Weekday::Monday))
        .and_then(|parsed| parsed.with_ordinal(NonZeroU16::new(5).expect("valid value")))
        .and_then(|parsed| parsed.with_day(NonZeroU8::new(5).expect("valid value")))
        .and_then(|parsed| parsed.with_hour_24(5))
        .and_then(|parsed| parsed.with_hour_12(NonZeroU8::new(5).expect("valid value")))
        .and_then(|parsed| parsed.with_hour_12_is_pm(true))
        .and_then(|parsed| parsed.with_minute(5))
        .and_then(|parsed| parsed.with_second(5))
        .and_then(|parsed| parsed.with_subsecond(5))
        .and_then(|parsed| parsed.with_offset_hour(5))
        .and_then(|parsed| parsed.with_offset_minute(5))
        .and_then(|parsed| parsed.with_offset_second(5))
        .expect("all values are valid");

    assert_eq!(parsed.year(), Some(5));
    assert_eq!(parsed.year_last_two(), Some(5));
    assert_eq!(parsed.iso_year(), Some(5));
    assert_eq!(parsed.iso_year_last_two(), Some(5));
    assert_eq!(parsed.month(), Some(Month::May));
    assert_eq!(parsed.sunday_week_number(), Some(5));
    assert_eq!(parsed.monday_week_number(), Some(5));
    assert_eq!(
        parsed.iso_week_number(),
        Some(NonZeroU8::new(5).expect("valid value"))
    );
    assert_eq!(parsed.weekday(), Some(Weekday::Monday));
    assert_eq!(
        parsed.ordinal(),
        Some(NonZeroU16::new(5).expect("valid value"))
    );
    assert_eq!(parsed.day(), Some(NonZeroU8::new(5).expect("valid value")));
    assert_eq!(parsed.hour_24(), Some(5));
    assert_eq!(
        parsed.hour_12(),
        Some(NonZeroU8::new(5).expect("valid value"))
    );
    assert_eq!(parsed.hour_12_is_pm(), Some(true));
    assert_eq!(parsed.minute(), Some(5));
    assert_eq!(parsed.second(), Some(5));
    assert_eq!(parsed.subsecond(), Some(5));
    assert_eq!(parsed.offset_hour(), Some(5));
    assert_eq!(parsed.offset_minute(), Some(5));
    assert_eq!(parsed.offset_second(), Some(5));
}

#[test]
fn single_item_parse() {
    assert!(Time::parse("a", &FormatItem::Literal(b"a")).is_err());
    assert!(Time::parse("b", &FormatItem::Literal(b"a")).is_err());
}

#[test]
fn component_err() {
    macro_rules! input_or_empty {
        () => {
            b""
        };
        ($input:expr) => {
            $input
        };
    }
    macro_rules! assert_invalid_component {
        ($component_name:expr, $component:expr $(, $input:expr)?) => {{
            let mut parsed = Parsed::new();
            assert_eq!(
                parsed.parse_component(input_or_empty!($($input)?), $component),
                Err(error::ParseFromDescription::InvalidComponent(
                    $component_name
                ))
            );
        }};
    }

    assert_invalid_component!("day", Component::Day(<_>::default()));
    assert_invalid_component!("month", Component::Month(<_>::default()));
    assert_invalid_component!("ordinal", Component::Ordinal(<_>::default()));
    assert_invalid_component!("weekday", Component::Weekday(<_>::default()));
    assert_invalid_component!("week number", Component::WeekNumber(<_>::default()));
    assert_invalid_component!("year", Component::Year(<_>::default()));
    assert_invalid_component!("minute", Component::Minute(<_>::default()));
    assert_invalid_component!("period", Component::Period(<_>::default()));
    assert_invalid_component!("second", Component::Second(<_>::default()));
    assert_invalid_component!("subsecond", Component::Subsecond(<_>::default()));
    assert_invalid_component!("offset hour", Component::OffsetHour(<_>::default()));
    assert_invalid_component!("offset minute", Component::OffsetMinute(<_>::default()));
    assert_invalid_component!("offset second", Component::OffsetSecond(<_>::default()));

    assert_invalid_component!(
        "week number",
        Component::WeekNumber(modifier!(WeekNumber {
            repr: WeekNumberRepr::Iso,
        })),
        b"00"
    );
    assert_invalid_component!(
        "hour",
        Component::Hour(modifier!(Hour {
            is_12_hour_clock: true,
        })),
        b"00"
    );
}
