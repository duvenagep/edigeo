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
    id: String,
    block_type: BlockTypes,
    entries: Vec<Line>,
}

impl Block {
    fn new() {
        todo!()
    }
}
