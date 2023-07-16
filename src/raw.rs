use std::io::{ BufRead, Seek, Read };

const TIFF_MARKERS: [[u8; 12]; 2] = [
    [0xff, 0xd8, 0xff, 0xe1, 0xc5, 0xfe, 0x45, 0x78, 0x69, 0x66, 0x00, 0x00],
    [0xff, 0xd8, 0xff, 0xe1, 0xff, 0xa8, 0x45, 0x78, 0x69, 0x66, 0x00, 0x00],
];
const TIFF_MARKER_LEN: usize = 12;
const TIFF_MARKET_TYPE_NUM: usize = 2;


pub fn find_tiff_marker<R: BufRead + Seek>(buf_reader: &mut R) -> Result<u64, &'static str> {
    let mut buf = Vec::new();
    buf_reader.seek(std::io::SeekFrom::Start(0));
    buf_reader.by_ref().take(65536).read_to_end(&mut buf);

    for tiff_marker_type_i in 0..TIFF_MARKET_TYPE_NUM {
        for i in 0..buf.len()-TIFF_MARKER_LEN {
            let mut marker_found = true;
            for j in 0..TIFF_MARKER_LEN {
                if buf[i+j] != TIFF_MARKERS[tiff_marker_type_i][j] {
                    marker_found = false;
                    break;
                }
            }

            if !marker_found {
                continue;
            }

            return Ok(i as u64);
        }
    }

    return Err("Tiff marker not found.");
}
