use super::{parse_number, PrettyPrint};
use crate::nsif::field::Field;
use bevy_reflect::Reflect;
use std::cmp::max;
use std::fmt::Display;
use std::{fs::File, io::Read};
use std::{usize, vec};

#[derive(Debug, Reflect)]
pub struct FileHeader {
    fhdr: Field,
    fver: Field,
    clevel: Field,
    stype: Field,
    ostaid: Field,
    fdt: Field,
    ftitle: Field,
    fsclas: Field,
    fsclsy: Field,
    fscode: Field,
    fsctlh: Field,
    fsrel: Field,
    fsdctp: Field,
    fsdcdt: Field,
    fsdcxm: Field,
    fsdg: Field,
    fsdgdt: Field,
    fscltx: Field,
    fscatp: Field,
    fscaut: Field,
    fscrsn: Field,
    fssrdt: Field,
    fsctln: Field,
    fscop: Field,
    fscpys: Field,
    encryp: Field,
    fbkgc: Field,
    oname: Field,
    ophone: Field,
    fl: Field,
    hl: Field,
    numi: Field,
    pub lishs: Field,
    pub lis: Field,
    nums: Field,
    lsshs: Field,
    lss: Field,
    numx: Field,
    numt: Field,
    ltshs: Field,
    lts: Field,
    numdes: Field,
    ldshs: Field,
    lds: Field,
    numres: Field,
    lreshs: Field,
    lres: Field,
    udhdl: Field,
    udhofl: Field,
    udhd: Field,
    xhdl: Field,
    xhdlofl: Field,
    xhd: Field,
}

