use std::convert::TryFrom;

use time::format_description::{modifier, Component, FormatItem};

#[test]
fn format_item_component_conversions() {
    let component = Component::Year(modifier::Year::default());
    let item = FormatItem::from(component);
    assert!(matches!(item, FormatItem::Component(inner) if inner == component));
    assert_eq!(Component::try_from(item), Ok(component));
    assert!(Component::try_from(FormatItem::Literal(b"")).is_err());
    assert!(<&[FormatItem<'_>]>::try_from(FormatItem::Literal(b"")).is_err());
}

#[test]
fn format_item_compound_conversions() {
    let compound = &[FormatItem::Literal(b"")][..];
    let item = FormatItem::from(compound);
    assert!(matches!(item, FormatItem::Compound(inner) if inner == compound));
    assert_eq!(<&[FormatItem<'_>]>::try_from(item), Ok(compound));
}

#[test]
fn format_item_equality() {
    let component = Component::Year(modifier::Year::default());
    let compound = &[FormatItem::Literal(b"")][..];
    let component_item = FormatItem::from(component);
    let compound_item = FormatItem::from(compound);

    assert_eq!(component, component_item);
    assert_eq!(component_item, component);
    assert_eq!(compound, compound_item);
    assert_eq!(compound_item, compound);
}
