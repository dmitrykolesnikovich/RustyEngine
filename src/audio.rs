use crate::prelude::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use std::array::IntoIter;

#[derive(Default)]
pub struct AudioManager {
    sfx_queue: Vec<SfxPreset>,
    music_queue: Vec<MusicPreset>,
    playing: AudioChannel,
}

impl AudioManager {
    pub fn play_sfx(&mut self, sfx_preset: SfxPreset) {
        self.sfx_queue.push(sfx_preset);
    }
    pub fn play_music(&mut self, music_preset: MusicPreset) {
        self.music_queue.push(music_preset);
    }
}

#[derive(Default)]
pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(queue_managed_audio_system.system());
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SfxPreset {
    Click,
    Confirmation1,
    Confirmation2,
    Congratulations,
    Forcefield1,
    Forcefield2,
    Impact1,
    Impact2,
    Impact3,
    Jingle1,
    Jingle2,
    Jingle3,
    Minimize1,
    Minimize2,
    Switch1,
    Switch2,
    Tones1,
    Tones2,
}

impl SfxPreset {
    fn to_path(self) -> &'static str {
        match self {
            SfxPreset::Click => "audio/sfx/click.ogg",
            SfxPreset::Confirmation1 => "audio/sfx/confirmation1.ogg",
            SfxPreset::Confirmation2 => "audio/sfx/confirmation2.ogg",
            SfxPreset::Congratulations => "audio/sfx/congratulations.ogg",
            SfxPreset::Forcefield1 => "audio/sfx/forcefield1.ogg",
            SfxPreset::Forcefield2 => "audio/sfx/forcefield2.ogg",
            SfxPreset::Impact1 => "audio/sfx/impact1.ogg",
            SfxPreset::Impact2 => "audio/sfx/impact2.ogg",
            SfxPreset::Impact3 => "audio/sfx/impact3.ogg",
            SfxPreset::Jingle1 => "audio/sfx/jingle1.ogg",
            SfxPreset::Jingle2 => "audio/sfx/jingle2.ogg",
            SfxPreset::Jingle3 => "audio/sfx/jingle3.ogg",
            SfxPreset::Minimize1 => "audio/sfx/minimize1.ogg",
            SfxPreset::Minimize2 => "audio/sfx/minimize2.ogg",
            SfxPreset::Switch1 => "audio/sfx/switch1.ogg",
            SfxPreset::Switch2 => "audio/sfx/switch2.ogg",
            SfxPreset::Tones1 => "audio/sfx/tones1.ogg",
            SfxPreset::Tones2 => "audio/sfx/tones2.ogg",
        }
    }

    pub fn variant_iter() -> IntoIter<SfxPreset, 18> {
        static SFX_PRESETS: [SfxPreset; 18] = [
            SfxPreset::Click,
            SfxPreset::Confirmation1,
            SfxPreset::Confirmation2,
            SfxPreset::Congratulations,
            SfxPreset::Forcefield1,
            SfxPreset::Forcefield2,
            SfxPreset::Impact1,
            SfxPreset::Impact2,
            SfxPreset::Impact3,
            SfxPreset::Jingle1,
            SfxPreset::Jingle2,
            SfxPreset::Jingle3,
            SfxPreset::Minimize1,
            SfxPreset::Minimize2,
            SfxPreset::Switch1,
            SfxPreset::Switch2,
            SfxPreset::Tones1,
            SfxPreset::Tones2,
        ];
        IntoIter::new(SFX_PRESETS)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MusicPreset {
    ArcadeFantasy,
    Classy8Bit,
    MysteriousMagic,
    WhimsicalPopsicle,
}

impl MusicPreset {
    fn to_path(self) -> &'static str {
        match self {
            MusicPreset::ArcadeFantasy => "audio/music/Arcade Fantasy.oga",
            MusicPreset::Classy8Bit => "audio/music/Classy 8-Bit.oga",
            MusicPreset::MysteriousMagic => "audio/music/Mysterious Magic.oga",
            MusicPreset::WhimsicalPopsicle => "audio/music/Whimsical Popsicle.oga",
        }
    }

    pub fn variant_iter() -> IntoIter<MusicPreset, 4> {
        static MUSIC_PRESETS: [MusicPreset; 4] = [
            MusicPreset::ArcadeFantasy,
            MusicPreset::Classy8Bit,
            MusicPreset::MysteriousMagic,
            MusicPreset::WhimsicalPopsicle,
        ];
        IntoIter::new(MUSIC_PRESETS)
    }
}

pub fn queue_managed_audio_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut game_state: ResMut<GameState>,
) {
    for sfx in game_state.audio_manager.sfx_queue.drain(..) {
        let sfx_handle = asset_server.load(sfx.to_path());
        audio.play(sfx_handle);
    }
    let playing = game_state.audio_manager.playing.clone();
    let mut new_playing = playing.clone();
    for music in game_state.audio_manager.music_queue.drain(..) {
        let music_path = music.to_path();
        let music_handle = asset_server.load(music_path);
        audio.stop_channel(&playing);
        new_playing = AudioChannel::new(music_path.into());
        audio.play_looped_in_channel(music_handle, &new_playing);
    }
    game_state.audio_manager.playing = new_playing;
}