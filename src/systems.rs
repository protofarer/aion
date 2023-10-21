use std::time::Duration;

use crate::components::*;
use crate::game::WindowDims;
use crate::time::Dt;
use legion::*;
use nalgebra_glm::Vec2;

#[system(for_each)]
pub fn update_positions(
    transform: &mut TransformCpt,
    rigidbody: &mut RigidBodyCpt,
    #[resource] dt: &Duration,
) {
    // transform.position.x += rigidbody.velocity.x * dt.0;
    // transform.position.y += rigidbody.velocity.y * dt.0;
    // ! does this work?
    transform.position += rigidbody.velocity * dt.as_secs_f32();
}

#[system(for_each)]
pub fn process_translational_input(
    input: &TranslationalInputCpt,
    movestats: &MovementStatsCpt,
    rigidbody: &mut RigidBodyCpt,
) {
    match input.direction {
        Some(Direction::E) => {
            rigidbody.velocity.x = movestats.speed as f32;
            rigidbody.velocity.y = 0f32;
        }
        Some(Direction::SE) => {
            let component_speed_f32 = movestats.speed as f32;
            let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
            rigidbody.velocity.x = component_speed;
            rigidbody.velocity.y = component_speed;
        }
        Some(Direction::S) => {
            rigidbody.velocity.x = 0f32;
            rigidbody.velocity.y = movestats.speed as f32;
        }
        Some(Direction::SW) => {
            let component_speed_f32 = movestats.speed as f32;
            let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
            rigidbody.velocity.x = -component_speed;
            rigidbody.velocity.y = component_speed;
        }
        Some(Direction::W) => {
            rigidbody.velocity.x = -(movestats.speed as f32);
            rigidbody.velocity.y = 0f32;
        }
        Some(Direction::NW) => {
            let component_speed_f32 = movestats.speed as f32;
            let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
            rigidbody.velocity.x = -component_speed;
            rigidbody.velocity.y = -component_speed;
        }
        Some(Direction::N) => {
            rigidbody.velocity.x = 0f32;
            rigidbody.velocity.y = -(movestats.speed as f32);
        }
        Some(Direction::NE) => {
            let component_speed_f32 = movestats.speed as f32;
            let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
            rigidbody.velocity.x = component_speed;
            rigidbody.velocity.y = -component_speed;
        }
        None => {
            rigidbody.velocity = Vec2::new(0.0, 0.0);
        }
    }
}

#[system(for_each)]
pub fn collision(
    transform: &mut TransformCpt,
    rigidbody: &mut RigidBodyCpt,
    collision_area: &CollisionAreaCpt,
    #[resource] window_dims: &WindowDims,
) {
    if transform.position.x + collision_area.w as f32 >= window_dims.w as f32
        || transform.position.x < 0f32
    {
        rigidbody.velocity.x = -rigidbody.velocity.x;
    }
    if transform.position.x < 0f32 {
        transform.position.x = 0f32;
    } else if transform.position.x + collision_area.w as f32 >= window_dims.w as f32 {
        transform.position.x = (window_dims.w - collision_area.w) as f32;
    }

    if transform.position.y + collision_area.h as f32 >= window_dims.h as f32
        || transform.position.y < 0f32
    {
        rigidbody.velocity.y = -rigidbody.velocity.y;
    }
    if transform.position.y < 0f32 {
        transform.position.y = 0f32;
    } else if transform.position.y + collision_area.h as f32 >= window_dims.h as f32 {
        transform.position.y = (window_dims.h - collision_area.h) as f32;
    }
}

// ? how to have this run only when input changes?
#[system(for_each)]
pub fn process_rotational_input(
    rotational_input: &RotationalInputCpt,
    move_stats: &MovementStatsCpt,
    transform: &mut TransformCpt,
    rigidbody: &mut RigidBodyCpt,
) {
    match rotational_input.turn_sign {
        Some(Turn::Right) => {
            transform.rotation =
                (transform.rotation + move_stats.turn_rate) % (2.0 * nalgebra_glm::pi::<f32>());
        }
        Some(Turn::Left) => {
            transform.rotation =
                (transform.rotation - move_stats.turn_rate) % (2.0 * nalgebra_glm::pi::<f32>());
        }
        _ => {}
    }
    match rotational_input.is_thrusting {
        true => {
            rigidbody.velocity.x = (transform.rotation).cos() * move_stats.speed as f32;
            rigidbody.velocity.y = (transform.rotation).sin() * move_stats.speed as f32;
        }
        false => {
            // test when decel cross 0 in either direction
            // todo don't hard set here, simply make no change, for maintaining momentum in prototype stage
            // todo repeats when no key is pressed...
            rigidbody.velocity = Vec2::default();
        }
    }
}

#[system(for_each)]
pub fn projectile_emission(
    pem: &ProjectileEmitter,
    proj: &Projectile,
    transform: &TransformCpt,
    rigidbody: &RigidBodyCpt,
) {
    // todo how to emit projectile on key press (player.is_firing_primary)
    // only fire if cooldown is up
}
