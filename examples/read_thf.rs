use edigeo::{read_lines_efficient, EdigeoDir};

fn main() {
    println!("Hello World");
    let dir = "data/edigeo-740240000A01/";

    let e = EdigeoDir::extract_files(dir);
    println!("{:#?}", &e);

    if let Ok(lines) = read_lines_efficient(e.thf) {
        let header = Header::parse_header(lines);
        println!("{:?}", header);
        // for line in lines.flatten() {
        //     println!("{}", line);
        // }
    }

    // let l = "BOMT 12:E0000A01.THF";
    // let (x, _y) = parse_header(l);

    // println!("{:?}", x);
}

#[derive(Debug, Default)]
pub struct Header {
    code: Vec<String>,
    value_type: Vec<String>,
    value_format: Vec<String>,
    value_size: Vec<String>,
}

impl Header {
    pub fn parse_header(line: Vec<String>) -> Self {
        Self {
            code: line[0..3].to_vec(),
            value_type: line[3..4].to_vec(),
            value_format: line[4..5].to_vec(),
            value_size: line[5..7].to_vec(),
        }
    }
}
