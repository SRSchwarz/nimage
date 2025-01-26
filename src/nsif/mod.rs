use self::field::Value;
use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use field::Field;
use fileheader::FileHeader;
use imagesegment::ImageSegment;
use std::collections::BTreeMap;
use std::fs::File;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use crate::nsif::field::IsEmpty;

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
    fn pretty_print(&self, exclude_empty_fields: bool) -> String
    where
        Self: Struct,
        Self: Sized,
    {
        let mut pretty = String::new();
        let reflected_self: &dyn Struct = self;
        reflected_self
            .iter_fields()
            .map(|f| f.try_downcast_ref::<Field>())
            .for_each(|f| {
                if let Some(field) = f {
                    let line = &format!("{}", field);
                    if !line.trim().is_empty() && (!exclude_empty_fields || !field.is_empty())
                    {
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
            .filter_map(|field| field.try_downcast_ref::<Field>())
            .collect();
        fields.insert(String::from("File Header"), fileheader_fields);

        for (i, image_segment) in self.image_segments.iter().enumerate() {
            let reflected_subheader: &dyn Struct = &image_segment.sub_header;
            let image_segment_fields = reflected_subheader
                .iter_fields()
                .filter_map(|field| field.try_downcast_ref::<Field>())
                .collect::<Vec<&Field>>();
            fields.insert(format!("Image Segment {}", i + 1), image_segment_fields);
        }

        fields
    }
}

impl PrettyPrint for NSIF {
    fn pretty_print(&self, include_empty_fields: bool) -> String {
        let mut pretty = String::new();
        pretty.push_str("File Header:\n");
        pretty.push_str(self.file_header.pretty_print(include_empty_fields).as_str());
        for (i, image_segment) in self.image_segments.iter().enumerate() {
            pretty.push_str("\n");
            pretty.push_str(format!("Image Segment {}:\n", i+1).as_str());
            pretty.push_str(image_segment.pretty_print(include_empty_fields).as_str());
        }
        pretty
    }
}

pub fn parse_string_from_bytes(vec: &Vec<u8>) -> Result<String, FromUtf8Error> {
    String::from_utf8(vec.clone())
}

pub fn parse_unsigned_integers_from_byte(vec: &[u8]) -> String {
    vec.iter()
        .map(|byte| format!("0x{:02x}", byte))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn parse_number_from_bytes(vec: &Vec<u8>) -> Result<i32, Box<dyn std::error::Error>> {
    let s = parse_string_from_bytes(vec)?;
    parse_number_from_string(&s).map_err(Into::into)
}

pub fn parse_number_from_string(s: &str) -> Result<i32, ParseIntError> {
    let s = s.trim_start_matches('0').to_owned();
    if s.is_empty() {
        Ok(0)
    } else {
        s.parse::<i32>()
    }
}
