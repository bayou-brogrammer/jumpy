use macroquad::{
    experimental::{
        collections::storage,
        scene::{self, RefMut},
    },
    telemetry,
};

use crate::Resources;

pub struct Fxses {}

impl scene::Node for Fxses {
    fn draw(_node: RefMut<Self>) {
        let mut resources = storage::get_mut::<Resources>();

        let _z = telemetry::ZoneGuard::new("draw particles");

        resources.hit_fxses.draw();
        resources.explosion_fxses.draw();
        resources.disarm_fxses.draw();
    }
}