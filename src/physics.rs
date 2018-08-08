use geometry;
use js;
use map;
use na;
use nc::shape::{Cuboid, ShapeHandle, TriMesh};
use np::{
    object::{BodyHandle, Material},
    volumetric::Volumetric,
    world::World,
};
use std::{f32::consts::FRAC_PI_2, sync::Mutex};

/// In m/s^2, as defined by la Conférence générale des poids et mesures
pub const STANDARD_GRAVITY: f32 = 9.80665;

lazy_static! {
    pub static ref WORLD: Mutex<World<f32>> =
        Mutex::new(World::new(time_in_sec));
    pub static ref PLAYER: Mutex<BodyHandle> =
        Mutex::new(BodyHandle::ground());
}

pub fn init_world(map_data: &map::Map, player_pos: &na::Point3<f32>) {
    let mut world = WORLD.lock().unwrap();

    world.set_gravity(na::Vector3::y() * -STANDARD_GRAVITY);

    let cuboid = ShapeHandle::new(Cuboid::new(na::Vector3::repeat(1.0)));
    let cuboid_inertia = cuboid.inertia(1.0);
    let cuboid_center_of_mass = cuboid.center_of_mass();
    let player_body_handle = world.add_rigid_body(
        na::Isometry3::new(player_pos.coords, na::Vector3::zeros()),
        cuboid_inertia,
        cuboid_center_of_mass,
    );

    *PLAYER.lock().unwrap() = player_body_handle;

    const COLLIDER_MARGIN: f32 = 0.025;
    let _player_collider = world.add_collider(
        COLLIDER_MARGIN,
        ShapeHandle::new(Cuboid::new(na::Vector3::repeat(
            1.0 - COLLIDER_MARGIN,
        ))),
        player_body_handle,
        na::Isometry3::identity(),
        Material::default(),
    );

    let hex_prism_verts = geometry::HEXAGONAL_PRISM_VERTS
        .exact_chunks(3)
        .map(|s| na::Point3::new(s[0], s[1], s[2]))
        .collect::<Vec<_>>();
    let hex_prism_indices = geometry::HEXAGONAL_PRISM_INDICES
        .exact_chunks(3)
        .map(|s| na::Point3::new(s[0], s[1], s[2]))
        .collect::<Vec<_>>();

    for (hex, (x, y)) in map_data.iter() {
        world.add_collider(
            COLLIDER_MARGIN,
            ShapeHandle::new(TriMesh::new(
                hex_prism_verts.clone(),
                hex_prism_indices.clone(),
                None,
            )),
            BodyHandle::ground(),
            na::Isometry3::new(
                na::Vector3::new(*x, hex.height, -(*y)),
                na::Vector3::new(-FRAC_PI_2, 0.0, 0.0),
            ),
            Material::default(),
        );
    }
}

#[inline]
fn time_in_sec() -> f64 {
    js::now() / 1000.0
}
