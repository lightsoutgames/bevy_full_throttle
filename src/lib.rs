use bevy::{app::AppExit, prelude::*, window::WindowFocused};
#[cfg(windows)]
use windows::{
    core::GUID,
    Win32::System::{Power, SystemServices::GUID_MIN_POWER_SAVINGS},
};

#[cfg(windows)]
#[derive(Resource, Deref, DerefMut)]
struct DefaultScheme(GUID);

#[cfg(not(windows))]
#[derive(Resource)]
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
            ctrlc::set_handler(move || {
                Power::PowerSetActiveScheme(None, Some(active));
                std::process::exit(1);
            })
            .expect("Failed to set exit handler");
        }
    }
    #[cfg(not(windows))]
    commands.insert_resource(DefaultScheme());
}

#[allow(unused_variables)]
fn focus_change(
    config: Res<FullThrottlePlugin>,
    mut focus: EventReader<WindowFocused>,
    scheme: Res<DefaultScheme>,
) {
    for event in focus.iter() {
        if event.focused {
            #[cfg(windows)]
            unsafe {
                Power::PowerSetActiveScheme(None, Some(&GUID_MIN_POWER_SAVINGS));
            }
        } else {
            #[cfg(windows)]
            if config.restore_original_scheme_on_unfocus {
                unsafe {
                    Power::PowerSetActiveScheme(None, Some(&**scheme));
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn exit(mut exit: EventReader<AppExit>, scheme: Res<DefaultScheme>) {
    for event in exit.iter() {
        #[cfg(windows)]
        unsafe {
            Power::PowerSetActiveScheme(None, Some(&**scheme));
        }
    }
}

#[derive(Resource, Clone, Copy, Default)]
pub struct FullThrottlePlugin {
    pub restore_original_scheme_on_unfocus: bool,
}

impl Plugin for FullThrottlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(*self)
            .add_startup_system(setup)
            .add_system(focus_change)
            .add_system(exit.in_base_set(CoreSet::PostUpdate));
    }
}
