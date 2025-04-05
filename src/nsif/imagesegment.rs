use super::{parse_number_from_bytes, parse_string_from_bytes, PrettyPrint};
use crate::nsif::error::NsifError;
use crate::nsif::field::{Field, Value};
use crate::nsif::parse_number_from_string;
use bevy_reflect::Reflect;
use jpeg2k::ImagePixelData;
use std::cmp::max;
use std::vec;
use std::{fs::File, io::Read};
use zune_jpeg::errors::DecodeErrors;
use zune_jpeg::JpegDecoder;

#[derive(Debug, Reflect)]
pub struct ImageSegment {
    pub sub_header: ImageSubheader,
    pub data: Vec<u8>,
}
impl ImageSegment {
    pub fn parse(
        mut file: &File,
        _subheader_length: i32,
        segment_length: i32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let sub_header = ImageSubheader::parse(file)?;
        let mut data = vec![0; segment_length as usize];
        file.read_exact(&mut data)?;
        Ok(ImageSegment { sub_header, data })
    }

    pub fn dimensions(&self) -> Result<(i32, i32), Box<dyn std::error::Error>> {
        if let (Value::SingleNumeric(height), Value::SingleNumeric(width)) =
            (&self.sub_header.nrows.value, &self.sub_header.ncols.value)
        {
            return Ok((
                parse_number_from_string(&height.value)?,
                parse_number_from_string(&width.value)?,
            ));
        }
        Err(Box::new(NsifError::InvalidDimensions))
    }

    pub fn as_rgb(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if let Value::SingleAlphanumeric(ic) = &self.sub_header.ic.value {
            return match ic.value.as_str() {
                "NC" => self.handle_nc().map_err(Into::into),
                "C3" => self.handle_c3().map_err(Into::into),
                "C8" => self.handle_c8().map_err(Into::into),
                _ => Err(Box::new(NsifError::IcNotSupported)),
            };
        }
        Err(Box::new(NsifError::ImageSegmentSubHeaderMalformed))
    }

    fn handle_nc(&self) -> Result<Vec<u8>, NsifError> {
        if let Value::SingleAlphanumeric(imode) = &self.sub_header.imode.value {
            if imode.value.as_str() == "P" {
                return Ok(self.data.clone());
            }
        }
        Err(NsifError::ImodeNotSupported)
    }

    fn handle_c3(&self) -> Result<Vec<u8>, DecodeErrors> {
        JpegDecoder::new(&self.data).decode()
    }

    fn handle_c8(&self) -> Result<Vec<u8>, jpeg2k::error::Error> {
        jpeg2k::Image::from_bytes(self.data.as_slice())
            .and_then(|image| image.get_pixels(None))
            .and_then(|image_data| {
                return match image_data.data {
                    ImagePixelData::L8(data)
                    | ImagePixelData::La8(data)
                    | ImagePixelData::Rgb8(data)
                    | ImagePixelData::Rgba8(data) => Ok(data),
                    _ => Err(jpeg2k::error::Error::UnknownFormatError(String::from(
                        "unsupported pixel format encountered",
                    ))),
                };
            })
    }
}

