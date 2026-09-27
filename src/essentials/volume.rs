use truce::prelude::*;

#[derive(Params)]
pub struct VolumeParams {
    #[param(
        name = "Gain",
        range = "linear(-60, 6)",
        unit = "dB",
        smooth = "exp(5)"
    )]
    pub gain: FloatParam,
}

impl VolumeParams {
    pub fn read(&self) -> f32 {
        self.gain.read()
    }

    pub fn id(&self) -> u32 {
        self.gain.id()
    }
}
