// Block type, Registry

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BlockType {
    #[default]
    Air,
    Grass,
    Dirt, 
    Stone
}

impl BlockType {
    pub fn is_solid(&self) -> bool {
        *self == BlockType::Air
    }
}