#[derive(Debug, Reflect)]
pub struct ImageSubheader {
    pub im: Field,
    pub iid1: Field,
    pub idatim: Field,
    pub tgtid: Field,
    pub iid2: Field,
    pub isclas: Field,
    pub isclsy: Field,
    pub iscode: Field,
    pub isctlh: Field,
    pub isrel: Field,
    pub isdctp: Field,
    pub isdcdt: Field,
    pub isdcxm: Field,
    pub isdg: Field,
    pub isdgdt: Field,
    pub iscltx: Field,
    pub iscatp: Field,
    pub iscaut: Field,
    pub iscrsn: Field,
    pub issrdt: Field,
    pub isctln: Field,
    pub encryp: Field,
    pub isorce: Field,
    pub nrows: Field,
    pub ncols: Field,
    pub pvtype: Field,
    pub irep: Field,
    pub icat: Field,
    pub abpp: Field,
    pub pjust: Field,
    pub icords: Field,
    pub igeolo: Field,
    pub nicom: Field,
    pub icoms: Field,
    pub ic: Field,
    pub comrat: Field,
    pub nbands: Field,
    pub xbands: Field,
    pub irepbands: Field,
    pub isubcats: Field,
    pub ifcs: Field,
    pub imflts: Field,
    pub nlutss: Field,
    pub neluts: Field,
    pub lutdss: Field,
    pub isync: Field,
    pub imode: Field,
    pub nbpr: Field,
    pub nbpc: Field,
    pub nppbh: Field,
    pub nppbv: Field,
    pub nbpp: Field,
    pub idlvl: Field,
    pub ialvl: Field,
    pub iloc: Field,
    pub imag: Field,
    pub udidl: Field,
    pub udofl: Field,
    pub udid: Field,
    pub ixshdl: Field,
    pub ixsofl: Field,
    pub ixshd: Field,
}
impl ImageSubheader {
    fn parse(mut file: &File) -> Result<ImageSubheader, Box<dyn std::error::Error>> {
        let mut im = vec![0; 2];
        let mut iid1 = vec![0; 10];
        let mut idatim = vec![0; 14];
        let mut tgtid = vec![0; 17];
        let mut iid2 = vec![0; 80];
        let mut isclas = vec![0; 1];
        let mut isclsy = vec![0; 2];
        let mut iscode = vec![0; 11];
        let mut isctlh = vec![0; 2];
        let mut isrel = vec![0; 20];
        let mut isdctp = vec![0; 2];
        let mut isdcdt = vec![0; 8];
        let mut isdcxm = vec![0; 4];
        let mut isdg = vec![0; 1];
        let mut isdgt = vec![0; 8];
        let mut iscltx = vec![0; 43];
        let mut iscatp = vec![0; 1];
        let mut iscaut = vec![0; 40];
        let mut iscrsn = vec![0; 1];
        let mut issrdt = vec![0; 8];
        let mut isctln = vec![0; 15];
        let mut encryp = vec![0; 1];
        let mut isorce = vec![0; 42];
        let mut nrows = vec![0; 8];
        let mut ncols = vec![0; 8];
        let mut pvtype = vec![0; 3];
        let mut irep = vec![0; 8];
        let mut icat = vec![0; 8];
        let mut abpp = vec![0; 2];
        let mut pjust = vec![0; 1];
        let mut icords = vec![0; 1];
        let mut igeolo = vec![0; 60];
        let mut nicom = vec![0; 1];
        let mut icoms = Vec::new();
        let mut ic = vec![0; 2];
        let mut comrat = vec![0; 4];
        let mut nbands = vec![0; 1];
        let mut xbands = vec![0; 5];
        let mut irepbands = Vec::new();
        let mut isubcats = Vec::new();
        let mut ifcs = Vec::new();
        let mut imflts = Vec::new();
        let mut nlutss = Vec::new();
        let mut neluts = Vec::new();
        let mut lutdss = Vec::new();
        let mut isync = vec![0; 1];
        let mut imode = vec![0; 1];
        let mut nbpr = vec![0; 4];
        let mut nbpc = vec![0; 4];
        let mut nppbh = vec![0; 4];
        let mut nppbv = vec![0; 4];
        let mut nbpp = vec![0; 2];
        let mut idlvl = vec![0; 3];
        let mut ialvl = vec![0; 3];
        let mut iloc = vec![0; 10];
        let mut imag = vec![0; 4];
        let mut udidl = vec![0; 5];
        let mut udofl = vec![0; 3];
        // udid is dynamically sized
        let mut ixshdl = vec![0; 5];
        let mut ixsofl = vec![0; 3];
        // ixshd is dynamically sized

        file.read_exact(&mut im)?;
        file.read_exact(&mut iid1)?;
        file.read_exact(&mut idatim)?;
        file.read_exact(&mut tgtid)?;
        file.read_exact(&mut iid2)?;
        file.read_exact(&mut isclas)?;
        file.read_exact(&mut isclsy)?;
        file.read_exact(&mut iscode)?;
        file.read_exact(&mut isctlh)?;
        file.read_exact(&mut isrel)?;
        file.read_exact(&mut isdctp)?;
        file.read_exact(&mut isdcdt)?;
        file.read_exact(&mut isdcxm)?;
        file.read_exact(&mut isdg)?;
        file.read_exact(&mut isdgt)?;
        file.read_exact(&mut iscltx)?;
        file.read_exact(&mut iscatp)?;
        file.read_exact(&mut iscaut)?;
        file.read_exact(&mut iscrsn)?;
        file.read_exact(&mut issrdt)?;
        file.read_exact(&mut isctln)?;
        file.read_exact(&mut encryp)?;
        file.read_exact(&mut isorce)?;
        file.read_exact(&mut nrows)?;
        file.read_exact(&mut ncols)?;
        file.read_exact(&mut pvtype)?;
        file.read_exact(&mut irep)?;
        file.read_exact(&mut icat)?;
        file.read_exact(&mut abpp)?;
        file.read_exact(&mut pjust)?;
        file.read_exact(&mut icords)?;
        file.read_exact(&mut igeolo)?;

        file.read_exact(&mut nicom)?;
        let number_of_image_comments = parse_number_from_bytes(&nicom).unwrap_or(0);
        for _ in 0..number_of_image_comments {
            let mut icom = vec![0; 80];
            file.read_exact(&mut icom)?;
            icoms.push(icom);
        }

        file.read_exact(&mut ic)?;
        let ic_value = String::from_utf8(ic.clone())?;
        if ic_value != "NC" && ic_value != "NM" {
            file.read_exact(&mut comrat)?;
        }

        file.read_exact(&mut nbands)?;
        let nbands_value = parse_number_from_bytes(&nbands).unwrap_or(0);
        if nbands_value == 0 {
            file.read_exact(&mut xbands)?;
        }
        let number_of_bands = if nbands_value > 0 {
            nbands_value
        } else {
            parse_number_from_bytes(&xbands).unwrap_or(0)
        };

        for _ in 0..number_of_bands {
            let mut irepband = vec![0; 2];
            let mut isubcat = vec![0; 6];
            let mut ifc = vec![0; 1];
            let mut imflt = vec![0; 3];
            let mut nluts = vec![0; 1];
            let mut nelut = vec![0; 5];
            let mut lutds = Vec::new();

            file.read_exact(&mut irepband)?;
            file.read_exact(&mut isubcat)?;
            file.read_exact(&mut ifc)?;
            file.read_exact(&mut imflt)?;
            file.read_exact(&mut nluts)?;
            let number_of_lut_entries = parse_number_from_bytes(&nluts).unwrap_or(0);
            if number_of_lut_entries != 0 {
                file.read_exact(&mut nelut)?;
            }
            let lut_entry_size = parse_number_from_bytes(&nelut).unwrap_or(0);
            for _ in 0..number_of_lut_entries {
                let mut lutd = vec![0; lut_entry_size as usize];
                file.read_exact(&mut lutd)?;
                lutds.push(lutd);
            }

            irepbands.push(irepband);
            isubcats.push(isubcat);
            ifcs.push(ifc);
            imflts.push(imflt);
            nlutss.push(nluts);
            neluts.push(nelut);
            lutdss.push(lutds);
        }

        file.read_exact(&mut isync)?;
        file.read_exact(&mut imode)?;
        file.read_exact(&mut nbpr)?;
        file.read_exact(&mut nbpc)?;
        file.read_exact(&mut nppbh)?;
        file.read_exact(&mut nppbv)?;
        file.read_exact(&mut nbpp)?;
        file.read_exact(&mut idlvl)?;
        file.read_exact(&mut ialvl)?;
        file.read_exact(&mut iloc)?;
        file.read_exact(&mut imag)?;
        file.read_exact(&mut udidl)?;
        let udid_length = max(parse_number_from_bytes(&udidl).unwrap_or(3) - 3, 0);
        let mut udid = vec![0; udid_length as usize];
        if udid_length != 0 {
            file.read_exact(&mut udofl)?;
            file.read_exact(&mut udid)?;
        }
        file.read_exact(&mut ixshdl)?;
        let ixshdl_length = max(parse_number_from_bytes(&ixshdl).unwrap_or(3) - 3, 0);
        if ixshdl_length != 0 {
            file.read_exact(&mut ixsofl)?;
        }
        let mut ixshd = vec![0; ixshdl_length as usize];
        if ixshdl_length != 0 {
            file.read_exact(&mut ixshd)?;
        }

        Ok(ImageSubheader {
            im: Field::from_alphanumeric("File Part Type", parse_string_from_bytes(&im)?),
            iid1: Field::from_alphanumeric("Image Identifier 1", parse_string_from_bytes(&iid1)?),
            idatim: Field::from_numeric("Image Date and Time", parse_string_from_bytes(&idatim)?),
            tgtid: Field::from_alphanumeric("Target Identifier", parse_string_from_bytes(&tgtid)?),
            iid2: Field::from_alphanumeric("Image Identifier 2", parse_string_from_bytes(&iid2)?),
            isclas: Field::from_alphanumeric(
                "Image Security Classification",
                parse_string_from_bytes(&isclas)?,
            ),
            isclsy: Field::from_alphanumeric(
                "Image Security Classification System",
                parse_string_from_bytes(&isclsy)?,
            ),
            iscode: Field::from_alphanumeric("Image Codewords", parse_string_from_bytes(&iscode)?),
            isctlh: Field::from_alphanumeric(
                "Image Control and Handling",
                parse_string_from_bytes(&isctlh)?,
            ),
            isrel: Field::from_alphanumeric(
                "Image Releasing Instructions",
                parse_string_from_bytes(&isrel)?,
            ),
            isdctp: Field::from_alphanumeric(
                "Image Declassification Type",
                parse_string_from_bytes(&isdctp)?,
            ),
            isdcdt: Field::from_alphanumeric(
                "Image Declassification Date",
                parse_string_from_bytes(&isdcdt)?,
            ),
            isdcxm: Field::from_alphanumeric(
                "Image Declassification Exemption",
                parse_string_from_bytes(&isdcxm)?,
            ),
            isdg: Field::from_alphanumeric("Image Downgrade", parse_string_from_bytes(&isdg)?),
            isdgdt: Field::from_alphanumeric(
                "Image Downgrade Date",
                parse_string_from_bytes(&isdgt)?,
            ),
            iscltx: Field::from_alphanumeric(
                "Image Classification Text",
                parse_string_from_bytes(&iscltx)?,
            ),
            iscatp: Field::from_alphanumeric(
                "Image Classification Authority Type",
                parse_string_from_bytes(&iscatp)?,
            ),
            iscaut: Field::from_alphanumeric(
                "Image Classification Authority",
                parse_string_from_bytes(&iscaut)?,
            ),
            iscrsn: Field::from_alphanumeric(
                "Image Classification Reason",
                parse_string_from_bytes(&iscrsn)?,
            ),
            issrdt: Field::from_alphanumeric(
                "Image Security Source Date",
                parse_string_from_bytes(&issrdt)?,
            ),
            isctln: Field::from_alphanumeric(
                "Image Security Control Number",
                parse_string_from_bytes(&isctln)?,
            ),
            encryp: Field::from_numeric("Encryption", parse_string_from_bytes(&encryp)?),
            isorce: Field::from_alphanumeric("Image Source", parse_string_from_bytes(&isorce)?),
            nrows: Field::from_numeric(
                "Number of Significant Rows in Image",
                parse_string_from_bytes(&nrows)?,
            ),
            ncols: Field::from_numeric(
                "Number of Significant Columns in Image",
                parse_string_from_bytes(&ncols)?,
            ),
            pvtype: Field::from_alphanumeric("Pixel Value Type", parse_string_from_bytes(&pvtype)?),
            irep: Field::from_alphanumeric("Image Representation", parse_string_from_bytes(&irep)?),
            icat: Field::from_alphanumeric("Image Category", parse_string_from_bytes(&icat)?),
            abpp: Field::from_numeric(
                "Actual Bits-per-Pixel per Band",
                parse_string_from_bytes(&abpp)?,
            ),
            pjust: Field::from_alphanumeric(
                "Pixel Justification",
                parse_string_from_bytes(&pjust)?,
            ),
            icords: Field::from_alphanumeric(
                "Image Coordinate Representation",
                parse_string_from_bytes(&icords)?,
            ),
            igeolo: Field::from_alphanumeric(
                "Image Geographic Location",
                parse_string_from_bytes(&igeolo)?,
            ),
            nicom: Field::from_numeric(
                "Number of Image Comments",
                parse_string_from_bytes(&nicom)?,
            ),
            icoms: Field::from_multiple_alphanumeric(
                "Image comments",
                icoms
                    .iter()
                    .map(parse_string_from_bytes)
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            ic: Field::from_alphanumeric("Image compression", parse_string_from_bytes(&ic)?),
            comrat: Field::from_alphanumeric(
                "Compression Rate Code",
                parse_string_from_bytes(&comrat)?,
            ),
            nbands: Field::from_numeric("Number of Bands", parse_string_from_bytes(&nbands)?),
            xbands: Field::from_numeric(
                "Number of Multispectral Bands",
                parse_string_from_bytes(&xbands)?,
            ),
            irepbands: Field::from_multiple_alphanumeric(
                "Band Representations",
                irepbands
                    .iter()
                    .map(parse_string_from_bytes)
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            isubcats: Field::from_multiple_alphanumeric(
                "Band Subcategories",
                isubcats
                    .iter()
                    .map(parse_string_from_bytes)
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            ifcs: Field::from_multiple_alphanumeric(
                "Band Image Filter Condition",
                ifcs.iter()
                    .map(parse_string_from_bytes)
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            imflts: Field::from_multiple_alphanumeric(
                "Band Standard Image Code",
                imflts
                    .iter()
                    .map(parse_string_from_bytes)
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            nlutss: Field::from_multiple_numeric(
                "Number of LUTs",
                nlutss
                    .iter()
                    .map(parse_string_from_bytes)
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            neluts: Field::from_multiple_numeric(
                "Number of LUT entries",
                neluts
                    .iter()
                    .map(parse_string_from_bytes)
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            lutdss: Field::from_nested_numeric(
                "LUTs",
                lutdss
                    .iter()
                    .filter_map(|l| l.iter().map(|l| parse_string_from_bytes(l).ok()).collect())
                    .collect(),
            ),

            isync: Field::from_numeric("Image Sync Code", parse_string_from_bytes(&isync)?),
            imode: Field::from_alphanumeric("Image Mode", parse_string_from_bytes(&imode)?),
            nbpr: Field::from_numeric("Number of Blocks per Row", parse_string_from_bytes(&nbpr)?),
            nbpc: Field::from_numeric(
                "Number of Blocks per Columns",
                parse_string_from_bytes(&nbpc)?,
            ),
            nppbh: Field::from_numeric(
                "Number of Pixels per Block Horizontal",
                parse_string_from_bytes(&nppbh)?,
            ),
            nppbv: Field::from_numeric(
                "Number of Pixels per Block Vertical",
                parse_string_from_bytes(&nppbv)?,
            ),
            nbpp: Field::from_numeric(
                "Number of Bits per Pixel per Band",
                parse_string_from_bytes(&nbpp)?,
            ),
            idlvl: Field::from_numeric("Image Display Level", parse_string_from_bytes(&idlvl)?),
            ialvl: Field::from_numeric("Image Attachment Level", parse_string_from_bytes(&ialvl)?),
            iloc: Field::from_numeric("Image Location", parse_string_from_bytes(&iloc)?),
            imag: Field::from_alphanumeric("Image Magnification", parse_string_from_bytes(&imag)?),
            udidl: Field::from_numeric(
                "User-Defined Image Data Length",
                parse_string_from_bytes(&udidl)?,
            ),
            udofl: Field::from_numeric("User-Defined Overflow", parse_string_from_bytes(&udofl)?),
            udid: Field::from_alphanumeric(
                "User-Defined Image Data",
                parse_string_from_bytes(&udid)?,
            ),
            ixshdl: Field::from_numeric(
                "Image Extended Subheader Length",
                parse_string_from_bytes(&ixshdl)?,
            ),
            ixsofl: Field::from_numeric(
                "Image Extended Subheader Overflow",
                parse_string_from_bytes(&ixsofl)
                    .unwrap_or("Failed to parse Extended Subheader Overflow".to_string()),
            ),
            ixshd: Field::from_alphanumeric(
                "Image Extended Subheader Data",
                parse_string_from_bytes(&ixshd)
                    .unwrap_or("Failed to parse Extended Subheader Data".to_string()),
            ),
        })
    }
}

impl PrettyPrint for ImageSubheader {}

impl PrettyPrint for ImageSegment {
    fn pretty_print(&self, include_empty_fields: bool) -> String {
        self.sub_header.pretty_print(include_empty_fields)
    }
}
