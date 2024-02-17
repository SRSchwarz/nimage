use std::path::PathBuf;

use super::imagesegment::ImageSegment;

use jpeg_encoder::{ColorType, Encoder};
pub fn export_to_jpeg(
    image_segment: &ImageSegment,
    path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = image_segment.data.clone();
    let mut output_path = path.clone();
    output_path.set_extension("jpg");

    let encoder = Encoder::new_file(output_path, 100)?;
    encoder.encode(&data, 1024, 913, ColorType::Rgb)?;
    Ok(())
}
