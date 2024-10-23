use edigeo::EdigeoDir;

#[test]
fn edigeo_read_full_dir() {
    let full_dir_path = "data/edigeo-740240000A01/";
    let e = EdigeoDir::extract_files(full_dir_path);

    let all_required = !e.geo.is_empty()
        || !e.qal.is_empty()
        || !e.thf.is_empty()
        || !e.t1.is_empty()
        || !e.t2.is_empty()
        || !e.t3.is_empty()
        || !e.s1.is_empty();

    // Assert that min required files are present
    assert!(all_required);
}

#[test]
fn edigeo_read_thf() {
    let full_dir_path = "data/edigeo-740240000A01/";
    let e = EdigeoDir::extract_files(full_dir_path);

    assert_eq!("data/edigeo-740240000A01/E0000A01.THF", e.thf);
}
