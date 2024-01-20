use crate::nsif::field::Field;
use bevy_reflect::{Reflect, Struct};
use std::fmt::{write, Display};
use std::vec;
use std::{fs::File, io::Read};

use super::PrettyPrint;

#[derive(Debug, Reflect)]
pub struct ImageSegment {
    sub_header: ImageSubheader,
}
impl ImageSegment {
    pub fn parse(mut file: &File) -> Result<ImageSegment, Box<dyn std::error::Error>> {
        let image_subheader = ImageSubheader::parse(file)?;
        Ok(ImageSegment {
            sub_header: image_subheader,
        })
    }
}

#[derive(Debug, Reflect)]
struct ImageSubheader {
    im: Field,
    iid1: Field,
    idatim: Field,
    tgtid: Field,
    iid2: Field,
    isclas: Field,
    isclsy: Field,
    iscode: Field,
    isctlh: Field,
    isrel: Field,
    isdctp: Field,
    isdcdt: Field,
    isdcxm: Field,
    isdg: Field,
    isdgdt: Field,
    iscltx: Field,
    iscatp: Field,
    iscaut: Field,
    iscrsn: Field,
    issrdt: Field,
    isctln: Field,
    encryp: Field,
    isorce: Field,
    nrows: Field,
    ncols: Field,
    pvtype: Field,
    irep: Field,
    icat: Field,
    abpp: Field,
    pjust: Field,
    icords: Field,
    igeolo: Field,
    nicom: Field,
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
        })
    }
}

impl PrettyPrint for ImageSubheader {}

impl Display for ImageSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sub_header.pretty_print())
    }
}
