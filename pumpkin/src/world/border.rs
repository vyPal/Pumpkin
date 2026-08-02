use pumpkin_protocol::java::client::play::{
    CInitializeWorldBorder, CSetBorderCenter, CSetBorderLerpSize, CSetBorderSize,
    CSetBorderWarningDelay, CSetBorderWarningDistance,
};

use crate::net::java::JavaClient;

use super::World;

/// Real-time milliseconds per server tick, used to translate the plugin API's tick-based
/// lerp `speed` (see `pumpkin-plugin-wit`'s `world-border.set-diameter`) into the
/// millisecond duration the client protocol actually expects.
const MS_PER_TICK: u64 = 50;

pub struct Worldborder {
    pub center_x: f64,
    pub center_z: f64,
    pub old_diameter: f64,
    pub new_diameter: f64,
    pub speed: i64,
    pub portal_teleport_boundary: i32,
    pub warning_blocks: i32,
    pub warning_time: i32,
    pub damage_per_block: f32,
    pub buffer: f32,
    /// The border's actual diameter right now — mid-lerp if a resize is in progress.
    /// `contains`/`contains_block`/`clamp_block` check against this, not `new_diameter`:
    /// otherwise a shrinking border is already treated as fully shrunk to its target size
    /// the instant `set_diameter` is called, even though clients only see it animate there
    /// over `speed` ticks. Kept in sync by `tick`.
    current_diameter: f64,
    /// Ticks remaining in the active lerp. 0 means no lerp is in progress and
    /// `current_diameter` already equals `new_diameter`.
    lerp_ticks_remaining: u64,
    /// Total ticks the active lerp spans, so `tick` can compute how far through it we are.
    lerp_ticks_total: u64,
}

impl Worldborder {
    #[must_use]
    pub const fn new(
        x: f64,
        z: f64,
        diameter: f64,
        speed: i64,
        warning_blocks: i32,
        warning_time: i32,
    ) -> Self {
        Self {
            center_x: x,
            center_z: z,
            old_diameter: diameter,
            new_diameter: diameter,
            speed,
            portal_teleport_boundary: 29_999_984,
            warning_blocks,
            warning_time,
            damage_per_block: 0.0,
            buffer: 0.0,
            current_diameter: diameter,
            lerp_ticks_remaining: 0,
            lerp_ticks_total: 0,
        }
    }

    pub async fn init_client(&self, client: &JavaClient) {
        // Mirrors what an already-connected client sees: animating from the live diameter
        // (not the stale lerp-start point in `old_diameter`) toward `new_diameter` over
        // whatever's left of the lerp.
        let remaining_ms = self.lerp_ticks_remaining * MS_PER_TICK;
        client
            .enqueue_packet(&CInitializeWorldBorder::new(
                self.center_x,
                self.center_z,
                self.current_diameter,
                self.new_diameter,
                (remaining_ms as i64).into(),
                self.portal_teleport_boundary.into(),
                self.warning_blocks.into(),
                self.warning_time.into(),
            ))
            .await;
    }

    pub fn set_center(&mut self, world: &World, x: f64, z: f64) {
        self.center_x = x;
        self.center_z = z;

        world.broadcast_packet_all(&CSetBorderCenter::new(self.center_x, self.center_z));
    }

    /// `speed` is in server ticks (the plugin API's documented unit), converted here to the
    /// milliseconds the wire protocol expects.
    pub fn set_diameter(&mut self, world: &World, diameter: f64, speed: Option<i64>) {
        self.old_diameter = self.current_diameter;
        self.new_diameter = diameter;

        if let Some(ticks) = speed {
            let ticks = ticks.max(0) as u64;
            self.speed = ticks as i64;
            self.lerp_ticks_total = ticks;
            self.lerp_ticks_remaining = ticks;
            if ticks == 0 {
                self.current_diameter = self.new_diameter;
            }
            let speed_ms = ticks.saturating_mul(MS_PER_TICK);
            world.broadcast_packet_all(&CSetBorderLerpSize::new(
                self.old_diameter,
                self.new_diameter,
                (speed_ms as i64).into(),
            ));
        } else {
            self.speed = 0;
            self.lerp_ticks_total = 0;
            self.lerp_ticks_remaining = 0;
            self.current_diameter = self.new_diameter;
            world.broadcast_packet_all(&CSetBorderSize::new(self.new_diameter));
        }
    }

    pub fn add_diameter(&mut self, world: &World, offset: f64, speed: Option<i64>) {
        self.set_diameter(world, self.new_diameter + offset, speed);
    }

    pub fn set_warning_delay(&mut self, world: &World, delay: i32) {
        self.warning_time = delay;

        world.broadcast_packet_all(&CSetBorderWarningDelay::new(self.warning_time.into()));
    }

    pub fn set_warning_distance(&mut self, world: &World, distance: i32) {
        self.warning_blocks = distance;

        world.broadcast_packet_all(&CSetBorderWarningDistance::new(self.warning_blocks.into()));
    }

    /// Advances an in-progress lerp by one tick, recomputing `current_diameter`. Called once
    /// per world tick from `World::tick_environment`. A no-op unless `set_diameter` started a
    /// lerp that hasn't finished yet.
    pub fn tick(&mut self) {
        if self.lerp_ticks_remaining == 0 {
            return;
        }
        self.lerp_ticks_remaining -= 1;
        if self.lerp_ticks_remaining == 0 {
            self.current_diameter = self.new_diameter;
        } else {
            let elapsed = self.lerp_ticks_total - self.lerp_ticks_remaining;
            #[expect(clippy::cast_precision_loss)]
            let t = elapsed as f64 / self.lerp_ticks_total as f64;
            self.current_diameter = self.old_diameter + (self.new_diameter - self.old_diameter) * t;
        }
    }

    /// The border's actual diameter right now — mid-lerp if a resize is in progress. This is
    /// what `contains`/`clamp_block` check against, and what the plugin API's `get-diameter`
    /// reports.
    #[must_use]
    pub const fn current_diameter(&self) -> f64 {
        self.current_diameter
    }

    #[must_use]
    pub fn contains(&self, x: f64, z: f64) -> bool {
        let half = self.current_diameter / 2.0;
        let min_x = self.center_x - half;
        let max_x = self.center_x + half;
        let min_z = self.center_z - half;
        let max_z = self.center_z + half;
        x >= min_x && x < max_x && z >= min_z && z < max_z
    }

    #[must_use]
    pub fn contains_block(&self, x: i32, z: i32) -> bool {
        self.contains(f64::from(x), f64::from(z))
            && self.contains(f64::from(x + 1), f64::from(z + 1))
    }

    #[must_use]
    pub fn clamp_block(&self, x: i32, z: i32) -> (i32, i32) {
        let half = self.current_diameter / 2.0;
        let min_x = (self.center_x - half).floor() as i32;
        let max_x = (self.center_x + half).floor() as i32 - 1;
        let min_z = (self.center_z - half).floor() as i32;
        let max_z = (self.center_z + half).floor() as i32 - 1;
        (x.clamp(min_x, max_x), z.clamp(min_z, max_z))
    }
}
