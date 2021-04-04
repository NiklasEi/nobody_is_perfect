use crate::loading::AudioAssets;
use crate::player::{BefriendEvent, DyingEvent, LevelUpEvent, NopeEvent};
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};
use rand::random;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AudioChannels {
            background: AudioChannel::new("background".to_owned()),
            effects: AudioChannel::new("effects".to_owned()),
        })
        .add_plugin(AudioPlugin)
        .add_system_set(
            SystemSet::on_enter(GameState::RenderBackground).with_system(start_audio.system()),
        )
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(lets_go_audio.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(befriend_audio.system())
                .with_system(nope_audio.system())
                .with_system(level_up_audio.system())
                .with_system(dying_audio.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(stop_audio.system()));
    }
}

struct AudioChannels {
    background: AudioChannel,
    effects: AudioChannel,
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.set_volume_in_channel(0.2, &channels.background);
    audio.play_looped_in_channel(audio_assets.background.clone(), &channels.background);

    audio.set_volume_in_channel(0.3, &channels.effects);
}

fn befriend_audio(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
    mut events: EventReader<BefriendEvent>,
) {
    if let Some(_event) = events.iter().last() {
        let random_value = random::<f32>();
        if random_value > 0.66 {
            audio.play_in_channel(audio_assets.hi_1.clone(), &channels.effects);
        } else if random_value > 0.33 {
            audio.play_in_channel(audio_assets.hi_2.clone(), &channels.effects);
        } else {
            audio.play_in_channel(audio_assets.hi_3.clone(), &channels.effects);
        }
    }
}

fn nope_audio(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
    mut events: EventReader<NopeEvent>,
) {
    if let Some(_event) = events.iter().last() {
        let random_value = random::<f32>();
        if random_value > 0.5 {
            audio.play_in_channel(audio_assets.nope_1.clone(), &channels.effects);
        } else {
            audio.play_in_channel(audio_assets.nope_2.clone(), &channels.effects);
        }
    }
}

fn dying_audio(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
    mut events: EventReader<DyingEvent>,
) {
    if let Some(_event) = events.iter().last() {
        audio.play_in_channel(audio_assets.dying.clone(), &channels.effects);
    }
}

fn level_up_audio(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
    mut events: EventReader<LevelUpEvent>,
) {
    if let Some(_event) = events.iter().last() {
        audio.play_in_channel(audio_assets.level_up.clone(), &channels.effects);
    }
}

fn lets_go_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.play_in_channel(audio_assets.lets_go.clone(), &channels.effects);
}

fn stop_audio(audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.stop_channel(&channels.effects);
}
