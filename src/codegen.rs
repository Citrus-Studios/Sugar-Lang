use crate::rid::RIDData;

pub struct CodeGen {
    rid: Vec<RIDData>,
}

impl CodeGen {
    pub fn new(rid: Vec<RIDData>) -> Self {
        Self { rid }
    }
    pub fn run(mut self) -> Result<(), String> {
        Ok(())
    }
}
