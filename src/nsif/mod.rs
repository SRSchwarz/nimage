use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use field::Field;
use fileheader::FileHeader;
use imagesegment::ImageSegment;
use std::collections::BTreeMap;
use std::{fmt::Display, fs::File};

use self::field::Value;

pub mod error;
pub mod export;
pub mod field;
pub mod fileheader;
pub mod imagesegment;

#[derive(Debug, Reflect)]
pub struct NSIF {
    pub file_header: FileHeader,
    pub image_segments: Vec<ImageSegment>,
    /*
    graphic_segments: Vec<GraphicSegment>,
    reserved_segments: Vec<ReservedSegment>,
    text_segments: Vec<TextSegment>,
    data_extension_segments: Vec<DataExtensionSegment>,
    reserved_extension_segments: Vec<ReservedExtensionSegment>,
    */
}

pub trait PrettyPrint {
    fn pretty_print(&self) -> String
    where
        Self: Struct,
        Self: Sized,
    {
        let mut pretty = String::new();
        let reflected_self: &dyn Struct = self;
        reflected_self
            .iter_fields()
            .map(|f| f.downcast_ref::<Field>())
            .for_each(|f| {
                if let Some(field) = f {
                    let line = &format!("{}", field);
                    if !line.trim().is_empty() {
                        pretty.push_str(&format!("    {}\n", line));
                    }
                }
            });
        pretty.pop();
        pretty
    }
}
/*
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
    pub fn parse(file: &File) -> Result<Self, Box<dyn std::error::Error>> {
        let file_header = FileHeader::parse(file)?;
        let mut image_segments = Vec::new();

        if let (
            Value::MultipleNumeric(image_segment_subheader_lengths),
            Value::MultipleNumeric(image_segment_lengths),
        ) = (&file_header.lishs.value, &file_header.lis.value)
        {
            for (subheader_length, segment_length) in image_segment_subheader_lengths
                .iter()
                .zip(image_segment_lengths.iter())
            {
                image_segments.push(ImageSegment::parse(
                    file,
                    parse_number_from_string(&subheader_length.value)?,
                    parse_number_from_string(&segment_length.value)?,
                )?);
            }
        }

        Ok(NSIF {
            file_header,
            image_segments,
        })
    }

    pub fn fields(&self) -> BTreeMap<String, Vec<&Field>> {
        let mut fields = BTreeMap::new();
        let reflected_fileheader: &dyn Struct = &self.file_header;
        let fileheader_fields = reflected_fileheader
            .iter_fields()
            .filter_map(|field| field.downcast_ref::<Field>())
            .collect();
        fields.insert(String::from("File Header"), fileheader_fields);

        for (i, image_segment) in self.image_segments.iter().enumerate() {
            let reflected_subheader: &dyn Struct = &image_segment.sub_header;
            let image_segment_fields = reflected_subheader
                .iter_fields()
                .filter_map(|field| field.downcast_ref::<Field>())
                .collect::<Vec<&Field>>();
            fields.insert(format!("Image Segment {}", i + 1), image_segment_fields);
        }

        fields
    }
}

impl Display for NSIF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "File Header:")?;
        writeln!(f, "{}", self.file_header)?;
        for (i, image_segment) in self.image_segments.iter().enumerate() {
            writeln!(f, "Image Segment {}:", (i + 1))?;
            write!(f, "{}", image_segment)?;
        }
        Ok(())
    }
}

pub fn parse_string_from_bytes(vec: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    String::from_utf8(vec.clone()).map_err(Into::into)
}

pub fn parse_unsigned_integers_from_byte(vec: &[u8]) -> String {
    vec.iter()
        .map(|byte| format!("0x{:02x}", byte))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn parse_number_from_bytes(vec: &Vec<u8>) -> Result<i32, Box<dyn std::error::Error>> {
    let s = parse_string_from_bytes(vec)?;
    parse_number_from_string(&s)
}

pub fn parse_number_from_string(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let s = s.trim_start_matches('0').to_owned();
    if s.is_empty() {
        Ok(0)
    } else {
        s.parse::<i32>().map_err(Into::into)
    }
}
