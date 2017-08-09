use std::fs;
use std::io::Read;

#[derive(Default, Debug)]
pub struct File {
    path: String,
    header_size: u32,
    total_size: u32, 
    version: u32,
}

impl File {
    pub fn new(file: String) -> File {
        File {
            path: file,
            header_size: 0,
            total_size: 0,
            version: 0
        }
    }

    pub fn process(&self) -> Result<bool, String> {
        println!("Processing: {}", self.path);

        // Read the file to a buffer:
        let mut file_data: Vec<u8> = Vec::new();
        let mut handle = fs::File::open(&self.path).expect("Failed to find ITL file: {}");
        handle.read_to_end(&mut file_data).expect("Failed to fully read ITL file");

        // Validate HDFM file signature:
        let hdfm_bytes = &file_data[0..4];
        if hdfm_bytes != &[104, 100, 102, 109] {
            return Result::Err("HDFM file signature mismatch!".to_string());
        }

        Result::Ok(true)
    }
}