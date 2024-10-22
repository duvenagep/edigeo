use edigeo::EdigeoDir;

fn main() {
    println!("Hello World");
    let dir = "data/edigeo-740240000A01/";

    let e = EdigeoDir::extract_files(dir);
    println!("{:#?}", e);

    let data = edigeo::read(e.geo);
    println!("{}", data);

    let thf = edigeo::read_bytes(e.thf);
    println!("{}", thf);
}
