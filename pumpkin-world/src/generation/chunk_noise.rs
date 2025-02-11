use std::{collections::HashMap, hash::Hash};

use pumpkin_macros::block_state;
use pumpkin_util::math::{floor_div, floor_mod, vector2::Vector2};

use crate::{block::BlockState, generation::section_coords};

use super::{
    aquifer_sampler::{
        AquiferSampler, AquiferSamplerImpl, FluidLevelSampler, SeaLevelAquiferSampler,
        WorldAquiferSampler,
    },
    biome_coords,
    generation_shapes::GenerationShape,
    noise_router::{
        chunk_density_function::{
            ChunkNoiseFunctionBuilderOptions, ChunkNoiseFunctionSampleOptions, SampleAction,
            WrapperData,
        },
        chunk_noise_router::ChunkNoiseRouter,
        density_function::{IndexToNoisePos, NoisePos, UnblendedNoisePos},
        proto_noise_router::ProtoChunkNoiseRouter,
    },
    ore_sampler::OreVeinSampler,
    positions::chunk_pos,
    GlobalRandomConfig,
};

pub const STONE_BLOCK: BlockState = block_state!("stone");
pub const LAVA_BLOCK: BlockState = block_state!("lava");
pub const WATER_BLOCK: BlockState = block_state!("water");

pub const CHUNK_DIM: u8 = 16;

#[derive(PartialEq, Eq, Clone, Hash, Default)]
pub struct ChunkNoiseState {}

pub struct ChunkNoiseHeightEstimator {
    surface_height_estimate: HashMap<u64, i32>,
    minimum_height_y: i32,
    maximum_height_y: i32,
    vertical_cell_block_count: usize,
}

impl ChunkNoiseHeightEstimator {
    pub fn estimate_surface_height(
        &mut self,
        router: &mut ChunkNoiseRouter,
        sample_options: &ChunkNoiseFunctionSampleOptions,
        block_x: i32,
        block_z: i32,
    ) -> i32 {
        let biome_aligned_x = biome_coords::to_block(biome_coords::from_block(block_x));
        let biome_aligned_z = biome_coords::to_block(biome_coords::from_block(block_z));
        let packed = chunk_pos::packed(&Vector2::new(biome_aligned_x, biome_aligned_z));

        if let Some(estimate) = self.surface_height_estimate.get(&packed) {
            *estimate
        } else {
            let estimate = self.calculate_height_estimate(router, sample_options, packed);
            self.surface_height_estimate.insert(packed, estimate);
            estimate
        }
    }

    fn calculate_height_estimate(
        &mut self,
        router: &mut ChunkNoiseRouter,
        options: &ChunkNoiseFunctionSampleOptions,
        packed_pos: u64,
    ) -> i32 {
        let x = chunk_pos::unpack_x(packed_pos);
        let z = chunk_pos::unpack_z(packed_pos);

        for y in (self.minimum_height_y..=self.maximum_height_y)
            .rev()
            .step_by(self.vertical_cell_block_count)
        {
            let density_sample = router
                .initial_density_without_jaggedness(&UnblendedNoisePos::new(x, y, z), options);
            if density_sample > 0.390625f64 {
                return y;
            }
        }

        i32::MAX
    }
}

pub enum BlockStateSampler {
    Aquifer(AquiferSampler),
    Ore(OreVeinSampler),
    Chained(ChainedBlockStateSampler),
}

impl BlockStateSampler {
    pub fn sample(
        &mut self,
        router: &mut ChunkNoiseRouter,
        pos: &impl NoisePos,
        sample_options: &ChunkNoiseFunctionSampleOptions,
        height_estimator: &mut ChunkNoiseHeightEstimator,
    ) -> Option<BlockState> {
        match self {
            Self::Aquifer(aquifer) => aquifer.apply(router, pos, sample_options, height_estimator),
            Self::Ore(ore) => ore.sample(router, pos, sample_options),
            Self::Chained(chained) => chained.sample(router, pos, sample_options, height_estimator),
        }
    }
}

