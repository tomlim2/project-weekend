use bevy::audio::{AudioSource, PlaybackSettings};
use bevy::prelude::*;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub enum SfxKind {
    Shot,
    Hit,
    Bomb,
    Death,
}

#[derive(Message)]
pub struct PlaySfx(pub SfxKind);

#[derive(Resource)]
pub struct SfxLibrary {
    pub shot: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub bomb: Handle<AudioSource>,
    pub death: Handle<AudioSource>,
}

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlaySfx>()
            .add_systems(Startup, init_library)
            .add_systems(Update, dispatch);
    }
}

fn init_library(mut commands: Commands, mut audio: ResMut<Assets<AudioSource>>) {
    commands.insert_resource(SfxLibrary {
        shot: audio.add(AudioSource {
            bytes: make_sine_wav(880.0, 0.05, 30.0, 0.18).into(),
        }),
        hit: audio.add(AudioSource {
            bytes: make_sine_wav(220.0, 0.10, 18.0, 0.40).into(),
        }),
        bomb: audio.add(AudioSource {
            bytes: make_sine_wav(110.0, 0.40, 4.0, 0.55).into(),
        }),
        death: audio.add(AudioSource {
            bytes: make_sine_wav(65.0, 0.60, 3.0, 0.55).into(),
        }),
    });
}

fn dispatch(
    mut commands: Commands,
    mut events: MessageReader<PlaySfx>,
    library: Option<Res<SfxLibrary>>,
) {
    let Some(library) = library else {
        return;
    };
    for ev in events.read() {
        let handle = match ev.0 {
            SfxKind::Shot => library.shot.clone(),
            SfxKind::Hit => library.hit.clone(),
            SfxKind::Bomb => library.bomb.clone(),
            SfxKind::Death => library.death.clone(),
        };
        commands.spawn((AudioPlayer::new(handle), PlaybackSettings::DESPAWN));
    }
}

fn make_sine_wav(freq: f32, duration: f32, decay: f32, vol: f32) -> Arc<[u8]> {
    const SR: u32 = 44_100;
    let n = (SR as f32 * duration) as u32;
    let bytes_per_sample: u32 = 2;
    let data_size = n * bytes_per_sample;
    let chunk_size = 36 + data_size;
    let mut buf: Vec<u8> = Vec::with_capacity(44 + data_size as usize);
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&chunk_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes()); // PCM
    buf.extend_from_slice(&1u16.to_le_bytes()); // mono
    buf.extend_from_slice(&SR.to_le_bytes());
    buf.extend_from_slice(&(SR * bytes_per_sample).to_le_bytes()); // byte rate
    buf.extend_from_slice(&(bytes_per_sample as u16).to_le_bytes()); // block align
    buf.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_size.to_le_bytes());
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let env = (-decay * t).exp();
        let s = (t * freq * std::f32::consts::TAU).sin() * env * vol;
        let s_i16 = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        buf.extend_from_slice(&s_i16.to_le_bytes());
    }
    Arc::from(buf.into_boxed_slice())
}
