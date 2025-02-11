use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};

use enum_dispatch::enum_dispatch;
use pumpkin_data::chunk::DoublePerlinNoiseParameters;

use crate::{
    generation::noise::perlin::DoublePerlinNoiseSampler,
    noise_router::{
        density_function_ast::{DensityFunctionRepr, SplineRepr},
        noise_router_ast::NoiseRouterRepr,
    },
    GlobalRandomConfig,
};

use super::{
    chunk_density_function::ChunkNoiseFunctionSampleOptions,
    chunk_noise_router::{ChunkNoiseFunctionComponent, StaticChunkNoiseFunctionComponentImpl},
    density_function::{
        math::{Binary, Clamp, Constant, Linear, Unary},
        misc::{ClampedYGradient, EndIsland, RangeChoice, WeirdScaled},
        noise::{InterpolatedNoiseSampler, Noise, ShiftA, ShiftB, ShiftedNoise},
        spline::{Spline, SplineFunction, SplinePoint, SplineValue},
        IndexToNoisePos, NoiseFunctionComponentRange, NoisePos, PassThrough,
        StaticIndependentChunkNoiseFunctionComponentImpl, Wrapper,
    },
};

#[enum_dispatch(
    StaticIndependentChunkNoiseFunctionComponentImpl,
    NoiseFunctionComponentRange
)]
#[derive(Clone)]
pub enum IndependentProtoNoiseFunctionComponent {
    Constant(Constant),
    EndIsland(EndIsland),
    Noise(Noise),
    ShiftA(ShiftA),
    ShiftB(ShiftB),
    InterpolatedNoise(InterpolatedNoiseSampler),
    ClampedYGradient(ClampedYGradient),
}

#[enum_dispatch(StaticChunkNoiseFunctionComponentImpl, NoiseFunctionComponentRange)]
#[derive(Clone)]
pub enum DependentProtoNoiseFunctionComponent {
    Linear(Linear),
    Unary(Unary),
    Binary(Binary),
    ShiftedNoise(ShiftedNoise),
    WeirdScaled(WeirdScaled),
    Clamp(Clamp),
    RangeChoice(RangeChoice),
    Spline(SplineFunction),
}

#[enum_dispatch(NoiseFunctionComponentRange)]
#[derive(Clone)]
pub enum ProtoNoiseFunctionComponent {
    Independent(IndependentProtoNoiseFunctionComponent),
    Dependent(DependentProtoNoiseFunctionComponent),
    Wrapper(Wrapper),
    PassThrough(PassThrough),
}

pub(crate) struct DoublePerlinNoiseBuilder<'a, 'b> {
    random_config: &'b GlobalRandomConfig,
    id_to_sampler_map: Vec<(&'a str, Arc<DoublePerlinNoiseSampler>)>,
}

impl<'a, 'b> DoublePerlinNoiseBuilder<'a, 'b> {
    pub fn new(rand: &'b GlobalRandomConfig) -> Self {
        Self {
            random_config: rand,
            id_to_sampler_map: Vec::new(),
        }
    }

