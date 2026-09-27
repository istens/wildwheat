use truce::prelude::*;

#[derive(Params)]
pub struct PanParams {
    #[param(name = "Pan", range = "linear(-1, 1)", unit = "pan", smooth = "exp(5)")]
    pub pan: FloatParam,
}

impl PanParams {
    pub fn read(&self) -> f32 {
        self.pan.read()
    }

    pub fn id(&self) -> u32 {
        self.pan.id()
    }
}

pub fn get_lr_pan_values(gain: f32, pan: f32) -> (f32, f32) {
    let gain_l = gain * (1.0 - pan.max(0.0));
    let gain_r = gain * (1.0 + pan.min(0.0));

    (gain_l, gain_r)
}
