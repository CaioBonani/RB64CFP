use std::fs::File;
use std::io::{Cursor, Read, Write};
use zip::{ZipWriter, write::FileOptions};

pub fn zip_file(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let file_name = std::path::Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let mut buffer = Cursor::new(Vec::new());
    {
        let mut zip = ZipWriter::new(&mut buffer);

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o644);

        let mut file_data = Vec::new();
        File::open(path)?.read_to_end(&mut file_data)?;

        zip.start_file(file_name, options)?;
        zip.write_all(&file_data)?;

        zip.finish()?; // importante!
    }

    Ok(buffer.into_inner())
}
