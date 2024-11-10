use super::error::NsifError;
use super::{
    parse_number_from_bytes, parse_string_from_bytes, parse_unsigned_integers_from_byte,
    PrettyPrint,
};
use crate::nsif::field::Field;
use bevy_reflect::Reflect;
use std::cmp::max;
use std::fmt::Display;
use std::{fs::File, io::Read};
use std::{usize, vec};

#[derive(Debug, Reflect)]
pub struct FileHeader {
    pub fhdr: Field,
    pub fver: Field,
    pub clevel: Field,
    pub stype: Field,
    pub ostaid: Field,
    pub fdt: Field,
    pub ftitle: Field,
    pub fsclas: Field,
    pub fsclsy: Field,
    pub fscode: Field,
    pub fsctlh: Field,
    pub fsrel: Field,
    pub fsdctp: Field,
    pub fsdcdt: Field,
    pub fsdcxm: Field,
    pub fsdg: Field,
    pub fsdgdt: Field,
    pub fscltx: Field,
    pub fscatp: Field,
    pub fscaut: Field,
    pub fscrsn: Field,
    pub fssrdt: Field,
    pub fsctln: Field,
    pub fscop: Field,
    pub fscpys: Field,
    pub encryp: Field,
    pub fbkgc: Field,
    pub oname: Field,
    pub ophone: Field,
    pub fl: Field,
    pub hl: Field,
    pub numi: Field,
    pub lishs: Field,
    pub lis: Field,
    pub nums: Field,
    pub lsshs: Field,
    pub lss: Field,
    pub numx: Field,
    pub numt: Field,
    pub ltshs: Field,
    pub lts: Field,
    pub numdes: Field,
    pub ldshs: Field,
    pub lds: Field,
    pub numres: Field,
    pub lreshs: Field,
    pub lres: Field,
    pub udhdl: Field,
    pub udhofl: Field,
    pub udhd: Field,
    pub xhdl: Field,
    pub xhdlofl: Field,
    pub xhd: Field,
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
        let fhdr_value = parse_string_from_bytes(&fhdr)?;
        if !matches!(fhdr_value.as_str(), "NITF" | "NSIF") {
            return Err(Box::new(NsifError::FileMismatch));
        }

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
        let number_of_image_segments = parse_number_from_bytes(&numi).unwrap_or(0);

        for _ in 0..number_of_image_segments {
            let mut lish = vec![0; 6];
            let mut li = vec![0; 10];
            file.read(&mut lish)?;
            file.read(&mut li)?;
            lishs.push(lish);
            lis.push(li);
        }

        file.read(&mut nums)?;
        let number_of_graphic_segments = parse_number_from_bytes(&nums).unwrap_or(0);

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
        let number_of_text_segments = parse_number_from_bytes(&numt).unwrap_or(0);

        for _ in 0..number_of_text_segments {
            let mut ltsh = vec![0; 4];
            let mut lt = vec![0; 5];
            file.read(&mut ltsh)?;
            file.read(&mut lt)?;
            ltshs.push(ltsh);
            lts.push(lt);
        }

        file.read(&mut numdes)?;
        let number_of_data_extension_segments = parse_number_from_bytes(&numdes).unwrap_or(0);

        for _ in 0..number_of_data_extension_segments {
            let mut ldsh = vec![0; 4];
            let mut ld = vec![0; 9];
            file.read(&mut ldsh)?;
            file.read(&mut ld)?;
            ldshs.push(ldsh);
            lds.push(ld);
        }

        file.read(&mut numres)?;
        let number_of_reserved_extension_segments = parse_number_from_bytes(&numres).unwrap_or(0);

        for _ in 0..number_of_reserved_extension_segments {
            let mut lresh = vec![0; 4];
            let mut lre = vec![0; 7];
            file.read(&mut lresh)?;
            file.read(&mut lre)?;
            lreshs.push(lresh);
            lres.push(lre);
        }

