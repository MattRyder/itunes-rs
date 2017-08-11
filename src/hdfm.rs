use std::fs;
use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Default, Debug)]
pub struct File {
    path: String,
    header_size: u32,
    total_size: u32, 
    version: String,
}

impl File {
    pub fn new(file: String) -> File {
        File {
            path: file.trim().to_string(),
            header_size: 0,
            total_size: 0,
            version: String::new(),
        }
    }

    pub fn process(&mut self) -> Result<bool, String> {
        println!("Processing: {}", self.path);

        // Read the file to a buffer:
        let mut file_data: Vec<u8> = Vec::new();
        let mut handle = fs::File::open(&self.path).expect("Failed to find ITL file");
        handle.read_to_end(&mut file_data).expect("Failed to fully read ITL file");

        // Validate HDFM file signature:
        let hdfm_bytes = &file_data[0..4];
        if hdfm_bytes != &[104, 100, 102, 109] {
            return Result::Err("HDFM file signature mismatch!".to_string());
        }

        self.header_size = File::read_u32(&file_data, 0x04);
        self.total_size = File::read_u32(&file_data, 0x08);

        let version_length = file_data[0x10];
        let version_data = &file_data[0x11..(0x11+version_length as usize)];
        self.version = String::from_utf8(version_data.to_vec()).expect("Failed to read HDFM header version");

        // Calculate how much to jump over for second unknown:
        let unk_size = self.header_size - (version_length as u32) - 17;
        println!("Unk size: 0x{0:x}", unk_size);

        Result::Ok(true)
    }

    fn read_u32(buf: &Vec<u8>, offset: usize) -> u32 {
        File::read(buf, offset, 4)
    }

    fn read(buf: &Vec<u8>, offset: usize, length: usize) -> u32 {
        let slice = &buf[offset..(offset + length)];
        let mut cursor = Cursor::new(slice);
        cursor.read_u32::<BigEndian>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use hdfm;

    #[test]
    fn can_create_file() {
        let file_path = String::from("./test_data/blank.itl");
        let expected = file_path.clone();
        let library = hdfm::File::new(file_path);
        
        assert_eq!(library.path, expected);
    }

    #[test]
    fn can_process() {
        let file_path = String::from("./test_data/blank.itl");
        let expected = file_path.clone();
        let mut library = hdfm::File::new(file_path);

        let res = library.process();
        assert_eq!(res, Ok(true));
    }
}