impl FileHeader {
    pub fn parse(mut file: &File) -> Result<FileHeader, Box<dyn std::error::Error>> {
        let mut fhdr = vec![0; 4];
        let mut fver = vec![0; 5];
        let mut clevel = vec![0; 2];
        let mut stype = vec![0; 4];
        let mut ostaid = vec![0; 10];
        let mut fdt = vec![0; 14];
        let mut ftitle = vec![0; 80];
        let mut fsclas = vec![0; 1];
        let mut fsclsy = vec![0; 2];
        let mut fscode = vec![0; 11];
        let mut fsctlh = vec![0; 2];
        let mut fsrel = vec![0; 20];
        let mut fsdctp = vec![0; 2];
        let mut fsdcdt = vec![0; 8];
        let mut fsdcxm = vec![0; 4];
        let mut fsdg = vec![0; 1];
        let mut fsdgdt = vec![0; 8];
        let mut fscltx = vec![0; 43];
        let mut fscatp = vec![0; 1];
        let mut fscaut = vec![0; 40];
        let mut fscrsn = vec![0; 1];
        let mut fssrdt = vec![0; 8];
        let mut fsctln = vec![0; 15];
        let mut fscop = vec![0; 5];
        let mut fscpys = vec![0; 5];
        let mut encryp = vec![0; 1];
        let mut fbkgc = vec![0; 3];
        let mut oname = vec![0; 24];
        let mut ophone = vec![0; 18];
        let mut fl = vec![0; 12];
        let mut hl = vec![0; 6];
        let mut numi = vec![0; 3];
        let mut lishs = Vec::new();
        let mut lis = Vec::new();
        let mut nums = vec![0; 3];
        let mut lsshs = Vec::new();
        let mut lss = Vec::new();
        let mut numx = vec![0; 3];
        let mut numt = vec![0; 3];
        let mut ltshs = Vec::new();
        let mut lts = Vec::new();
        let mut numdes = vec![0; 3];
        let mut ldshs = Vec::new();
        let mut lds = Vec::new();
        let mut numres = vec![0; 3];
        let mut lreshs = Vec::new();
        let mut lres = Vec::new();
        let mut udhdl = vec![0; 5];
        let mut udhofl = vec![0; 3];
        // udhd is dynamically sized
        let mut xhdl = vec![0; 5];
        let mut xhdlofl = vec![0; 3];
        // xhd is dynamically sized

        file.read(&mut fhdr)?;
        file.read(&mut fver)?;
        file.read(&mut clevel)?;
        file.read(&mut stype)?;
        file.read(&mut ostaid)?;
        file.read(&mut fdt)?;
        file.read(&mut ftitle)?;
        file.read(&mut fsclas)?;
        file.read(&mut fsclsy)?;
        file.read(&mut fscode)?;
        file.read(&mut fsctlh)?;
        file.read(&mut fsrel)?;
        file.read(&mut fsdctp)?;
        file.read(&mut fsdcdt)?;
        file.read(&mut fsdcxm)?;
        file.read(&mut fsdg)?;
        file.read(&mut fsdgdt)?;
        file.read(&mut fscltx)?;
        file.read(&mut fscatp)?;
        file.read(&mut fscaut)?;
        file.read(&mut fscrsn)?;
        file.read(&mut fssrdt)?;
        file.read(&mut fsctln)?;
        file.read(&mut fscop)?;
        file.read(&mut fscpys)?;
        file.read(&mut encryp)?;
        file.read(&mut fbkgc)?;
        file.read(&mut oname)?;
        file.read(&mut ophone)?;
        file.read(&mut fl)?;
        file.read(&mut hl)?;

        file.read(&mut numi)?;
        let number_of_image_segments = parse_number(&numi).unwrap_or(0);

        for _ in 0..number_of_image_segments {
            let mut lish = vec![0; 6];
            let mut li = vec![0; 10];
            file.read(&mut lish)?;
            file.read(&mut li)?;
            lishs.push(lish);
            lis.push(li);
        }

        file.read(&mut nums)?;
        let number_of_graphic_segments = parse_number(&nums).unwrap_or(0);

        for _ in 0..number_of_graphic_segments {
            let mut lssh = vec![0; 4];
            let mut ls = vec![0; 6];
            file.read(&mut lssh)?;
            file.read(&mut ls)?;
            lsshs.push(lssh);
            lss.push(ls);
        }

        file.read(&mut numx)?;

        file.read(&mut numt)?;
        let number_of_text_segments = parse_number(&numt).unwrap_or(0);

        for _ in 0..number_of_text_segments {
            let mut ltsh = vec![0; 4];
            let mut lt = vec![0; 5];
            file.read(&mut ltsh)?;
            file.read(&mut lt)?;
            ltshs.push(ltsh);
            lts.push(lt);
        }

        file.read(&mut numdes)?;
        let number_of_data_extension_segments = parse_number(&numdes).unwrap_or(0);

        for _ in 0..number_of_data_extension_segments {
            let mut ldsh = vec![0; 4];
            let mut ld = vec![0; 9];
            file.read(&mut ldsh)?;
            file.read(&mut ld)?;
            ldshs.push(ldsh);
            lds.push(ld);
        }

        file.read(&mut numres)?;
        let number_of_reserved_extension_segments = parse_number(&numres).unwrap_or(0);

        for _ in 0..number_of_reserved_extension_segments {
            let mut lresh = vec![0; 4];
            let mut lre = vec![0; 7];
            file.read(&mut lresh)?;
            file.read(&mut lre)?;
            lreshs.push(lresh);
            lres.push(lre);
        }

        file.read(&mut udhdl)?;
        let udhd_length = max(parse_number(&udhdl).unwrap_or(3) - 3, 0);

        if udhd_length != 0 {
            file.read(&mut udhofl)?;
        }

        let mut udhd = vec![0; udhd_length as usize];
        file.read(&mut udhd)?;

        file.read(&mut xhdl)?;
        let xhd_length = max(parse_number(&xhdl).unwrap_or(3) - 3, 0);

        if xhd_length != 0 {
            file.read(&mut xhdlofl)?;
        }

        let mut xhd = vec![0; xhd_length as usize];
        file.read(&mut xhd)?;

        Ok(FileHeader {
            fhdr: Field::from_single("File Profile Name", fhdr),
            fver: Field::from_single("File Version", fver),
            clevel: Field::from_single("Complexity level", clevel),
            stype: Field::from_single("Standard Type", stype),
            ostaid: Field::from_single("Originating Station Identifier", ostaid),
            fdt: Field::from_single("File Date and Time", fdt),
            ftitle: Field::from_single("File Title", ftitle),
            fsclas: Field::from_single("File Security Classification", fsclas),
            fsclsy: Field::from_single("File Security Classification System", fsclsy),
            fscode: Field::from_single("File Codewords", fscode),
            fsctlh: Field::from_single("File Control and Handling", fsctlh),
            fsrel: Field::from_single("File Releasing Instructions", fsrel),
            fsdctp: Field::from_single("File Declassification Type", fsdctp),
            fsdcdt: Field::from_single("File Declassification Date", fsdcdt),
            fsdcxm: Field::from_single("File Declassifcation Exemption", fsdcxm),
            fsdg: Field::from_single("File Downgrade", fsdg),
            fsdgdt: Field::from_single("File Downgrade Date", fsdgdt),
            fscltx: Field::from_single("File Classifcation Text", fscltx),
            fscatp: Field::from_single("File Classification Authority Type", fscatp),
            fscaut: Field::from_single("File Classification Authority", fscaut),
            fscrsn: Field::from_single("File Classification Reason", fscrsn),
            fssrdt: Field::from_single("File Security Source Date", fssrdt),
            fsctln: Field::from_single("File Security Control Number", fsctln),
            fscop: Field::from_single("File Copy Number", fscop),
            fscpys: Field::from_single("File Number of Copies", fscpys),
            encryp: Field::from_single("Encryption", encryp),
            fbkgc: Field::from_single("File Background Color", fbkgc),
            oname: Field::from_single("Originator's Name", oname),
            ophone: Field::from_single("Originator's Phone Number", ophone),
            fl: Field::from_single("File Length", fl),
            hl: Field::from_single("NSIF File Header Length", hl),
            numi: Field::from_single("Number of Image Segments", numi),
            lishs: Field::from_multiple("Length of Image Subheader", lishs),
            lis: Field::from_multiple("Length of Image Segment", lis),
            nums: Field::from_single("Number of Graphic Segments", nums),
            lsshs: Field::from_multiple("Length of Graphic Subheader", lsshs),
            lss: Field::from_multiple("Length of Graphic Segment", lss),
            numx: Field::from_single("Reserved for Future Use", numx),
            numt: Field::from_single("Number of Text Segments", numt),
            ltshs: Field::from_multiple("Length of Text Subheader", ltshs),
            lts: Field::from_multiple("Length of Text Segment", lts),
            numdes: Field::from_single("Number of Data Extension Segments", numdes),
            ldshs: Field::from_multiple("Length of Data Extension Segment Subheader", ldshs),
            lds: Field::from_multiple("Length of Data Extension Segment", lds),
            numres: Field::from_single("Number of Reserved Extension Segments", numres),
            lreshs: Field::from_multiple("Length of Reserved Extension Segment Subheader", lreshs),
            lres: Field::from_multiple("Length of Reserved Extension Segment", lres),
            udhdl: Field::from_single("User-Defined Header Data Length", udhdl),
            udhofl: Field::from_single("User-Defined Header Overflow", udhofl),
            udhd: Field::from_single("User-Defined Header Data", udhd),
            xhdl: Field::from_single("Extended Header Data Length", xhdl),
            xhdlofl: Field::from_single("Extended Header Data Overflow", xhdlofl),
            xhd: Field::from_single("Extended Header Data", xhd),
        })
    }
}
impl PrettyPrint for FileHeader {}

impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}
