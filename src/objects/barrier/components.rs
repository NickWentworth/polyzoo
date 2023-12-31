use super::{
    cycle::{BarrierCycleUtil, UndirectedGraph},
    BarrierData,
};
use bevy::prelude::*;

pub struct BarrierComponentPlugin;
impl Plugin for BarrierComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PossibleBarrierCycle>()
            .add_systems(Update, (mark_fence_movement, handle_fence_movement))
            // on_possible_cycle needs to ensure that the placed barriers are all actually spawned in when it runs
            .add_systems(PostUpdate, on_possible_cycle);
    }
}

/// Instance of a barrier's post in the world
#[derive(Component, Clone)]
pub struct BarrierPost {
    pub data: Handle<BarrierData>,
    /// List of all connecting fence entities
    pub fences: Vec<Entity>,
}

/// Instance of a barrier's fence in the world
#[derive(Component, Clone)]
pub struct BarrierFence {
    pub data: Handle<BarrierData>,
    /// Connection between posts that this fence serves
    pub connection: [Entity; 2],
}

/// Event to be called whenever a possible cycle was created between barrier posts
#[derive(Event)]
pub struct PossibleBarrierCycle {
    pub root: Entity,
}

/// Marks a fence as changed if either of its connected posts are moved
fn mark_fence_movement(
    moved_posts: Query<&BarrierPost, Changed<Transform>>,
    mut fences: Query<&mut BarrierFence>,
) {
    for moved_post in moved_posts.iter() {
        for &fence_entity in moved_post.fences.iter() {
            if let Ok(mut fence) = fences.get_mut(fence_entity) {
                fence.set_changed();
            }
        }
    }
}

/// Handles adjusting the fence's transform if its `BarrierFence` component has been changed
fn handle_fence_movement(
    mut moved_fences: Query<(&BarrierFence, &mut Transform), Changed<BarrierFence>>,
    all_posts: Query<&Transform, (With<BarrierPost>, Without<BarrierFence>)>,
) {
    for (fence, mut transform) in moved_fences.iter_mut() {
        // get positions of both posts
        let Ok(from) = all_posts.get(fence.connection[0]).map(|t| t.translation) else { continue };
        let Ok(to) = all_posts.get(fence.connection[1]).map(|t| t.translation) else { continue };

        // and calculate transform values for this fence
        let midpoint = from.lerp(to, 0.5);
        let angle = -f32::atan2(to.z - from.z, to.x - from.x);
        let distance = from.distance(to);

        *transform = Transform {
            translation: midpoint,
            rotation: Quat::from_rotation_y(angle),
            scale: Vec3::new(distance, 1.0, 1.0),
        };
    }
}

fn on_possible_cycle(
    mut possible_cycles: EventReader<PossibleBarrierCycle>,
    cycle_util: BarrierCycleUtil,
) {
    for possible_cycle in possible_cycles.iter() {
        match cycle_util.get_cycle(possible_cycle.root) {
            Some(list) => {
                println!("Cycle found! {} barriers in cycle:", list.len());
                println!("{:?}", list);
            }
            None => println!("Cycle not found"),
        }
    }
}
