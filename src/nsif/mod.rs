use std::{fmt::Display, fs::File};

pub mod field;
pub mod fileheader;
pub mod imagesegment;

use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use field::Field;
use field::FieldValue;
use fileheader::FileHeader;
use imagesegment::ImageSegment;

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
            .map(|field| field.downcast_ref::<Field>().unwrap())
            .for_each(|field| {
                let line = &format!("{}", field);
                if !line.trim().is_empty() {
                    pretty.push_str(&format!("    {}\n", line));
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
    pub fn parse(file: &File) -> Result<NSIF, Box<dyn std::error::Error>> {
        let file_header = FileHeader::parse(file)?;
        let mut image_segments = Vec::new();

        if let (
            FieldValue::Multiple(image_segment_subheader_lengths),
            FieldValue::Multiple(image_segment_lengths),
        ) = (&file_header.lishs.value, &file_header.lis.value)
        {
            for (subheader_length, segment_length) in image_segment_subheader_lengths
                .iter()
                .zip(image_segment_lengths.iter())
            {
                image_segments.push(ImageSegment::parse(
                    file,
                    parse_number(subheader_length)?,
                    parse_number(segment_length)?,
                )?);
            }
        }

        Ok(NSIF {
            file_header,
            image_segments,
        })
    }

    pub fn fields(&self) -> Vec<&Field> {
        let mut fields = Vec::new();
        let reflected_fileheader: &dyn Struct = &self.file_header;
        fields.extend(
            reflected_fileheader
                .iter_fields()
                .map(|field| field.downcast_ref::<Field>().unwrap()),
        );
        for image_segment in &self.image_segments {
            let reflected_subheader: &dyn Struct = &image_segment.sub_header;
            fields.extend(
                reflected_subheader
                    .iter_fields()
                    .map(|field| field.downcast_ref::<Field>().unwrap()),
            );
        }

        fields
    }
}

impl Display for NSIF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", "File Header:")?;
        write!(f, "{}\n", self.file_header)?;
        for (i, image_segment) in self.image_segments.iter().enumerate() {
            write!(f, "{} {}:\n", "Image Segment", (i + 1).to_string())?;
            write!(f, "{}", image_segment)?;
        }
        Ok(())
    }
}

pub fn parse_string(vec: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    String::from_utf8(vec.clone()).map_err(Into::into)
}

fn parse_number(vec: &Vec<u8>) -> Result<i32, Box<dyn std::error::Error>> {
    let s = parse_string(vec)?.trim_start_matches('0').to_owned();
    if s.is_empty() {
        Ok(0)
    } else {
        s.parse::<i32>().map_err(Into::into)
    }
}
