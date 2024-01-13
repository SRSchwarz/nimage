use std::{fmt::Display, fs::File};

mod fileheader;

use fileheader::FileHeader;

#[derive(Debug)]
pub struct NSIF {
    file_header: FileHeader,
    /*
    image_segments: Vec<ImageSegment>,
    graphic_segments: Vec<GraphicSegment>,
    reserved_segments: Vec<ReservedSegment>,
    text_segments: Vec<TextSegment>,
    data_extension_segments: Vec<DataExtensionSegment>,
    reserved_extension_segments: Vec<ReservedExtensionSegment>,
    */
}

/*
#[derive(Debug)]
struct ImageSegment {
    sub_header: ImageSubheader,
    data: Vec<u8>,
}

#[derive(Debug)]
struct ImageSubheader {}

#[derive(Debug)]
struct GraphicSegment {}

#[derive(Debug)]
struct ReservedSegment {}

#[derive(Debug)]
struct TextSegment {}

#[derive(Debug)]
struct DataExtensionSegment {}

#[derive(Debug)]
struct ReservedExtensionSegment {}
*/
impl NSIF {
    pub fn parse(file: &File) -> Result<NSIF, Box<dyn std::error::Error>> {
        let file_header = FileHeader::parse(file)?;
        Ok(NSIF { file_header })
    }
}

impl Display for NSIF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "File Header:\n")?;
        write!(f, "{}", self.file_header)
    }
}