pub struct ChainedBlockStateSampler {
    pub(crate) samplers: Box<[BlockStateSampler]>,
}

impl ChainedBlockStateSampler {
    pub fn new(samplers: Box<[BlockStateSampler]>) -> Self {
        Self { samplers }
    }

    fn sample(
        &mut self,
        router: &mut ChunkNoiseRouter,
        pos: &impl NoisePos,
        sample_options: &ChunkNoiseFunctionSampleOptions,
        height_estimator: &mut ChunkNoiseHeightEstimator,
    ) -> Option<BlockState> {
        self.samplers
            .iter_mut()
            .map(|sampler| sampler.sample(router, pos, sample_options, height_estimator))
            .find(|state| state.is_some())
            .unwrap_or(None)
    }
}

struct InterpolationIndexMapper {
    x: i32,
    z: i32,

    minimum_cell_y: i32,
    vertical_cell_block_count: i32,
}

impl IndexToNoisePos for InterpolationIndexMapper {
    fn at(
        &self,
        index: usize,
        sample_data: Option<&mut ChunkNoiseFunctionSampleOptions>,
    ) -> impl NoisePos + 'static {
        if let Some(sample_data) = sample_data {
            sample_data.cache_result_unique_id += 1;
            sample_data.fill_index = index;
        }

        let y = (index as i32 + self.minimum_cell_y) * self.vertical_cell_block_count;

        // TODO: Change this when Blender is implemented
        UnblendedNoisePos::new(self.x, y, self.z)
    }
}

struct ChunkIndexMapper {
    start_x: i32,
    start_y: i32,
    start_z: i32,

    horizontal_cell_block_count: usize,
    vertical_cell_block_count: usize,
}

impl IndexToNoisePos for ChunkIndexMapper {
    fn at(
        &self,
        index: usize,
        sample_options: Option<&mut ChunkNoiseFunctionSampleOptions>,
    ) -> impl NoisePos + 'static {
        let cell_z_position = floor_mod(index, self.horizontal_cell_block_count);
        let xy_portion = floor_div(index, self.horizontal_cell_block_count);
        let cell_x_position = floor_mod(xy_portion, self.horizontal_cell_block_count);
        let cell_y_position = self.vertical_cell_block_count
            - 1
            - floor_div(xy_portion, self.horizontal_cell_block_count);

        if let Some(sample_options) = sample_options {
            sample_options.fill_index = index;
            if let SampleAction::Wrappers(wrapper_data) = &mut sample_options.action {
                wrapper_data.cell_x_block_position = cell_x_position;
                wrapper_data.cell_y_block_position = cell_y_position;
                wrapper_data.cell_z_block_position = cell_z_position;
            }
        }

        // TODO: Change this when Blender is implemented
        UnblendedNoisePos::new(
            self.start_x + cell_x_position as i32,
            self.start_y + cell_y_position as i32,
            self.start_z + cell_z_position as i32,
        )
    }
}

pub struct ChunkNoiseGenerator<'a> {
    pub state_sampler: BlockStateSampler,
    generation_shape: GenerationShape,

    start_cell_pos: Vector2<i32>,

    vertical_cell_count: usize,
    minimum_cell_y: i32,

    cache_fill_unique_id: u64,
    cache_result_unique_id: u64,

    pub router: ChunkNoiseRouter<'a>,
    pub height_estimator: ChunkNoiseHeightEstimator,
}

