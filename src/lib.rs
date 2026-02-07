use std::borrow::Cow;

use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite, DekuSize)]
#[deku(endian = "big")]
#[deku(magic = b"FOR1")]
struct BeamHeader {
    size: u32,
    #[deku(assert_eq = "*b\"BEAM\"")]
    _beam_magic: [u8; 4],
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
struct GenericBeamChunk {
    // name: [u8; 4],
    #[deku(update = "self.data.len()")]
    size: u32,
    #[deku(count = "(4 * ((size+3) / 4))")]
    data: Vec<u8>,
}

impl GenericBeamChunk {
    fn data(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.data)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "[u8; 4]")]
enum BeamChunkType {
    #[deku(id = "[b'A', b't', b'U', b'8']")]
    AtU8(AtomChunk),
    #[deku(id = "[b'E', b'x', b'p', b'T']")]
    Export(ExportChunk),
    #[deku(id_pat = "_")]
    Other([u8; 4]),
}

impl BeamChunkType {
    fn name(&self) -> &str {
         match self {
            BeamChunkType::AtU8(_) => "AtU8",
            BeamChunkType::Export(_) => "ExpT",
            BeamChunkType::Other(id) => unsafe {std::str::from_utf8_unchecked(id.as_slice())},
        }
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
struct AtomChunk {
    size: u32,
    number_of_atoms: u32,
    #[deku(count = "number_of_atoms")]
    #[deku(pad_bytes_after = "(4 * ((size+3) / 4)) - size")]
    atoms: Vec<AtomChunkItem>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
struct AtomChunkItem {
    length: u8,
    #[deku(count = "length")]
    name: Vec<u8>,
}

impl AtomChunkItem {
    pub fn name(&self) -> &str {
        // technically this should aways succeed, but maybe for external data we probably need to verify it (or use utf8_lossy)
        unsafe {std::str::from_utf8_unchecked(&self.name)}
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
struct ExportChunk {
    size: u32,
    export_count: u32,
    #[deku(count = "export_count")]
    #[deku(pad_bytes_after = "(4 * ((size+3) / 4)) - size")]
    functions: Vec<ExportChunkItem>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
struct ExportChunkItem {
    function_index: u32,
    arity: u32,
    label: u32,
}

fn parse_beam(bytes: &[u8]) {
    let (mut needle, header) = BeamHeader::from_bytes((&bytes, 0)).unwrap();
    dbg!(header);
    
    while let Ok((mut next_needle, chunk)) = BeamChunkType::from_bytes(needle) {

        match chunk {
            BeamChunkType::AtU8(atoms) => {
                dbg!(atoms.number_of_atoms);
                for atom in atoms.atoms {
                    dbg!(atom.name());
                }
            }
            BeamChunkType::Export(export) => {
                dbg!(export.export_count);
                for func in export.functions {
                    dbg!(func.function_index);
                    dbg!(func.arity);
                    dbg!(func.label);
                }
            }
            // continue: https://blog.stenmans.org/theBeamBook/#import_table_chunk
            BeamChunkType::Other(id) => {
                let tmp = GenericBeamChunk::from_bytes(next_needle).unwrap();
                dbg!(chunk.name());
                next_needle = tmp.0;
                let chunk = tmp.1;
                dbg!(chunk.size);
                dbg!(chunk.data());
                // dbg!(chunk);
            }
        }

        needle = next_needle;
    }
}

#[test]
fn parse_beam_header_correct() {
    let data = b"FOR1\0\0\x01\x01BEAM";
    let (_, a) = BeamHeader::from_bytes((data, 0)).unwrap();

    assert_eq!(a.size, 257);
    assert_eq!(BeamHeader::SIZE_BYTES.unwrap(), 12);
}

#[test]
fn parse_beam_header_invalid_beam_magic() {
    let data = b"FOR1\0\0\x01\x01BEAx";
    BeamHeader::from_bytes((data, 0)).unwrap_err();
}

#[test]
fn parse_beam_test() {
    let beam_bytes = std::fs::read("Elixir.Test.beam").unwrap();
    parse_beam(&beam_bytes);


    panic!()
}