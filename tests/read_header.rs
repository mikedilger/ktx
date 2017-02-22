
extern crate ktx;

use std::path::PathBuf;
use std::fs::File;
use ktx::KtxHeader;

#[test]
pub fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/particle_fire.ktxh");
    let mut f = File::open(path).unwrap();

    let header = KtxHeader::deserialize(&mut f).unwrap();

    assert_eq!(header, KtxHeader {
        gl_type: 0,
        gl_type_size: 1,
        gl_format: 0,
        gl_internal_format: 33779,
        gl_base_internal_format: 6408,
        pixel_width: 256,
        pixel_height: 256,
        pixel_depth: 0,
        number_of_array_elements: 0,
        number_of_faces: 1,
        number_of_mipmap_levels: 9,
        bytes_of_key_value_data: 32,
    });
}