impl<'a> ChunkNoiseGenerator<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        noise_router_base: &'a ProtoChunkNoiseRouter,
        random_config: &GlobalRandomConfig,
        horizontal_cell_count: u8,
        start_block_x: i32,
        start_block_z: i32,
        generation_shape: GenerationShape,
        level_sampler: FluidLevelSampler,
        aquifers: bool,
        ore_veins: bool,
    ) -> Self {
        let start_cell_pos = Vector2::new(
            floor_div(
                start_block_x,
                generation_shape.horizontal_cell_block_count() as i32,
            ),
            floor_div(
                start_block_z,
                generation_shape.horizontal_cell_block_count() as i32,
            ),
        );

        let biome_pos = Vector2::new(
            biome_coords::from_block(start_block_x),
            biome_coords::from_block(start_block_z),
        );
        let horizontal_biome_end = biome_coords::from_block(
            horizontal_cell_count * generation_shape.horizontal_cell_block_count(),
        );

        let vertical_cell_count = generation_shape.height() as usize
            / generation_shape.vertical_cell_block_count() as usize;
        let minimum_cell_y = floor_div(
            generation_shape.min_y() as i32,
            generation_shape.vertical_cell_block_count() as i32,
        );
        let vertical_cell_block_count = generation_shape.vertical_cell_block_count();
        let horizontal_cell_block_count = generation_shape.horizontal_cell_block_count();

        let builder_options = ChunkNoiseFunctionBuilderOptions::new(
            horizontal_cell_block_count as usize,
            vertical_cell_block_count as usize,
            vertical_cell_count,
            horizontal_cell_count as usize,
            biome_pos.x,
            biome_pos.z,
            horizontal_biome_end as usize,
        );

        let aquifer_sampler = if aquifers {
            let section_x = section_coords::block_to_section(start_block_x);
            let section_z = section_coords::block_to_section(start_block_z);
            AquiferSampler::Aquifier(WorldAquiferSampler::new(
                Vector2::new(section_x, section_z),
                random_config.aquifier_random_deriver.clone(),
                generation_shape.min_y(),
                generation_shape.height(),
                level_sampler,
            ))
        } else {
            AquiferSampler::SeaLevel(SeaLevelAquiferSampler::new(level_sampler))
        };

        let mut samplers = vec![BlockStateSampler::Aquifer(aquifer_sampler)];

        if ore_veins {
            let ore_sampler = OreVeinSampler::new(random_config.ore_random_deriver.clone());
            samplers.push(BlockStateSampler::Ore(ore_sampler));
        };

        let state_sampler =
            BlockStateSampler::Chained(ChainedBlockStateSampler::new(samplers.into_boxed_slice()));

        let height_estimator = ChunkNoiseHeightEstimator {
            surface_height_estimate: HashMap::new(),
            minimum_height_y: generation_shape.min_y() as i32,
            maximum_height_y: generation_shape.min_y() as i32 + generation_shape.height() as i32,
            vertical_cell_block_count: vertical_cell_block_count as usize,
        };

        let router = ChunkNoiseRouter::generate(noise_router_base, &builder_options);

        Self {
            state_sampler,
            generation_shape,

            start_cell_pos,

            vertical_cell_count,
            minimum_cell_y,

            cache_fill_unique_id: 0,
            cache_result_unique_id: 0,

            router,
            height_estimator,
        }
    }

    #[inline]
    pub fn sample_start_density(&mut self) {
        self.cache_result_unique_id = 0;
        self.sample_density(true, self.start_cell_pos.x);
    }

    #[inline]
    pub fn sample_end_density(&mut self, cell_x: u8) {
        self.sample_density(false, self.start_cell_pos.x + cell_x as i32 + 1);
    }

    fn sample_density(&mut self, start: bool, current_x: i32) {
        let x = current_x * self.horizontal_cell_block_count() as i32;

        for cell_z in 0..=self.horizontal_cell_block_count() {
            let current_cell_z_pos = self.start_cell_pos.z + cell_z as i32;
            let z = current_cell_z_pos * self.horizontal_cell_block_count() as i32;
            self.cache_fill_unique_id += 1;

            let mapper = InterpolationIndexMapper {
                x,
                z,
                minimum_cell_y: self.minimum_cell_y,
                vertical_cell_block_count: self.vertical_cell_block_count() as i32,
            };

            let mut options = ChunkNoiseFunctionSampleOptions::new(
                false,
                SampleAction::Wrappers(WrapperData {
                    cell_x_block_position: 0,
                    cell_y_block_position: 0,
                    cell_z_block_position: 0,
                    horizontal_cell_block_count: self.horizontal_cell_block_count() as usize,
                    vertical_cell_block_count: self.vertical_cell_block_count() as usize,
                }),
                self.cache_result_unique_id,
                self.cache_fill_unique_id,
                0,
            );

            self.fill_interpolator_buffers(start, cell_z as usize, &mapper, &mut options);
            self.cache_result_unique_id = options.cache_result_unique_id;
        }
        self.cache_fill_unique_id += 1;
    }

    #[inline]
    fn fill_interpolator_buffers(
        &mut self,
        start: bool,
        cell_z: usize,
        mapper: &impl IndexToNoisePos,
        sample_options: &mut ChunkNoiseFunctionSampleOptions,
    ) {
        self.router
            .fill_interpolator_buffers(start, cell_z, mapper, sample_options);
    }

    #[inline]
    pub fn interpolate_x(&mut self, delta: f64) {
        self.router.interpolate_x(delta);
    }

    #[inline]
    pub fn interpolate_y(&mut self, delta: f64) {
        self.router.interpolate_y(delta);
    }

    #[inline]
    pub fn interpolate_z(&mut self, delta: f64) {
        self.cache_result_unique_id += 1;
        self.router.interpolate_z(delta);
    }

    #[inline]
    pub fn swap_buffers(&mut self) {
        self.router.swap_buffers();
    }

    pub fn on_sampled_cell_corners(&mut self, cell_x: u8, cell_y: u16, cell_z: u8) {
        self.router
            .on_sampled_cell_corners(cell_y as usize, cell_z as usize);
        self.cache_fill_unique_id += 1;

        let start_x =
            (self.start_cell_pos.x + cell_x as i32) * self.horizontal_cell_block_count() as i32;
        let start_y =
            (cell_y as i32 + self.minimum_cell_y) * self.vertical_cell_block_count() as i32;
        let start_z =
            (self.start_cell_pos.z + cell_z as i32) * self.horizontal_cell_block_count() as i32;

        let mapper = ChunkIndexMapper {
            start_x,
            start_y,
            start_z,
            horizontal_cell_block_count: self.horizontal_cell_block_count() as usize,
            vertical_cell_block_count: self.vertical_cell_block_count() as usize,
        };

        let mut sample_options = ChunkNoiseFunctionSampleOptions::new(
            true,
            SampleAction::Wrappers(WrapperData {
                cell_x_block_position: 0,
                cell_y_block_position: 0,
                cell_z_block_position: 0,
                horizontal_cell_block_count: self.horizontal_cell_block_count() as usize,
                vertical_cell_block_count: self.vertical_cell_block_count() as usize,
            }),
            self.cache_result_unique_id,
            self.cache_fill_unique_id,
            0,
        );

        self.router.fill_cell_caches(&mapper, &mut sample_options);
        self.cache_fill_unique_id += 1;
    }

    pub fn sample_block_state(
        &mut self,
        start_x: i32,
        start_y: i32,
        start_z: i32,
        cell_x: usize,
        cell_y: usize,
        cell_z: usize,
    ) -> Option<BlockState> {
        //TODO: Fix this when Blender is added
        let pos = UnblendedNoisePos::new(
            start_x + cell_x as i32,
            start_y + cell_y as i32,
            start_z + cell_z as i32,
        );
        let options = ChunkNoiseFunctionSampleOptions::new(
            false,
            SampleAction::Wrappers(WrapperData {
                cell_x_block_position: cell_x,
                cell_y_block_position: cell_y,
                cell_z_block_position: cell_z,
                horizontal_cell_block_count: self.horizontal_cell_block_count() as usize,
                vertical_cell_block_count: self.vertical_cell_block_count() as usize,
            }),
            self.cache_result_unique_id,
            self.cache_fill_unique_id,
            0,
        );

        self.state_sampler
            .sample(&mut self.router, &pos, &options, &mut self.height_estimator)
    }

    pub fn horizontal_cell_block_count(&self) -> u8 {
        self.generation_shape.horizontal_cell_block_count()
    }

    pub fn vertical_cell_block_count(&self) -> u8 {
        self.generation_shape.vertical_cell_block_count()
    }

    pub fn min_y(&self) -> i8 {
        self.generation_shape.min_y()
    }

    pub fn minimum_cell_y(&self) -> i8 {
        self.generation_shape.min_y() / self.generation_shape.vertical_cell_block_count() as i8
    }

    pub fn height(&self) -> u16 {
        self.generation_shape.height()
    }
}

