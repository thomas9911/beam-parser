use deku::prelude::*;

// from lib/compiler/src/beam_opcodes.hrl
// -define(tag_u, 0).
// -define(tag_i, 1).
// -define(tag_a, 2).
// -define(tag_x, 3).
// -define(tag_y, 4).
// -define(tag_f, 5).
// -define(tag_h, 6).
// -define(tag_z, 7).

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum Tag {
    #[deku(id = 0)]
    Literal(TagValue),
    #[deku(id = 1)]
    Integer(TagValue),
    #[deku(id = 2)]
    Atom(TagValue),
    #[deku(id = 3)]
    XRegister(TagValue),
    #[deku(id = 4)]
    YRegister(TagValue),
    #[deku(id = 5)]
    Label(TagValue),
    #[deku(id = 6)]
    Character(TagValue),
    #[deku(id = 7)]
    Extended(ExtendedTag),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 1)]
pub enum TagValue {
    #[deku(id = 0)]
    Small(SmallInlineValue),
    #[deku(id = 1)]
    Normal(LargerTagValue),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 1)]
pub enum LargerTagValue {
    #[deku(id = 0)]
    Normal(NormalInlineValue),
    #[deku(id = 1)]
    Large(LargeValue),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, DekuSize)]
pub struct SmallInlineValue {
    #[deku(bits = 4)]
    data: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, DekuSize)]
pub struct NormalInlineValue {
    #[deku(bits = 11)]
    data: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum LargeValue {
    #[deku(id = "0b111")]
    External,
    #[deku(id_pat = "_")]
    LargeInline {
        /// range from 2-8, 0,1,2 are skipped because they are captured with the other encodings
        size: u8,
        #[deku(ctx = "*size")]
        val: LargeInlineValue,
    },
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "size: u8")]
pub struct LargeInlineValue {
    #[deku(count = "size + 2")]
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, DekuSize)]
#[deku(id_type = "u8", bits = 5)]
pub enum ExtendedTag {
    #[deku(id = "0b00010")]
    List,
    #[deku(id = "0b00100")]
    FloatingPoint,
    #[deku(id = "0b00110")]
    AllocationList,
    #[deku(id = "0b01000")]
    Literal,
    #[deku(id = "0b01010")]
    TypeHint,
    #[deku(id_pat = "_")]
    Other(u8),
}

// #[test]
// fn parse_tags_test() {
//     let (_, tag) = Tag::from_bytes((&[0b00000011], 0)).unwrap();
//     assert_eq!(tag, Tag::Literal);
//     let (_, tag) = Tag::from_bytes((&[0b00100011], 0)).unwrap();
//     assert_eq!(tag, Tag::Integer);
//     let (_, tag) = Tag::from_bytes((&[0b01000011], 0)).unwrap();
//     assert_eq!(tag, Tag::Atom);
//     let (_, tag) = Tag::from_bytes((&[0b11000011], 0)).unwrap();
//     assert_eq!(tag, Tag::Character);
//     let (_, tag) = Tag::from_bytes((&[0b11101000], 0)).unwrap();
//     assert_eq!(tag, Tag::Extended(ExtendedTag::Literal));
//     let (_, tag) = Tag::from_bytes((&[0b11100010], 0)).unwrap();
//     assert_eq!(tag, Tag::Extended(ExtendedTag::List));
// }

#[test]
fn parse_external_value_test() {
    let bytes = &[0b00011001, 123, 125, 127];

    let (_, tag) = Tag::from_bytes((bytes, 0)).unwrap();

    let expected = Tag::Literal(TagValue::Normal(LargerTagValue::Large(
        LargeValue::LargeInline {
            size: 1,
            val: LargeInlineValue {
                data: vec![123, 125, 127],
            },
        },
    )));

    assert_eq!(expected, tag);
}
