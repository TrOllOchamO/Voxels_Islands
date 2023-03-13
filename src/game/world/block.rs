// The Block struct represent a block by a u16 like this -> IIII IIII IIIU UOOO
// 11 bits for the block ID represented here with the letter "I"
// 3 bits for the block orientation represented with the letter "O"
// 2 bits unused marked as "U"

const AIR_BLOCK: u16 = 0x0000;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Block(pub u16);

#[derive(Default)]
pub enum BlockOrientation {
    #[default]
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

impl Block {
    pub fn new(block_id: u16, orientation: BlockOrientation) -> Self {
        let mut block = Self::default();
        block.set_orientation(orientation);
        block.set_id(block_id);
        block
    }

    pub fn default() -> Self {
        Block(AIR_BLOCK)
    }

    pub fn get_orientation(&self) -> BlockOrientation {
        let orientation_code = (self.0 << 13) >> 13;

        match orientation_code {
            0 => BlockOrientation::PositiveX,
            1 => BlockOrientation::NegativeX,
            2 => BlockOrientation::PositiveY,
            3 => BlockOrientation::NegativeY,
            4 => BlockOrientation::PositiveZ,
            5 => BlockOrientation::NegativeZ,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn get_id(&self) -> u16 {
        self.0 >> 5
    }

    #[inline]
    pub fn get_as_u16(&self) -> u16 {
        self.0
    }

    #[inline]
    pub fn set_orientation(&mut self, orientation: BlockOrientation) {
        self.0 &= 0xFFF8;

        match orientation {
            BlockOrientation::PositiveX => {}
            BlockOrientation::NegativeX => self.0 |= 0x1,
            BlockOrientation::PositiveY => self.0 |= 0x2,
            BlockOrientation::NegativeY => self.0 |= 0x3,
            BlockOrientation::PositiveZ => self.0 |= 0x4,
            BlockOrientation::NegativeZ => self.0 |= 0x5,
        }
    }

    pub fn set_id(&mut self, block_id: u16) {
        self.0 &= 0x001F;
        let id_shifted = block_id << 5;
        self.0 |= id_shifted;
    }
}
