use pumpkin_util::math::{position::BlockPos, vector3::Vector3};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

use crate::entity::living::LivingEntity;

#[derive(Default)]
pub struct Navigator {
    current_goal: Option<NavigatorGoal>,
}

pub struct NavigatorGoal {
    pub current_progress: Vector3<f64>,
    pub destination: Vector3<f64>,
    pub speed: f64,
    pub path: Vec<Vector3<f64>>,
    pub path_index: usize,
    #[cfg(feature = "pathfinding-debug")]
    pub debug_info: Option<DebugInfo>,
}

#[cfg(feature = "pathfinding-debug")]
pub struct DebugInfo {
    pub waypoints: Vec<Vector3<f64>>,
    pub goal_pos: Vector3<f64>,
}

impl Navigator {
    pub fn set_progress(&mut self, mut goal: NavigatorGoal) {
        // Initialize path if empty
        if goal.path.is_empty() {
            goal.path.push(goal.current_progress);
        }
        self.current_goal = Some(goal);
    }

    pub fn cancel(&mut self) {
        self.current_goal = None;
    }

    pub async fn tick(&mut self, entity: &LivingEntity) {
        if let Some(goal) = &mut self.current_goal {
            // Check if we reached the final destination
            let distance_to_dest = goal.current_progress.distance_to_vec(goal.destination);
            if distance_to_dest < 0.5 {
                self.current_goal = None;
                return;
            }

            // If we don't have a path or finished current path, calculate new path
            if goal.path.is_empty() || goal.path_index >= goal.path.len() {
                // Extract values to avoid borrow checker issues
                let current_progress = goal.current_progress;
                let destination = goal.destination;
                
                match calculate_path(entity, current_progress, destination).await {
                    Some(path) => {
                        goal.path = path;
                        goal.path_index = 0;
                        
                        #[cfg(feature = "pathfinding-debug")]
                        {
                            goal.debug_info = Some(DebugInfo {
                                waypoints: goal.path.clone(),
                                goal_pos: goal.destination,
                            });
                            visualize_path(entity, goal).await;
                        }
                    }
                    None => {
                        // No path found, cancel navigation
                        self.current_goal = None;
                        return;
                    }
                }
            }

            // Follow the path
            if let Some(next_waypoint) = goal.path.get(goal.path_index) {
                let direction = next_waypoint.sub(&goal.current_progress);
                let distance = direction.length();

                if distance < 0.3 {
                    // Reached waypoint, move to next
                    goal.path_index += 1;
                    
                    // Check if we need to jump for the next waypoint
                    if let Some(next_next) = goal.path.get(goal.path_index) {
                        if next_next.y > goal.current_progress.y + 0.5 {
                            // Need to jump
                            entity.jumping.store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                } else {
                    // Move towards waypoint
                    let movement = direction.normalize() * goal.speed.min(distance);
                    goal.current_progress += movement;
                    
                    entity.entity.set_pos(goal.current_progress);
                    entity.entity.send_pos().await;
                }
            }
        }
    }

    #[must_use]
    pub fn is_idle(&self) -> bool {
        self.current_goal.is_none()
    }
}

/// A* pathfinding algorithm
async fn calculate_path(
    entity: &LivingEntity,
    start: Vector3<f64>,
    goal: Vector3<f64>,
) -> Option<Vec<Vector3<f64>>> {
        let world = &entity.entity.world;
        
        // Priority queue for nodes to explore (min-heap based on f_score)
        let mut open_set = BinaryHeap::new();
        let mut g_score: HashMap<BlockPos, f64> = HashMap::new();
        let mut came_from: HashMap<BlockPos, BlockPos> = HashMap::new();
        let mut closed_set: HashSet<BlockPos> = HashSet::new();

        let start_pos = BlockPos(start.to_i32());
        let goal_pos = BlockPos(goal.to_i32());

        g_score.insert(start_pos, 0.0);
        let h_start = heuristic(start, goal);
        open_set.push(Node {
            position: start_pos,
            f_score: h_start,
        });

        let max_iterations = 1000; // Prevent infinite loops
        let mut iterations = 0;

        while let Some(current_node) = open_set.pop() {
            iterations += 1;
            if iterations > max_iterations {
                break;
            }

            let current = current_node.position;

            // Check if we reached the goal
            if current == goal_pos || current.distance_to(&goal_pos) < 2.0 {
                // Reconstruct path
                let mut path = Vec::new();
                let mut current_path = current;
                
                while let Some(&prev) = came_from.get(&current_path) {
                    path.push(current_path.0.to_f64() + Vector3::new(0.5, 0.0, 0.5));
                    current_path = prev;
                }
                
                path.push(start);
                path.reverse();
                path.push(goal);
                
                return Some(path);
            }

            if closed_set.contains(&current) {
                continue;
            }
            closed_set.insert(current);

            let current_g = *g_score.get(&current).unwrap_or(&f64::MAX);

            // Explore neighbors (including vertical movement)
            for neighbor_pos in get_neighbors(&current).await {
                if closed_set.contains(&neighbor_pos) {
                    continue;
                }

                // Check if neighbor is walkable
                if !is_walkable(world, &neighbor_pos, &current).await {
                    continue;
                }

                let move_cost = calculate_move_cost(&current, &neighbor_pos);
                let tentative_g = current_g + move_cost;

                if tentative_g < *g_score.get(&neighbor_pos).unwrap_or(&f64::MAX) {
                    came_from.insert(neighbor_pos, current);
                    g_score.insert(neighbor_pos, tentative_g);
                    
                    let h = heuristic(neighbor_pos.0.to_f64(), goal);
                    let f = tentative_g + h;
                    
                    open_set.push(Node {
                        position: neighbor_pos,
                        f_score: f,
                    });
                }
            }
        }

        None // No path found
}

#[cfg(feature = "pathfinding-debug")]
async fn visualize_path(entity: &LivingEntity, goal: &NavigatorGoal) {
    if let Some(debug_info) = &goal.debug_info {
        let world = &entity.entity.world;
        
        // Show waypoints with green particles
        for waypoint in &debug_info.waypoints {
            use pumpkin_data::particle::Particle;
            use pumpkin_protocol::codec::var_int::VarInt;
            use pumpkin_protocol::java::client::play::CParticle;
            
            let particle_packet = CParticle::new(
                true, // force_spawn
                false, // important
                *waypoint,
                Vector3::new(0.1, 0.1, 0.1), // offset
                0.0, // max_speed
                5, // particle_count
                VarInt(Particle::HappyVillager as i32),
                &[],
            );
            
            world.broadcast_packet_all(&particle_packet).await;
        }
        
        // Show goal with blue particle
        let goal_particle = CParticle::new(
            true,
            false,
            debug_info.goal_pos,
            Vector3::new(0.2, 0.2, 0.2),
            0.0,
            10,
            VarInt(Particle::Dust as i32), // Using dust for blue color effect
            &[],
        );
        
        world.broadcast_packet_all(&goal_particle).await;
    }
}

pub struct Node {
    pub position: BlockPos,
    pub f_score: f64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.f_score.partial_cmp(&self.f_score).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl Eq for Node {}

/// Calculate heuristic (Manhattan distance for simplicity, but allowing diagonal)
fn heuristic(a: Vector3<f64>, b: Vector3<f64>) -> f64 {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    let dz = (a.z - b.z).abs();
    
    // Euclidean distance is more accurate for actual movement
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Calculate cost of moving from one position to another
fn calculate_move_cost(from: &BlockPos, to: &BlockPos) -> f64 {
    let dx = (to.0.x - from.0.x).abs();
    let dy = (to.0.y - from.0.y).abs();
    let dz = (to.0.z - from.0.z).abs();
    
    // Base cost
    let mut cost = ((dx * dx + dz * dz) as f64).sqrt();
    
    // Jumping is more expensive
    if dy > 0 {
        cost += dy as f64 * 2.0;
    }
    
    // Falling has some cost but less than jumping
    if dy < 0 {
        cost += dy.abs() as f64 * 0.5;
    }
    
    cost
}

/// Get neighboring positions including vertical movement
async fn get_neighbors(pos: &BlockPos) -> Vec<BlockPos> {
    let mut neighbors = Vec::new();
    
    // Horizontal and vertical movement
    for dx in -1i32..=1 {
        for dz in -1i32..=1 {
            for dy in -1i32..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                
                // Skip pure diagonal on same level and diagonal vertical moves
                if dx.abs() + dz.abs() == 2 && dy != 0 {
                    continue;
                }
                
                let neighbor = BlockPos(Vector3::new(
                    pos.0.x + dx,
                    pos.0.y + dy,
                    pos.0.z + dz,
                ));
                neighbors.push(neighbor);
            }
        }
    }
    
    neighbors
}

/// Check if a position is walkable
async fn is_walkable(
    world: &crate::world::World,
    pos: &BlockPos,
    from: &BlockPos,
) -> bool {
    // Check the block at position (should be air or passable)
    let block_state = world.get_block_state(pos).await;
    let shapes = block_state.get_block_collision_shapes();
    
    // Position should be passable (no collision)
    if !shapes.is_empty() {
        return false;
    }
    
    // Check the block below (should be solid to stand on, unless we're falling)
    let below = BlockPos(Vector3::new(pos.0.x, pos.0.y - 1, pos.0.z));
    let below_state = world.get_block_state(&below).await;
    let below_shapes = below_state.get_block_collision_shapes();
    
    // If moving up, we need solid ground below
    if pos.0.y >= from.0.y {
        if below_shapes.is_empty() {
            // No ground below, can't walk here unless jumping from adjacent block
            if pos.0.y > from.0.y && (pos.0.x - from.0.x).abs() <= 1 && (pos.0.z - from.0.z).abs() <= 1 {
                // Jumping is okay
                return true;
            }
            return false;
        }
    }
    
    // Check head clearance (block above should be passable)
    let above = BlockPos(Vector3::new(pos.0.x, pos.0.y + 1, pos.0.z));
    let above_state = world.get_block_state(&above).await;
    let above_shapes = above_state.get_block_collision_shapes();
    
    if !above_shapes.is_empty() {
        return false;
    }
    
    true
}