/*
#[cfg(test)]
mod test {
    use pumpkin_util::math::vector2::Vector2;

    use crate::generation::{
        aquifer_sampler::{FluidLevel, FluidLevelSampler},
        generation_shapes::GenerationShape,
        noise::{config::NoiseConfig, router::OVERWORLD_NOISE_ROUTER},
        positions::chunk_pos,
        proto_chunk::StandardChunkFluidLevelSampler,
    };

    use super::{ChunkNoiseGenerator, LAVA_BLOCK, WATER_BLOCK};

    #[test]
    fn test_estimate_height() {
        let shape = GenerationShape::SURFACE;
        let chunk_pos = Vector2::new(7, 4);
        let config = NoiseConfig::new(0, &OVERWORLD_NOISE_ROUTER);
        let sampler = FluidLevelSampler::Chunk(StandardChunkFluidLevelSampler::new(
            FluidLevel::new(63, WATER_BLOCK),
            FluidLevel::new(-54, LAVA_BLOCK),
        ));
        let mut noise = ChunkNoiseGenerator::new(
            16 / shape.horizontal_cell_block_count(),
            chunk_pos::start_block_x(&chunk_pos),
            chunk_pos::start_block_z(&chunk_pos),
            shape,
            &config,
            sampler,
            true,
            true,
        );

        let values = [
            ((-10, -10), 48),
            ((-10, -9), 48),
            ((-10, -8), 48),
            ((-10, -7), 48),
            ((-10, -6), 48),
            ((-10, -5), 48),
            ((-10, -4), 48),
            ((-10, -3), 48),
            ((-10, -2), 48),
            ((-10, -1), 48),
            ((-10, 0), 56),
            ((-10, 1), 56),
            ((-10, 2), 56),
            ((-10, 3), 56),
            ((-10, 4), 56),
            ((-10, 5), 56),
            ((-10, 6), 56),
            ((-10, 7), 56),
            ((-10, 8), 56),
            ((-10, 9), 56),
            ((-10, 10), 56),
            ((-9, -10), 48),
            ((-9, -9), 48),
            ((-9, -8), 48),
            ((-9, -7), 48),
            ((-9, -6), 48),
            ((-9, -5), 48),
            ((-9, -4), 48),
            ((-9, -3), 48),
            ((-9, -2), 48),
            ((-9, -1), 48),
            ((-9, 0), 56),
            ((-9, 1), 56),
            ((-9, 2), 56),
            ((-9, 3), 56),
            ((-9, 4), 56),
            ((-9, 5), 56),
            ((-9, 6), 56),
            ((-9, 7), 56),
            ((-9, 8), 56),
            ((-9, 9), 56),
            ((-9, 10), 56),
            ((-8, -10), 40),
            ((-8, -9), 40),
            ((-8, -8), 48),
            ((-8, -7), 48),
            ((-8, -6), 48),
            ((-8, -5), 48),
            ((-8, -4), 48),
            ((-8, -3), 48),
            ((-8, -2), 48),
            ((-8, -1), 48),
            ((-8, 0), 56),
            ((-8, 1), 56),
            ((-8, 2), 56),
            ((-8, 3), 56),
            ((-8, 4), 56),
            ((-8, 5), 56),
            ((-8, 6), 56),
            ((-8, 7), 56),
            ((-8, 8), 56),
            ((-8, 9), 56),
            ((-8, 10), 56),
            ((-7, -10), 40),
            ((-7, -9), 40),
            ((-7, -8), 48),
            ((-7, -7), 48),
            ((-7, -6), 48),
            ((-7, -5), 48),
            ((-7, -4), 48),
            ((-7, -3), 48),
            ((-7, -2), 48),
            ((-7, -1), 48),
            ((-7, 0), 56),
            ((-7, 1), 56),
            ((-7, 2), 56),
            ((-7, 3), 56),
            ((-7, 4), 56),
            ((-7, 5), 56),
            ((-7, 6), 56),
            ((-7, 7), 56),
            ((-7, 8), 56),
            ((-7, 9), 56),
            ((-7, 10), 56),
            ((-6, -10), 40),
            ((-6, -9), 40),
            ((-6, -8), 48),
            ((-6, -7), 48),
            ((-6, -6), 48),
            ((-6, -5), 48),
            ((-6, -4), 48),
            ((-6, -3), 48),
            ((-6, -2), 48),
            ((-6, -1), 48),
            ((-6, 0), 56),
            ((-6, 1), 56),
            ((-6, 2), 56),
            ((-6, 3), 56),
            ((-6, 4), 56),
            ((-6, 5), 56),
            ((-6, 6), 56),
            ((-6, 7), 56),
            ((-6, 8), 56),
            ((-6, 9), 56),
            ((-6, 10), 56),
            ((-5, -10), 40),
            ((-5, -9), 40),
            ((-5, -8), 48),
            ((-5, -7), 48),
            ((-5, -6), 48),
            ((-5, -5), 48),
            ((-5, -4), 48),
            ((-5, -3), 48),
            ((-5, -2), 48),
            ((-5, -1), 48),
            ((-5, 0), 56),
            ((-5, 1), 56),
            ((-5, 2), 56),
            ((-5, 3), 56),
            ((-5, 4), 56),
            ((-5, 5), 56),
            ((-5, 6), 56),
            ((-5, 7), 56),
            ((-5, 8), 56),
            ((-5, 9), 56),
            ((-5, 10), 56),
            ((-4, -10), 40),
            ((-4, -9), 40),
            ((-4, -8), 40),
            ((-4, -7), 40),
            ((-4, -6), 40),
            ((-4, -5), 40),
            ((-4, -4), 48),
            ((-4, -3), 48),
            ((-4, -2), 48),
            ((-4, -1), 48),
            ((-4, 0), 48),
            ((-4, 1), 48),
            ((-4, 2), 48),
            ((-4, 3), 48),
            ((-4, 4), 48),
            ((-4, 5), 48),
            ((-4, 6), 48),
            ((-4, 7), 48),
            ((-4, 8), 48),
            ((-4, 9), 48),
            ((-4, 10), 48),
            ((-3, -10), 40),
            ((-3, -9), 40),
            ((-3, -8), 40),
            ((-3, -7), 40),
            ((-3, -6), 40),
            ((-3, -5), 40),
            ((-3, -4), 48),
            ((-3, -3), 48),
            ((-3, -2), 48),
            ((-3, -1), 48),
            ((-3, 0), 48),
            ((-3, 1), 48),
            ((-3, 2), 48),
            ((-3, 3), 48),
            ((-3, 4), 48),
            ((-3, 5), 48),
            ((-3, 6), 48),
            ((-3, 7), 48),
            ((-3, 8), 48),
            ((-3, 9), 48),
            ((-3, 10), 48),
            ((-2, -10), 40),
            ((-2, -9), 40),
            ((-2, -8), 40),
            ((-2, -7), 40),
            ((-2, -6), 40),
            ((-2, -5), 40),
            ((-2, -4), 48),
            ((-2, -3), 48),
            ((-2, -2), 48),
            ((-2, -1), 48),
            ((-2, 0), 48),
            ((-2, 1), 48),
            ((-2, 2), 48),
            ((-2, 3), 48),
            ((-2, 4), 48),
            ((-2, 5), 48),
            ((-2, 6), 48),
            ((-2, 7), 48),
            ((-2, 8), 48),
            ((-2, 9), 48),
            ((-2, 10), 48),
            ((-1, -10), 40),
            ((-1, -9), 40),
            ((-1, -8), 40),
            ((-1, -7), 40),
            ((-1, -6), 40),
            ((-1, -5), 40),
            ((-1, -4), 48),
            ((-1, -3), 48),
            ((-1, -2), 48),
            ((-1, -1), 48),
            ((-1, 0), 48),
            ((-1, 1), 48),
            ((-1, 2), 48),
            ((-1, 3), 48),
            ((-1, 4), 48),
            ((-1, 5), 48),
            ((-1, 6), 48),
            ((-1, 7), 48),
            ((-1, 8), 48),
            ((-1, 9), 48),
            ((-1, 10), 48),
            ((0, -10), 48),
            ((0, -9), 48),
            ((0, -8), 40),
            ((0, -7), 40),
            ((0, -6), 40),
            ((0, -5), 40),
            ((0, -4), 40),
            ((0, -3), 40),
            ((0, -2), 40),
            ((0, -1), 40),
            ((0, 0), 40),
            ((0, 1), 40),
            ((0, 2), 40),
            ((0, 3), 40),
            ((0, 4), 48),
            ((0, 5), 48),
            ((0, 6), 48),
            ((0, 7), 48),
            ((0, 8), 48),
            ((0, 9), 48),
            ((0, 10), 48),
            ((1, -10), 48),
            ((1, -9), 48),
            ((1, -8), 40),
            ((1, -7), 40),
            ((1, -6), 40),
            ((1, -5), 40),
            ((1, -4), 40),
            ((1, -3), 40),
            ((1, -2), 40),
            ((1, -1), 40),
            ((1, 0), 40),
            ((1, 1), 40),
            ((1, 2), 40),
            ((1, 3), 40),
            ((1, 4), 48),
            ((1, 5), 48),
            ((1, 6), 48),
            ((1, 7), 48),
            ((1, 8), 48),
            ((1, 9), 48),
            ((1, 10), 48),
            ((2, -10), 48),
            ((2, -9), 48),
            ((2, -8), 40),
            ((2, -7), 40),
            ((2, -6), 40),
            ((2, -5), 40),
            ((2, -4), 40),
            ((2, -3), 40),
            ((2, -2), 40),
            ((2, -1), 40),
            ((2, 0), 40),
            ((2, 1), 40),
            ((2, 2), 40),
            ((2, 3), 40),
            ((2, 4), 48),
            ((2, 5), 48),
            ((2, 6), 48),
            ((2, 7), 48),
            ((2, 8), 48),
            ((2, 9), 48),
            ((2, 10), 48),
            ((3, -10), 48),
            ((3, -9), 48),
            ((3, -8), 40),
            ((3, -7), 40),
            ((3, -6), 40),
            ((3, -5), 40),
            ((3, -4), 40),
            ((3, -3), 40),
            ((3, -2), 40),
            ((3, -1), 40),
            ((3, 0), 40),
            ((3, 1), 40),
            ((3, 2), 40),
            ((3, 3), 40),
            ((3, 4), 48),
            ((3, 5), 48),
            ((3, 6), 48),
            ((3, 7), 48),
            ((3, 8), 48),
            ((3, 9), 48),
            ((3, 10), 48),
            ((4, -10), 48),
            ((4, -9), 48),
            ((4, -8), 48),
            ((4, -7), 48),
            ((4, -6), 48),
            ((4, -5), 48),
            ((4, -4), 40),
            ((4, -3), 40),
            ((4, -2), 40),
            ((4, -1), 40),
            ((4, 0), 40),
            ((4, 1), 40),
            ((4, 2), 40),
            ((4, 3), 40),
            ((4, 4), 48),
            ((4, 5), 48),
            ((4, 6), 48),
            ((4, 7), 48),
            ((4, 8), 48),
            ((4, 9), 48),
            ((4, 10), 48),
            ((5, -10), 48),
            ((5, -9), 48),
            ((5, -8), 48),
            ((5, -7), 48),
            ((5, -6), 48),
            ((5, -5), 48),
            ((5, -4), 40),
            ((5, -3), 40),
            ((5, -2), 40),
            ((5, -1), 40),
            ((5, 0), 40),
            ((5, 1), 40),
            ((5, 2), 40),
            ((5, 3), 40),
            ((5, 4), 48),
            ((5, 5), 48),
            ((5, 6), 48),
            ((5, 7), 48),
            ((5, 8), 48),
            ((5, 9), 48),
            ((5, 10), 48),
            ((6, -10), 48),
            ((6, -9), 48),
            ((6, -8), 48),
            ((6, -7), 48),
            ((6, -6), 48),
            ((6, -5), 48),
            ((6, -4), 40),
            ((6, -3), 40),
            ((6, -2), 40),
            ((6, -1), 40),
            ((6, 0), 40),
            ((6, 1), 40),
            ((6, 2), 40),
            ((6, 3), 40),
            ((6, 4), 48),
            ((6, 5), 48),
            ((6, 6), 48),
            ((6, 7), 48),
            ((6, 8), 48),
            ((6, 9), 48),
            ((6, 10), 48),
            ((7, -10), 48),
            ((7, -9), 48),
            ((7, -8), 48),
            ((7, -7), 48),
            ((7, -6), 48),
            ((7, -5), 48),
            ((7, -4), 40),
            ((7, -3), 40),
            ((7, -2), 40),
            ((7, -1), 40),
            ((7, 0), 40),
            ((7, 1), 40),
            ((7, 2), 40),
            ((7, 3), 40),
            ((7, 4), 48),
            ((7, 5), 48),
            ((7, 6), 48),
            ((7, 7), 48),
            ((7, 8), 48),
            ((7, 9), 48),
            ((7, 10), 48),
            ((8, -10), 48),
            ((8, -9), 48),
            ((8, -8), 48),
            ((8, -7), 48),
            ((8, -6), 48),
            ((8, -5), 48),
            ((8, -4), 40),
            ((8, -3), 40),
            ((8, -2), 40),
            ((8, -1), 40),
            ((8, 0), 40),
            ((8, 1), 40),
            ((8, 2), 40),
            ((8, 3), 40),
            ((8, 4), 48),
            ((8, 5), 48),
            ((8, 6), 48),
            ((8, 7), 48),
            ((8, 8), 48),
            ((8, 9), 48),
            ((8, 10), 48),
            ((9, -10), 48),
            ((9, -9), 48),
            ((9, -8), 48),
            ((9, -7), 48),
            ((9, -6), 48),
            ((9, -5), 48),
            ((9, -4), 40),
            ((9, -3), 40),
            ((9, -2), 40),
            ((9, -1), 40),
            ((9, 0), 40),
            ((9, 1), 40),
            ((9, 2), 40),
            ((9, 3), 40),
            ((9, 4), 48),
            ((9, 5), 48),
            ((9, 6), 48),
            ((9, 7), 48),
            ((9, 8), 48),
            ((9, 9), 48),
            ((9, 10), 48),
            ((10, -10), 48),
            ((10, -9), 48),
            ((10, -8), 48),
            ((10, -7), 48),
            ((10, -6), 48),
            ((10, -5), 48),
            ((10, -4), 40),
            ((10, -3), 40),
            ((10, -2), 40),
            ((10, -1), 40),
            ((10, 0), 40),
            ((10, 1), 40),
            ((10, 2), 40),
            ((10, 3), 40),
            ((10, 4), 48),
            ((10, 5), 48),
            ((10, 6), 48),
            ((10, 7), 48),
            ((10, 8), 48),
            ((10, 9), 48),
            ((10, 10), 48),
        ];

        for ((x, z), result) in values {
            let functions = &mut noise.height_estimator;
            let state = &noise.shared;
            assert_eq!(functions.estimate_surface_height(state, x, z), result);
        }
    }

    // TODO: Add test to verify the height estimator has no interpolators or cell caches
}
*/