    fn get_noise_sampler_for_id(&mut self, id: &'a str) -> Arc<DoublePerlinNoiseSampler> {
        self.id_to_sampler_map
            .iter()
            .find_map(|ele| {
                if ele.0.eq(id) {
                    Some(ele.1.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                let parameters = DoublePerlinNoiseParameters::id_to_parameters(id)
                    .unwrap_or_else(|| panic!("Unknown noise id: {}", id));

                // Note that the parameters' id is differenent than `id`
                let mut random = self
                    .random_config
                    .base_random_deriver
                    .split_string(parameters.id());
                let sampler = DoublePerlinNoiseSampler::new(&mut random, parameters, false);
                let wrapped = Arc::new(sampler);
                self.id_to_sampler_map.push((id, wrapped.clone()));
                wrapped
            })
    }
}

// Invariant: all index references point to components that have a lower index than the
// component referencing it

/// Returns the index of component the AST represents on the stack
pub(crate) fn recursive_build_proto_stack<'a>(
    ast: &'a DensityFunctionRepr,
    random_config: &GlobalRandomConfig,
    stack: &mut Vec<ProtoNoiseFunctionComponent>,
    map: &mut HashMap<u64, usize>,
    perlin_noise_builder: &mut DoublePerlinNoiseBuilder<'a, '_>,
) -> usize {
    let mut hasher = DefaultHasher::new();
    ast.hash(&mut hasher);
    let ast_hash = hasher.finish();

    map.get(&ast_hash).copied().unwrap_or_else(|| {
        let component = match ast {
            DensityFunctionRepr::Spline { spline, data } => {
                let spline = match recursive_build_spline(
                    spline,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                ) {
                    SplineValue::Spline(spline) => spline,
                    _ => unreachable!(),
                };

                ProtoNoiseFunctionComponent::Dependent(
                    DependentProtoNoiseFunctionComponent::Spline(SplineFunction::new(spline, data)),
                )
            }
            DensityFunctionRepr::EndIslands => ProtoNoiseFunctionComponent::Independent(
                IndependentProtoNoiseFunctionComponent::EndIsland(EndIsland::new(
                    random_config.seed,
                )),
            ),
            DensityFunctionRepr::Noise { data } => {
                let sampler = perlin_noise_builder.get_noise_sampler_for_id(&data.noise_id);
                ProtoNoiseFunctionComponent::Independent(
                    IndependentProtoNoiseFunctionComponent::Noise(Noise::new(sampler, data)),
                )
            }
            DensityFunctionRepr::ShiftA { noise_id } => {
                let sampler = perlin_noise_builder.get_noise_sampler_for_id(noise_id);
                ProtoNoiseFunctionComponent::Independent(
                    IndependentProtoNoiseFunctionComponent::ShiftA(ShiftA::new(sampler)),
                )
            }
            DensityFunctionRepr::ShiftB { noise_id } => {
                let sampler = perlin_noise_builder.get_noise_sampler_for_id(noise_id);
                ProtoNoiseFunctionComponent::Independent(
                    IndependentProtoNoiseFunctionComponent::ShiftB(ShiftB::new(sampler)),
                )
            }
            DensityFunctionRepr::BlendDensity { input } => {
                // TODO: Replace this when the blender is implemented
                return recursive_build_proto_stack(
                    input,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );
            }
            DensityFunctionRepr::BlendAlpha => {
                // TODO: Replace this with the cache when the blender is implemented
                ProtoNoiseFunctionComponent::Independent(
                    IndependentProtoNoiseFunctionComponent::Constant(Constant::new(1.0)),
                )
            }
            DensityFunctionRepr::BlendOffset => {
                // TODO: Replace this with the cache when the blender is implemented
                ProtoNoiseFunctionComponent::Independent(
                    IndependentProtoNoiseFunctionComponent::Constant(Constant::new(0.0)),
                )
            }
            DensityFunctionRepr::Beardifier => {
                // TODO: Replace this when world structures are implemented
                ProtoNoiseFunctionComponent::Independent(
                    IndependentProtoNoiseFunctionComponent::Constant(Constant::new(0.0)),
                )
            }
            DensityFunctionRepr::ShiftedNoise {
                shift_x,
                shift_y,
                shift_z,
                data,
            } => {
                let input_x_index = recursive_build_proto_stack(
                    shift_x,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                let input_y_index = recursive_build_proto_stack(
                    shift_y,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                let input_z_index = recursive_build_proto_stack(
                    shift_z,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                let sampler = perlin_noise_builder.get_noise_sampler_for_id(&data.noise_id);
                ProtoNoiseFunctionComponent::Dependent(
                    DependentProtoNoiseFunctionComponent::ShiftedNoise(ShiftedNoise::new(
                        input_x_index,
                        input_y_index,
                        input_z_index,
                        sampler,
                        data,
                    )),
                )
            }
            DensityFunctionRepr::RangeChoice {
                input,
                when_in_range,
                when_out_range,
                data,
            } => {
                let input_index = recursive_build_proto_stack(
                    input,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                let in_range_index = recursive_build_proto_stack(
                    when_in_range,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                let out_range_index = recursive_build_proto_stack(
                    when_out_range,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                let min_value = stack[in_range_index]
                    .min()
                    .min(stack[out_range_index].min());
                let max_value = stack[in_range_index]
                    .max()
                    .max(stack[out_range_index].max());

                ProtoNoiseFunctionComponent::Dependent(
                    DependentProtoNoiseFunctionComponent::RangeChoice(RangeChoice::new(
                        input_index,
                        in_range_index,
                        out_range_index,
                        min_value,
                        max_value,
                        data,
                    )),
                )
            }
            DensityFunctionRepr::Binary {
                argument1,
                argument2,
                data,
            } => {
                let input1_index = recursive_build_proto_stack(
                    argument1,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                let input2_index = recursive_build_proto_stack(
                    argument2,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                ProtoNoiseFunctionComponent::Dependent(
                    DependentProtoNoiseFunctionComponent::Binary(Binary::new(
                        input1_index,
                        input2_index,
                        data,
                    )),
                )
            }
            DensityFunctionRepr::ClampedYGradient { data } => {
                ProtoNoiseFunctionComponent::Independent(
                    IndependentProtoNoiseFunctionComponent::ClampedYGradient(
                        ClampedYGradient::new(data),
                    ),
                )
            }
            DensityFunctionRepr::Constant { value } => ProtoNoiseFunctionComponent::Independent(
                IndependentProtoNoiseFunctionComponent::Constant(Constant::new(value.0)),
            ),
            DensityFunctionRepr::Wrapper { input, wrapper } => {
                let input_index = recursive_build_proto_stack(
                    input,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );
                let min_value = stack[input_index].min();
                let max_value = stack[input_index].max();

                ProtoNoiseFunctionComponent::Wrapper(Wrapper::new(
                    input_index,
                    *wrapper,
                    min_value,
                    max_value,
                ))
            }
            DensityFunctionRepr::Linear { input, data } => {
                let input_index = recursive_build_proto_stack(
                    input,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                ProtoNoiseFunctionComponent::Dependent(
                    DependentProtoNoiseFunctionComponent::Linear(Linear::new(input_index, data)),
                )
            }
            DensityFunctionRepr::Clamp { input, data } => {
                let input_index = recursive_build_proto_stack(
                    input,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                ProtoNoiseFunctionComponent::Dependent(DependentProtoNoiseFunctionComponent::Clamp(
                    Clamp::new(input_index, data),
                ))
            }
            DensityFunctionRepr::Unary { input, data } => {
                let input_index = recursive_build_proto_stack(
                    input,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                ProtoNoiseFunctionComponent::Dependent(DependentProtoNoiseFunctionComponent::Unary(
                    Unary::new(input_index, data),
                ))
            }
            DensityFunctionRepr::WeirdScaled { input, data } => {
                let input_index = recursive_build_proto_stack(
                    input,
                    random_config,
                    stack,
                    map,
                    perlin_noise_builder,
                );

                let sampler = perlin_noise_builder.get_noise_sampler_for_id(&data.noise_id);
                ProtoNoiseFunctionComponent::Dependent(
                    DependentProtoNoiseFunctionComponent::WeirdScaled(WeirdScaled::new(
                        input_index,
                        sampler,
                        data,
                    )),
                )
            }
            DensityFunctionRepr::InterpolatedNoiseSampler { data } => {
                let mut random_generator = random_config
                    .base_random_deriver
                    .split_string("minecraft:terrain");

                ProtoNoiseFunctionComponent::Independent(
                    IndependentProtoNoiseFunctionComponent::InterpolatedNoise(
                        InterpolatedNoiseSampler::new(data, &mut random_generator),
                    ),
                )
            }
        };

        // Invariant: the current component is at the top of the stack
        let component_index = stack.len();
        stack.push(component);
        map.insert(ast_hash, component_index);
        component_index
    })
}

fn recursive_build_spline<'a>(
    spline_ast: &'a SplineRepr,
    random_config: &GlobalRandomConfig,
    stack: &mut Vec<ProtoNoiseFunctionComponent>,
    map: &mut HashMap<u64, usize>,
    perlin_noise_builder: &mut DoublePerlinNoiseBuilder<'a, '_>,
) -> SplineValue {
    match spline_ast {
        SplineRepr::Standard {
            location_function,
            locations,
            values,
            derivatives,
        } => {
            let input_index = recursive_build_proto_stack(
                location_function,
                random_config,
                stack,
                map,
                perlin_noise_builder,
            );

            let points: Vec<_> = locations
                .iter()
                .zip(values)
                .zip(derivatives)
                .map(|((location, v), derivative)| {
                    let value =
                        recursive_build_spline(v, random_config, stack, map, perlin_noise_builder);
                    SplinePoint::new(location.0, value, derivative.0)
                })
                .collect();

            SplineValue::Spline(Spline::new(input_index, points.into_boxed_slice()))
        }
        SplineRepr::Fixed { value } => SplineValue::Fixed(value.0),
    }
}

#[derive(Clone)]
pub struct ProtoChunkNoiseRouter {
    pub barrier_noise: usize,
    pub fluid_level_floodedness_noise: usize,
    pub fluid_level_spread_noise: usize,
    pub lava_noise: usize,
    pub erosion: usize,
    pub depth: usize,
    pub initial_density_without_jaggedness: usize,
    pub final_density: usize,
    pub vein_toggle: usize,
    pub vein_ridged: usize,
    pub vein_gap: usize,
    pub component_stack: Box<[ProtoNoiseFunctionComponent]>,
}

impl ProtoChunkNoiseRouter {
    pub fn generate(ast: &NoiseRouterRepr, random_config: &GlobalRandomConfig) -> Self {
        // Contiguous memory for our function components
        let mut stack = Vec::<ProtoNoiseFunctionComponent>::new();
        // Map of AST hash to index in the stack
        let mut map = HashMap::<u64, usize>::new();
        let mut perlin_noise_builder = DoublePerlinNoiseBuilder::new(random_config);

        Self {
            barrier_noise: recursive_build_proto_stack(
                ast.barrier_noise(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            fluid_level_floodedness_noise: recursive_build_proto_stack(
                ast.fluid_level_floodedness_noise(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            fluid_level_spread_noise: recursive_build_proto_stack(
                ast.fluid_level_spread_noise(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            lava_noise: recursive_build_proto_stack(
                ast.lava_noise(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            depth: recursive_build_proto_stack(
                ast.depth(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            erosion: recursive_build_proto_stack(
                ast.erosion(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            final_density: recursive_build_proto_stack(
                ast.final_density(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            initial_density_without_jaggedness: recursive_build_proto_stack(
                ast.initial_density_without_jaggedness(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            vein_gap: recursive_build_proto_stack(
                ast.vein_gap(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            vein_ridged: recursive_build_proto_stack(
                ast.vein_ridged(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            vein_toggle: recursive_build_proto_stack(
                ast.vein_toggle(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            component_stack: stack.into_boxed_slice(),
        }
    }
}

pub struct ProtoMultiNoiseSampler {
    pub temperature: usize,
    // AKA: Humidity
    pub vegetation: usize,
    pub continents: usize,
    pub erosion: usize,
    pub depth: usize,
    // AKA: Weirdness
    pub ridges: usize,
    pub component_stack: Box<[ProtoNoiseFunctionComponent]>,
}

impl ProtoMultiNoiseSampler {
    pub fn generate(ast: &NoiseRouterRepr, random_config: &GlobalRandomConfig) -> Self {
        // Contiguous memory for our function components
        let mut stack = Vec::<ProtoNoiseFunctionComponent>::new();
        // Map of AST hash to index in the stack
        let mut map = HashMap::<u64, usize>::new();
        let mut perlin_noise_builder = DoublePerlinNoiseBuilder::new(random_config);

        Self {
            temperature: recursive_build_proto_stack(
                ast.temperature(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            vegetation: recursive_build_proto_stack(
                ast.vegetation(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            continents: recursive_build_proto_stack(
                ast.continents(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            erosion: recursive_build_proto_stack(
                ast.erosion(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            depth: recursive_build_proto_stack(
                ast.depth(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            ridges: recursive_build_proto_stack(
                ast.ridges(),
                random_config,
                &mut stack,
                &mut map,
                &mut perlin_noise_builder,
            ),
            component_stack: stack.into_boxed_slice(),
        }
    }
}
