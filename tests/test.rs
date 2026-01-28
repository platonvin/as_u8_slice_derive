#![feature(macro_derive)]

use as_u8_slice_derive::AsU8Slice;

#[derive(AsU8Slice)]
#[repr(C)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(as_u8_slice_derive::AsU8Slice)]
#[repr(C)]
struct TuplePoint(u16, u16);

#[test]
fn test_struct() {
    let p = Point {
        x: 0xC001BABE,
        y: 0xFEE1DEAD,
    };
    let slice = p.as_u8_slice();

    assert_eq!(slice.len(), 8);

    let mut expected = Vec::new();
    expected.extend_from_slice(&p.x.to_ne_bytes());
    expected.extend_from_slice(&p.y.to_ne_bytes());

    assert_eq!(slice, expected.as_slice());
}

#[test]
fn test_tuple() {
    let t = TuplePoint(0x0102, 0x0304);
    let slice = t.as_u8_slice();

    assert_eq!(slice.len(), 4);

    let mut expected = Vec::new();
    expected.extend_from_slice(&t.0.to_ne_bytes());
    expected.extend_from_slice(&t.1.to_ne_bytes());

    assert_eq!(slice, expected.as_slice());
}
