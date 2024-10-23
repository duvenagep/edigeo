use edigeo::{read_lines_efficient, EdigeoDir};

fn main() {
    println!("Hello World");
    let dir = "data/edigeo-740240000A01/";

    let e = EdigeoDir::extract_files(dir);
    // println!("{:#?}", &e);

    // let data = edigeo::read_bytes(&e.geo);
    // println!("{}", data);

    if let Ok(lines) = read_lines_efficient(e.thf) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }

    // let lines = read_lines(&e.geo);

    // let header = lines.get(0).unwrap();
    // println!("{:?}", header);

    // let thf = edigeo::read_bytes(&e.thf);
    // println!("{}", thf);
}