        file.read(&mut udhdl)?;
        let udhd_length = max(parse_number_from_bytes(&udhdl).unwrap_or(3) - 3, 0);

        if udhd_length != 0 {
            file.read(&mut udhofl)?;
        }

        let mut udhd = vec![0; udhd_length as usize];
        file.read(&mut udhd)?;

        file.read(&mut xhdl)?;
        let xhd_length = max(parse_number_from_bytes(&xhdl).unwrap_or(3) - 3, 0);

        if xhd_length != 0 {
            file.read(&mut xhdlofl)?;
        }

        let mut xhd = vec![0; xhd_length as usize];
        file.read(&mut xhd)?;

        Ok(FileHeader {
            fhdr: Field::from_alphanumeric("File Profile Name", parse_string_from_bytes(&fhdr)?),
            fver: Field::from_alphanumeric("File Version", parse_string_from_bytes(&fver)?),
            clevel: Field::from_numeric("Complexity level", parse_string_from_bytes(&clevel)?),
            stype: Field::from_alphanumeric("Standard Type", parse_string_from_bytes(&stype)?),
            ostaid: Field::from_alphanumeric(
                "Originating Station Identifier",
                parse_string_from_bytes(&ostaid)?,
            ),
            fdt: Field::from_numeric("File Date and Time", parse_string_from_bytes(&fdt)?),
            ftitle: Field::from_alphanumeric("File Title", parse_string_from_bytes(&ftitle)?),
            fsclas: Field::from_alphanumeric(
                "File Security Classification",
                parse_string_from_bytes(&fsclas)?,
            ),
            fsclsy: Field::from_alphanumeric(
                "File Security Classification System",
                parse_string_from_bytes(&fsclsy)?,
            ),
            fscode: Field::from_alphanumeric("File Codewords", parse_string_from_bytes(&fscode)?),
            fsctlh: Field::from_alphanumeric(
                "File Control and Handling",
                parse_string_from_bytes(&fsctlh)?,
            ),
            fsrel: Field::from_alphanumeric(
                "File Releasing Instructions",
                parse_string_from_bytes(&fsrel)?,
            ),
            fsdctp: Field::from_alphanumeric(
                "File Declassification Type",
                parse_string_from_bytes(&fsdctp)?,
            ),
            fsdcdt: Field::from_alphanumeric(
                "File Declassification Date",
                parse_string_from_bytes(&fsdcdt)?,
            ),
            fsdcxm: Field::from_alphanumeric(
                "File Declassifcation Exemption",
                parse_string_from_bytes(&fsdcxm)?,
            ),
            fsdg: Field::from_alphanumeric("File Downgrade", parse_string_from_bytes(&fsdg)?),
            fsdgdt: Field::from_alphanumeric(
                "File Downgrade Date",
                parse_string_from_bytes(&fsdgdt)?,
            ),
            fscltx: Field::from_alphanumeric(
                "File Classifcation Text",
                parse_string_from_bytes(&fscltx)?,
            ),
            fscatp: Field::from_alphanumeric(
                "File Classification Authority Type",
                parse_string_from_bytes(&fscatp)?,
            ),
            fscaut: Field::from_alphanumeric(
                "File Classification Authority",
                parse_string_from_bytes(&fscaut)?,
            ),
            fscrsn: Field::from_alphanumeric(
                "File Classification Reason",
                parse_string_from_bytes(&fscrsn)?,
            ),
            fssrdt: Field::from_alphanumeric(
                "File Security Source Date",
                parse_string_from_bytes(&fssrdt)?,
            ),
            fsctln: Field::from_alphanumeric(
                "File Security Control Number",
                parse_string_from_bytes(&fsctln)?,
            ),
            fscop: Field::from_numeric("File Copy Number", parse_string_from_bytes(&fscop)?),
            fscpys: Field::from_numeric("File Number of Copies", parse_string_from_bytes(&fscpys)?),
            encryp: Field::from_numeric("Encryption", parse_string_from_bytes(&encryp)?),
            fbkgc: Field::from_alphanumeric(
                "File Background Color",
                parse_unsigned_integers_from_byte(&fbkgc),
            ),
            oname: Field::from_alphanumeric("Originator's Name", parse_string_from_bytes(&oname)?),
            ophone: Field::from_alphanumeric(
                "Originator's Phone Number",
                parse_string_from_bytes(&ophone)?,
            ),
            fl: Field::from_numeric("File Length", parse_string_from_bytes(&fl)?),
            hl: Field::from_numeric("NSIF File Header Length", parse_string_from_bytes(&hl)?),
            numi: Field::from_numeric("Number of Image Segments", parse_string_from_bytes(&numi)?),
            lishs: Field::from_multiple_numeric(
                "Length of Image Subheader",
                lishs
                    .iter()
                    .map(|l| parse_string_from_bytes(l))
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            lis: Field::from_multiple_numeric(
                "Length of Image Segment",
                lis.iter()
                    .map(|l| parse_string_from_bytes(l))
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            nums: Field::from_numeric(
                "Number of Graphic Segments",
                parse_string_from_bytes(&nums)?,
            ),
            lsshs: Field::from_multiple_numeric(
                "Length of Graphic Subheader",
                lsshs
                    .iter()
                    .map(|l| parse_string_from_bytes(l))
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            lss: Field::from_multiple_numeric(
                "Length of Graphic Segment",
                lss.iter()
                    .map(|l| parse_string_from_bytes(l))
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            numx: Field::from_numeric("Reserved for Future Use", parse_string_from_bytes(&numx)?),
            numt: Field::from_numeric("Number of Text Segments", parse_string_from_bytes(&numt)?),
            ltshs: Field::from_multiple_numeric(
                "Length of Text Subheader",
                ltshs
                    .iter()
                    .map(|l| parse_string_from_bytes(l))
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            lts: Field::from_multiple_numeric(
                "Length of Text Segment",
                lts.iter()
                    .map(|l| parse_string_from_bytes(l))
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            numdes: Field::from_numeric(
                "Number of Data Extension Segments",
                parse_string_from_bytes(&numdes)?,
            ),
            ldshs: Field::from_multiple_numeric(
                "Length of Data Extension Segment Subheader",
                ldshs
                    .iter()
                    .map(|l| parse_string_from_bytes(l))
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            lds: Field::from_multiple_numeric(
                "Length of Data Extension Segment",
                lds.iter()
                    .map(|l| parse_string_from_bytes(l))
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            numres: Field::from_numeric(
                "Number of Reserved Extension Segments",
                parse_string_from_bytes(&numres)?,
            ),
            lreshs: Field::from_multiple_numeric(
                "Length of Reserved Extension Segment Subheader",
                lreshs
                    .iter()
                    .map(|l| parse_string_from_bytes(l))
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            lres: Field::from_multiple_numeric(
                "Length of Reserved Extension Segment",
                lres.iter()
                    .map(|l| parse_string_from_bytes(l))
                    .collect::<Result<Vec<String>, _>>()?,
            ),
            udhdl: Field::from_numeric(
                "User-Defined Header Data Length",
                parse_string_from_bytes(&udhdl)?,
            ),
            udhofl: Field::from_numeric(
                "User-Defined Header Overflow",
                parse_string_from_bytes(&udhofl)?,
            ),
            udhd: Field::from_alphanumeric(
                "User-Defined Header Data",
                parse_string_from_bytes(&udhd)?,
            ),
            xhdl: Field::from_numeric(
                "Extended Header Data Length",
                parse_string_from_bytes(&xhdl)?,
            ),
            xhdlofl: Field::from_numeric(
                "Extended Header Data Overflow",
                parse_string_from_bytes(&xhdlofl)?,
            ),
            xhd: Field::from_alphanumeric("Extended Header Data", parse_string_from_bytes(&xhd)?),
        })
    }
}

impl PrettyPrint for FileHeader {}

impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}
