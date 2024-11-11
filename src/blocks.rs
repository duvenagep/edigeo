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

pub trait ParseBlock {
    fn parse_block(&self) -> Self;
}

pub struct THFFile {
    data: Vec<u8>,
    blocks: Vec<Block>,
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
