pub struct AssetPaths {
    pub fira_sans: &'static str,
    pub audio_background: &'static str,
    pub audio_hi_1: &'static str,
    pub audio_hi_2: &'static str,
    pub audio_nope_1: &'static str,
    pub audio_nope_2: &'static str,
    pub audio_hi_3: &'static str,
    pub texture_bevy: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    fira_sans: "fonts/FiraSans-Bold.ttf",
    audio_background: "audio/background.ogg",
    audio_hi_1: "audio/hi1.ogg",
    audio_hi_2: "audio/hi2.ogg",
    audio_hi_3: "audio/hi3.ogg",
    audio_nope_1: "audio/nope_1.ogg",
    audio_nope_2: "audio/nope_2.ogg",
    texture_bevy: "textures/bevy.png",
};
