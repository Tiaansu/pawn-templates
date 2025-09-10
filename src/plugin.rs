use log::info;
use samp::plugin::SampPlugin;

pub struct PawnTemplates {
    pub pool: Vec<liquid::Template>,
}

impl SampPlugin for PawnTemplates {
    fn on_load(&mut self) {
        info!("pawn_templates Loaded!");
    }

    fn on_unload(&mut self) {
        info!("pawn_templates Unloaded!");
    }
}
