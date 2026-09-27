mod essentials;

use crate::essentials::{pan::PanParams, volume::VolumeParams};
use truce::prelude::*;
use truce_gui_types::layout::{GridLayout, knob, widgets};

use crate::essentials::pan::get_lr_pan_values;

#[derive(Params)]
pub struct WildwheatParams {
    #[nested]
    pub volume: VolumeParams,

    #[nested]
    pub pan: PanParams,
}

// The plugin struct is its own DSP state (`type DspState = Self`). The
// shell owns it and preserves it across a hot-reload, so a code-only
// reload keeps reverb tails and oscillator phase alive.
#[derive(Default)]
pub struct Wildwheat {
    // Per-instance DSP state - filters, delay lines, phase counters.
    // Fields need `Default`. Add them as your DSP grows.
}

impl PluginLogic for Wildwheat {
    type Params = WildwheatParams;
    type DspState = Self;

    fn process(
        _state: &mut Self::DspState,
        params: &Self::Params,
        buffer: &mut AudioBuffer,
        _events: &EventList,
        _context: &mut ProcessContext,
    ) -> ProcessStatus {
        for i in 0..buffer.num_samples() {
            let gain = db_to_linear(params.volume.read());
            let pan = params.pan.read();

            let (gain_l, gain_r) = get_lr_pan_values(gain, pan);

            for ch in 0..buffer.channels() {
                let (inp, out) = buffer.io(ch);
                let current_gain = if ch == 0 { gain_l } else { gain_r };
                out[i] = inp[i] * current_gain;
            }
        }
        ProcessStatus::Normal
    }

    fn editor(params: Arc<WildwheatParams>) -> Box<dyn Editor> {
        let volume_knob = knob(params.volume.id(), "gain");
        let pan_knob = knob(params.pan.id(), "pan");

        truce_gui::default_editor(
            params.clone(),
            GridLayout::build(vec![widgets(vec![volume_knob, pan_knob])]),
        )
    }
}

truce::plugin! {
    logic: Wildwheat,
    params: WildwheatParams,
}

// Installs the real-time allocation checker under `--features rt-paranoid`
// (a no-op otherwise). Wrap a driver run in `assert_no_audio_alloc` to
// fail a test if `process` ever allocates. See the audio-testing guide.
truce::enable_rt_paranoid!();
