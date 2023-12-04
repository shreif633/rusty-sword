use bevy::prelude::*;
use bevy::diagnostic::*;

pub struct Framerate;

impl Plugin for Framerate {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Update, log_framerate);
    }
}

fn log_framerate(diagnostics: Res<DiagnosticsStore>) {
    if let Some(value) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        println!("fps: {}", value)
    }
}