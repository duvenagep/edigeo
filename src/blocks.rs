use crate::Line;

#[derive(Debug)]
pub enum BlockTypes {
    FEA,
    LNK,
    PNO,
    PAR,
    PFE,
    GEO,
    QUP,
}

#[derive(Debug)]
struct Block {
    block_type: BlockTypes,
    entries: Vec<Line>,
}

impl Block {
    fn new() {
        todo!()
    }
}
