use std::cmp::max;
use std::fmt::Display;
use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct FileHeader {
    fhdr: Vec<u8>,
    fver: Vec<u8>,
    clevel: Vec<u8>,
    stype: Vec<u8>,
    ostaid: Vec<u8>,
    fdt: Vec<u8>,
    ftitle: Vec<u8>,
    fsclas: Vec<u8>,
    fsclsy: Vec<u8>,
    fscode: Vec<u8>,
    fsctlh: Vec<u8>,
    fsrel: Vec<u8>,
    fsdctp: Vec<u8>,
    fsdcdt: Vec<u8>,
    fsdcxm: Vec<u8>,
    fsdg: Vec<u8>,
    fsdgdt: Vec<u8>,
    fscltx: Vec<u8>,
    fscatp: Vec<u8>,
    fscaut: Vec<u8>,
    fscrsn: Vec<u8>,
    fssrdt: Vec<u8>,
    fsctln: Vec<u8>,
    fscop: Vec<u8>,
    fscpys: Vec<u8>,
    encryp: Vec<u8>,
    fbkgc: Vec<u8>,
    oname: Vec<u8>,
    ophone: Vec<u8>,
    fl: Vec<u8>,
    hl: Vec<u8>,
    numi: Vec<u8>,
    lishs: Vec<Vec<u8>>,
    lis: Vec<Vec<u8>>,
    nums: Vec<u8>,
    lsshs: Vec<Vec<u8>>,
    lss: Vec<Vec<u8>>,
    numx: Vec<u8>,
    numt: Vec<u8>,
    ltshs: Vec<Vec<u8>>,
    lts: Vec<Vec<u8>>,
    numdes: Vec<u8>,
    ldshs: Vec<Vec<u8>>,
    lds: Vec<Vec<u8>>,
    numres: Vec<u8>,
    lreshs: Vec<Vec<u8>>,
    lres: Vec<Vec<u8>>,
    udhdl: Vec<u8>,
    udhofl: Vec<u8>,
    udhd: Vec<u8>,
    xhdl: Vec<u8>,
    xhdlofl: Vec<u8>,
    xhd: Vec<u8>,
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
        let number_of_image_segments: i32 = String::from_utf8(numi.clone())?
            .trim_start_matches('0')
            .parse()
            .unwrap_or(0);

        for _ in 0..number_of_image_segments {
            let mut lish = vec![0; 6];
            let mut li = vec![0; 10];
            file.read(&mut lish)?;
            file.read(&mut li)?;
            lishs.push(lish);
            lis.push(li);
        }

        file.read(&mut nums)?;
        let number_of_graphic_segments: i32 = String::from_utf8(nums.clone())?
            .trim_start_matches('0')
            .parse()
            .unwrap_or(0);

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
        let number_of_text_segments: i32 = String::from_utf8(numt.clone())?
            .trim_start_matches('0')
            .parse()
            .unwrap_or(0);

        for _ in 0..number_of_text_segments {
            let mut ltsh = vec![0; 4];
            let mut lt = vec![0; 5];
            file.read(&mut ltsh)?;
            file.read(&mut lt)?;
            ltshs.push(ltsh);
            lts.push(lt);
        }

        file.read(&mut numdes)?;
        let number_of_data_extension_segments: i32 = String::from_utf8(numdes.clone())?
            .trim_start_matches('0')
            .parse()
            .unwrap_or(0);

        for _ in 0..number_of_data_extension_segments {
            let mut ldsh = vec![0; 4];
            let mut ld = vec![0; 9];
            file.read(&mut ldsh)?;
            file.read(&mut ld)?;
            ldshs.push(ldsh);
            lds.push(ld);
        }

        file.read(&mut numres)?;
        let number_of_reserved_extension_segments: i32 = String::from_utf8(numres.clone())?
            .trim_start_matches('0')
            .parse()
            .unwrap_or(0);

        for _ in 0..number_of_reserved_extension_segments {
            let mut lresh = vec![0; 4];
            let mut lre = vec![0; 7];
            file.read(&mut lresh)?;
            file.read(&mut lre)?;
            lreshs.push(lresh);
            lres.push(lre);
        }

        file.read(&mut udhdl)?;
        let udhd_length: usize = max(
            String::from_utf8(udhdl.clone())?
                .trim_matches('0')
                .parse()
                .unwrap_or(3)
                - 3,
            0,
        );

        if udhd_length != 0 {
            file.read(&mut udhofl)?;
        }

        let mut udhd = vec![0; udhd_length];
        file.read(&mut udhd)?;

        file.read(&mut xhdl)?;
        let xhd_length: usize = max(
            String::from_utf8(xhdl.clone())?
                .trim_matches('0')
                .parse()
                .unwrap_or(3)
                - 3,
            0,
        );

        if xhd_length != 0 {
            file.read(&mut xhdlofl)?;
        }

        let mut xhd = vec![0; xhd_length];
        file.read(&mut xhd)?;

        Ok(FileHeader {
            fhdr,
            fver,
            clevel,
            stype,
            ostaid,
            fdt,
            ftitle,
            fsclas,
            fsclsy,
            fscode,
            fsctlh,
            fsrel,
            fsdctp,
            fsdcdt,
            fsdcxm,
            fsdg,
            fsdgdt,
            fscltx,
            fscatp,
            fscaut,
            fscrsn,
            fssrdt,
            fsctln,
            fscop,
            fscpys,
            encryp,
            fbkgc,
            oname,
            ophone,
            fl,
            hl,
            numi,
            lishs,
            lis,
            nums,
            lsshs,
            lss,
            numx,
            numt,
            ltshs,
            lts,
            numdes,
            ldshs,
            lds,
            numres,
            lreshs,
            lres,
            udhdl,
            udhofl,
            udhd,
            xhdl,
            xhdlofl,
            xhd,
        })
    }

    fn pretty_print(&self) -> String {
        let mut pretty = String::new();
        pretty.push_str(&make_label("File Profile Name", &self.fhdr));
        pretty.push_str(&make_label("File Version", &self.fver));
        pretty.push_str(&make_label("Complexity Level", &self.clevel));
        pretty.push_str(&make_label("Standard Type", &self.stype));
        pretty.push_str(&make_label("Originating Station Identifier", &self.ostaid));
        pretty.push_str(&make_label("File Date and Time", &self.fdt));
        pretty.push_str(&make_label("File Title", &self.ftitle));
        pretty.push_str(&make_label("File Security Classification", &self.fsclas));
        pretty.push_str(&make_label(
            "File Security Classification System",
            &self.fsclsy,
        ));
        pretty.push_str(&make_label("File Codewords", &self.fscode));
        pretty.pop();
        pretty
    }
}

impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

fn make_label(description: &str, value: &Vec<u8>) -> String {
    format!("    {}: {}\n", description, parse_string(value).unwrap())
}

fn parse_string(vec: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    String::from_utf8(vec.clone()).map_err(Into::into)
}

/*
fn parse_number(vec: &Vec<u8>) -> Result<i32, Box<dyn std::error::Error>> {
    let s = parse_string(vec)?.trim_start_matches('0').to_owned();
    if s.is_empty() {
        Ok(0)
    } else {
        s.parse::<i32>().map_err(Into::into)
    }
}
*/
