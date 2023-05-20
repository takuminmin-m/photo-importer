use std::io::{ BufRead, Seek, Read };

const TIFF_MARKER: [u8; 12] = [
    0xff, 0xd8, 0xff, 0xe1, 0xc5, 0xfe, 0x45, 0x78, 0x69, 0x66, 0x00, 0x00
];
const TIFF_MARKER_LEN: usize = 12;


pub fn find_tiff_marker<R: BufRead + Seek>(buf_reader: &mut R) -> Result<u64, &'static str> {
    let mut buf = Vec::new();
    buf_reader.seek(std::io::SeekFrom::Start(0));
    buf_reader.by_ref().take(65536).read_to_end(&mut buf);

    for i in 0..buf.len()-TIFF_MARKER_LEN {
        let mut marker_found = true;
        for j in 0..TIFF_MARKER_LEN {
            if buf[i+j] != TIFF_MARKER[j] {
                marker_found = false;
                break;
            }
        }

        if !marker_found {
            continue;
        }

        return Ok(i as u64);
    }

    return Err("Tiff marker not found.");
}
