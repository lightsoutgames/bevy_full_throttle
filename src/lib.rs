use bevy::prelude::*;
#[cfg(windows)]
use windows::Win32::System::{Power, SystemServices::GUID_MIN_POWER_SAVINGS};

fn setup() {
    #[cfg(windows)]
    unsafe {
        Power::PowerSetActiveScheme(None, &GUID_MIN_POWER_SAVINGS);
    }
}

pub struct FullThrottlePlugin;

impl Plugin for FullThrottlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
