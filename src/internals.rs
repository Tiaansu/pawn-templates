use crate::plugin::PawnTemplates;

static mut GLOBAL_INDEX: usize = 0;

pub fn insert_template(pawn_templates: &mut PawnTemplates, template: liquid::Template) -> usize {
    unsafe {
        pawn_templates.pool.push(template);
        GLOBAL_INDEX += 1;
        GLOBAL_INDEX
    }
}

#[derive(Debug)]
pub enum ArgumentPairType {
    Invalid = 0,
    String = 1,
    Int = 2,
    Float = 3,
}

impl ArgumentPairType {
    pub fn from_i32(i: i32) -> Self {
        match i {
            1 => ArgumentPairType::String,
            2 => ArgumentPairType::Int,
            3 => ArgumentPairType::Float,
            _ => ArgumentPairType::Invalid,
        }
    }
}
