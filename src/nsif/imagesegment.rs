use super::field::FieldValue;
use super::{parse_number, PrettyPrint};
use crate::nsif::field::Field;
use bevy_reflect::Reflect;
use core::panic;
use std::cmp::max;
use std::fmt::Display;
use std::vec;
use std::{fs::File, io::Read};

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
    ) -> Result<ImageSegment, Box<dyn std::error::Error>> {
        let image_subheader = ImageSubheader::parse(file)?;
        let mut data = vec![0; segment_length as usize];
        file.read(&mut data)?;
        Ok(ImageSegment {
            sub_header: image_subheader,
            data,
        })
    }

    pub fn dimensions(&self) -> (i32, i32) {
        if let (FieldValue::Single(height), FieldValue::Single(width)) =
            (&self.sub_header.nrows.value, &self.sub_header.ncols.value)
        {
            return (
                parse_number(&height).unwrap(),
                parse_number(&width).unwrap(),
            );
        }
        panic!()
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

        file.read(&mut im)?;
        file.read(&mut iid1)?;
        file.read(&mut idatim)?;
        file.read(&mut tgtid)?;
        file.read(&mut iid2)?;
        file.read(&mut isclas)?;
        file.read(&mut isclsy)?;
        file.read(&mut iscode)?;
        file.read(&mut isctlh)?;
        file.read(&mut isrel)?;
        file.read(&mut isdctp)?;
        file.read(&mut isdcdt)?;
        file.read(&mut isdcxm)?;
        file.read(&mut isdg)?;
        file.read(&mut isdgt)?;
        file.read(&mut iscltx)?;
        file.read(&mut iscatp)?;
        file.read(&mut iscaut)?;
        file.read(&mut iscrsn)?;
        file.read(&mut issrdt)?;
        file.read(&mut isctln)?;
        file.read(&mut encryp)?;
        file.read(&mut isorce)?;
        file.read(&mut nrows)?;
        file.read(&mut ncols)?;
        file.read(&mut pvtype)?;
        file.read(&mut irep)?;
        file.read(&mut icat)?;
        file.read(&mut abpp)?;
        file.read(&mut pjust)?;
        file.read(&mut icords)?;
        file.read(&mut igeolo)?;

        file.read(&mut nicom)?;
        let number_of_image_comments = parse_number(&nicom).unwrap_or(0);
        for _ in 0..number_of_image_comments {
            let mut icom = vec![0; 80];
            file.read(&mut icom)?;
            icoms.push(icom);
        }

        file.read(&mut ic)?;
        let ic_value = String::from_utf8(ic.clone())?;
        if ic_value != "NC" && ic_value != "NM" {
            file.read(&mut comrat)?;
        }

        file.read(&mut nbands)?;
        let nbands_value = parse_number(&nbands).unwrap_or(0);
        if nbands_value == 0 {
            file.read(&mut xbands)?;
        }
        let number_of_bands = if nbands_value > 0 {
            nbands_value
        } else {
            parse_number(&xbands).unwrap_or(0)
        };

        for _ in 0..number_of_bands {
            let mut irepband = vec![0; 2];
            let mut isubcat = vec![0; 6];
            let mut ifc = vec![0; 1];
            let mut imflt = vec![0; 3];
            let mut nluts = vec![0; 1];
            let mut nelut = vec![0; 5];
            let mut lutds = Vec::new();

            file.read(&mut irepband)?;
            file.read(&mut isubcat)?;
            file.read(&mut ifc)?;
            file.read(&mut imflt)?;
            file.read(&mut nluts)?;
            let number_of_lut_entries = parse_number(&nluts).unwrap_or(0);
            if number_of_lut_entries != 0 {
                file.read(&mut nelut)?;
            }
            let lut_entry_size = parse_number(&nelut).unwrap_or(0);
            for _ in 0..number_of_lut_entries {
                let mut lutd = vec![0; lut_entry_size as usize];
                file.read(&mut lutd)?;
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

        file.read(&mut isync)?;
        file.read(&mut imode)?;
        file.read(&mut nbpr)?;
        file.read(&mut nbpc)?;
        file.read(&mut nppbh)?;
        file.read(&mut nppbv)?;
        file.read(&mut nbpp)?;
        file.read(&mut idlvl)?;
        file.read(&mut ialvl)?;
        file.read(&mut iloc)?;
        file.read(&mut imag)?;
        file.read(&mut udidl)?;
        let udid_length = max(parse_number(&udidl).unwrap_or(3) - 3, 0);
        let mut udid = vec![0; udid_length as usize];
        if udid_length != 0 {
            file.read(&mut udofl)?;
            file.read(&mut udid)?;
        }
        file.read(&mut ixshdl)?;
        let ixshdl_length = parse_number(&ixshdl).unwrap_or(0);
        if ixshdl_length != 0 {
            file.read(&mut ixsofl)?;
        }
        let ixsofl_length = max(parse_number(&ixsofl).unwrap_or(3) - 3, 0);
        let mut ixshd = vec![0; ixsofl_length as usize];
        if ixsofl_length != 0 {
            file.read(&mut ixshd)?;
        }

        Ok(ImageSubheader {
            im: Field::from_single("File Part Type", im),
            iid1: Field::from_single("Image Identifier 1", iid1),
            idatim: Field::from_single("Image Date and Time", idatim),
            tgtid: Field::from_single("Target Identifier", tgtid),
            iid2: Field::from_single("Image Identifier 2", iid2),
            isclas: Field::from_single("Image Security Classification", isclas),
            isclsy: Field::from_single("Image Security Classification System", isclsy),
            iscode: Field::from_single("Image Codewords", iscode),
            isctlh: Field::from_single("Image Control and Handling", isctlh),
            isrel: Field::from_single("Image Releasing Instructions", isrel),
            isdctp: Field::from_single("Image Declassification Type", isdctp),
            isdcdt: Field::from_single("Image Declassification Date", isdcdt),
            isdcxm: Field::from_single("Image Declassification Exemption", isdcxm),
            isdg: Field::from_single("Image Downgrade", isdg),
            isdgdt: Field::from_single("Image Downgrade Date", isdgt),
            iscltx: Field::from_single("Image Classification Text", iscltx),
            iscatp: Field::from_single("Image Classification Authority Type", iscatp),
            iscaut: Field::from_single("Image Classification Authority", iscaut),
            iscrsn: Field::from_single("Image Classification Reason", iscrsn),
            issrdt: Field::from_single("Image Security Source Date", issrdt),
            isctln: Field::from_single("Image Security Control Number", isctln),
            encryp: Field::from_single("Encryption", encryp),
            isorce: Field::from_single("Image Source", isorce),
            nrows: Field::from_single("Number of Significant Rows in Image", nrows),
            ncols: Field::from_single("Number of Significant Columns in Image", ncols),
            pvtype: Field::from_single("Pixel Value Type", pvtype),
            irep: Field::from_single("Image Representation", irep),
            icat: Field::from_single("Image Category", icat),
            abpp: Field::from_single("Actual Bits-per-Pixel per Band", abpp),
            pjust: Field::from_single("Pixel Justification", pjust),
            icords: Field::from_single("Image Coordinate Representation", icords),
            igeolo: Field::from_single("Image Geographic Location", igeolo),
            nicom: Field::from_single("Number of Image Comments", nicom),
            icoms: Field::from_multiple("Image comments", icoms),
            ic: Field::from_single("Image compression", ic),
            comrat: Field::from_single("Compression Rate Code", comrat),
            nbands: Field::from_single("Number of Bands", nbands),
            xbands: Field::from_single("Number of Multispectral Bands", xbands),
            irepbands: Field::from_multiple("Band Representations", irepbands),
            isubcats: Field::from_multiple("Band Subcategories", isubcats),
            ifcs: Field::from_multiple("Band Image Filter Condition", ifcs),
            imflts: Field::from_multiple("Band Standard Image Code", imflts),
            nlutss: Field::from_multiple("Number of LUTs", nlutss),
            neluts: Field::from_multiple("Number of LUT entries", neluts),
            lutdss: Field::from_nested("LUTs", lutdss),
            isync: Field::from_single("Image Sync Code", isync),
            imode: Field::from_single("Image Mode", imode),
            nbpr: Field::from_single("Number of Blocks per Row", nbpr),
            nbpc: Field::from_single("Number of Blocks per Columns", nbpc),
            nppbh: Field::from_single("Number of Pixels per Block Horizontal", nppbh),
            nppbv: Field::from_single("Number of Pixels per Block Vertical", nppbv),
            nbpp: Field::from_single("Number of Bits per Pixel per Band", nbpp),
            idlvl: Field::from_single("Image Display Level", idlvl),
            ialvl: Field::from_single("Image Attachment Level", ialvl),
            iloc: Field::from_single("Image Location", iloc),
            imag: Field::from_single("Image Magnification", imag),
            udidl: Field::from_single("User-Defined Image Data Length", udidl),
            udofl: Field::from_single("User-Defined Overflow", udofl),
            udid: Field::from_single("User-Defined Image Data", udid),
            ixshdl: Field::from_single("Image Extended Subheader Length", ixshdl),
            ixsofl: Field::from_single("Image Extended Subheader Overflow", ixsofl),
            ixshd: Field::from_single("Image Extended Subheader Data", ixshd),
        })
    }
}

impl PrettyPrint for ImageSubheader {}

impl Display for ImageSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sub_header.pretty_print())
    }
}
