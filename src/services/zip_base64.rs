use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;

pub fn zip_file(input_path: &str, zip_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(input_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    let mut zip_file = File::create(zip_path)?;
    let mut zip = zip::ZipWriter::new(&mut zip_file);

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;

    zip.start_file(file_name, options)?;
    zip.write_all(&buffer)?;
    zip.finish()?;

    Ok(())
}
