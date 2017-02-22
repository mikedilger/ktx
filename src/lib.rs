
extern crate byteorder;

use std::io;

use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

const FILE_IDENTIFIER: [u8; 12] = [0xAB, 0x4B, 0x54, 0x58,
                                   0x20, 0x31, 0x31, 0xBB,
                                   0x0D, 0x0A, 0x1A, 0x0A];

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KtxHeader {
    pub gl_type: u32,
    pub gl_type_size: u32,
    pub gl_format: u32,
    pub gl_internal_format: u32,
    pub gl_base_internal_format: u32,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub pixel_depth: u32,
    pub number_of_array_elements: u32,
    pub number_of_faces: u32,
    pub number_of_mipmap_levels: u32,
    pub bytes_of_key_value_data: u32,
}

// fixme: in Ktx itself, store the endianness.  Ktx itself should not be repr(C) as it
//        will not exactly represent the data.

impl KtxHeader {
    pub fn deserialize<R: io::Read>(source: &mut R) -> Result<KtxHeader, io::Error>
    {
        // Read identifier
        let mut buffer: [u8; 12] = [0; 12];
        source.read_exact(&mut buffer)?;
        if buffer != FILE_IDENTIFIER {
            return Err(io::Error::new(io::ErrorKind::Other, "File is not a KTX file."));
        }

        // Read endianness
        let mut buffer: [u8; 4] = [0; 4];
        source.read_exact(&mut buffer)?;
        let little_endian: bool = match buffer[0] {
            0x01 => true,
            0x04 => false,
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Invalid KTX file.")),
        };

        if little_endian {
            KtxHeader::deserialize_little_endian(source)
        } else {
            KtxHeader::deserialize_big_endian(source)
        }
    }

    fn deserialize_little_endian<R: io::Read>(source: &mut R) -> Result<KtxHeader, io::Error>
    {
        Ok(KtxHeader {
            gl_type: source.read_u32::<LittleEndian>()?,
            gl_type_size: source.read_u32::<LittleEndian>()?,
            gl_format: source.read_u32::<LittleEndian>()?,
            gl_internal_format: source.read_u32::<LittleEndian>()?,
            gl_base_internal_format: source.read_u32::<LittleEndian>()?,
            pixel_width: source.read_u32::<LittleEndian>()?,
            pixel_height: source.read_u32::<LittleEndian>()?,
            pixel_depth: source.read_u32::<LittleEndian>()?,
            number_of_array_elements: source.read_u32::<LittleEndian>()?,
            number_of_faces: source.read_u32::<LittleEndian>()?,
            number_of_mipmap_levels: source.read_u32::<LittleEndian>()?,
            bytes_of_key_value_data: source.read_u32::<LittleEndian>()?,
        })
    }

    fn deserialize_big_endian<R: io::Read>(source: &mut R) -> Result<KtxHeader, io::Error>
    {
        Ok(KtxHeader {
            gl_type: source.read_u32::<BigEndian>()?,
            gl_type_size: source.read_u32::<BigEndian>()?,
            gl_format: source.read_u32::<BigEndian>()?,
            gl_internal_format: source.read_u32::<BigEndian>()?,
            gl_base_internal_format: source.read_u32::<BigEndian>()?,
            pixel_width: source.read_u32::<BigEndian>()?,
            pixel_height: source.read_u32::<BigEndian>()?,
            pixel_depth: source.read_u32::<BigEndian>()?,
            number_of_array_elements: source.read_u32::<BigEndian>()?,
            number_of_faces: source.read_u32::<BigEndian>()?,
            number_of_mipmap_levels: source.read_u32::<BigEndian>()?,
            bytes_of_key_value_data: source.read_u32::<BigEndian>()?,
        })
    }
}

/*
pub struct KeyValuePairData {
}

pub struct KeyValuePairDatum {
    key_and_value_byte_size: u32,
    key_and_value_bytes: [u8],
    value_padding: [u8],
}
 */

//impl Ktx {
    /*
    pub fn new(gl_type: u32) -> Ktx
    {
        Ktx {
            indentifier: FILE_IDENTIFIER,
            endianness: 0x04030201, // specifies our endianness, whatever that is.
            gl_type: 0,
            gl_type_size: 0,
            gl_format: 
        }
    }
     */
//}
