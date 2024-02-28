use super::imagesegment::ImageSegment;
use std::path::PathBuf;

use jpeg_encoder::{ColorType, Encoder};
pub fn export_to_jpeg(
    image_segment: &ImageSegment,
    path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = image_segment.data.clone();
    let mut output_path = path.clone();
    output_path.set_extension("jpg");

    let encoder = Encoder::new_file(output_path, 100)?;
    let (height, width) = image_segment.dimensions();
    encoder.encode(&data, width as _, height as _, ColorType::Rgb)?;
    Ok(())
}
