use bevy::{prelude::*, window::WindowFocused};
#[cfg(windows)]
use windows::{
    core::GUID,
    Win32::System::{Power, SystemServices::GUID_MIN_POWER_SAVINGS},
};

#[derive(Clone, Copy, Default)]
pub struct FullThrottleConfig {
    pub restore_original_scheme_on_unfocus: bool,
}

#[cfg(windows)]
#[derive(Deref, DerefMut)]
struct DefaultScheme(GUID);

#[cfg(not(windows))]
struct DefaultScheme();

#[allow(unused_mut, unused_variables)]
fn setup(mut commands: Commands) {
    #[cfg(windows)]
    unsafe {
        let mut active: *mut GUID = std::ptr::null_mut();
        Power::PowerGetActiveScheme(None, &mut active);
        if let Some(active) = active.as_ref() {
            let scheme = DefaultScheme(*active);
            commands.insert_resource(scheme);
        }
    }
    #[cfg(not(windows))]
    commands.insert_resource(DefaultScheme());
}

#[allow(unused_variables)]
fn focus_change(
    config: Option<Res<FullThrottleConfig>>,
    mut focus: EventReader<WindowFocused>,
    scheme: Res<DefaultScheme>,
) {
    let config: FullThrottleConfig = config
        .map(|v| *v)
        .unwrap_or_else(|| FullThrottleConfig::default());
    for event in focus.iter() {
        if event.focused {
            #[cfg(windows)]
            unsafe {
                Power::PowerSetActiveScheme(None, &GUID_MIN_POWER_SAVINGS);
            }
        } else {
            #[cfg(windows)]
            if config.restore_original_scheme_on_unfocus {
                unsafe {
                    Power::PowerSetActiveScheme(None, &**scheme);
                }
            }
        }
    }
}

pub struct FullThrottlePlugin;

impl Plugin for FullThrottlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(focus_change);
    }
}
