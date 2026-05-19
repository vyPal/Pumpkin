/* This file is generated. Do not edit manually. */
#[allow(clippy::all, unused_imports, dead_code)]
fn build_placed_features()
-> std::collections::HashMap<pumpkin_data::placed_feature::PlacedFeature, PlacedFeature> {
    use crate::block::BlockStateCodec;
    use crate::generation::block_predicate::{
        AllOfBlockPredicate, AnyOfBlockPredicate, BlockPredicate, HasSturdyFacePredicate,
        InsideWorldBoundsBlockPredicate, MatchingBlockTagPredicate, MatchingBlocksBlockPredicate,
        MatchingBlocksWrapper, MatchingFluidsBlockPredicate, NotBlockPredicate,
        OffsetBlocksBlockPredicate, ReplaceableBlockPredicate, SolidBlockPredicate,
        WouldSurviveBlockPredicate,
    };
    use crate::generation::height_provider::{
        HeightProvider, TrapezoidHeightProvider, UniformHeightProvider,
        VeryBiasedToBottomHeightProvider,
    };
    use pumpkin_data::{Block, BlockDirection};
    use pumpkin_util::HeightMap;
    use pumpkin_util::math::int_provider::{
        BiasedToBottomIntProvider, ClampedIntProvider, ClampedNormalIntProvider,
        ConstantIntProvider, IntProvider, NormalIntProvider, TrapezoidIntProvider,
        UniformIntProvider, WeightedEntry, WeightedListIntProvider,
    };
    use pumpkin_util::math::vector3::Vector3;
    use pumpkin_util::y_offset::{AboveBottom, Absolute, BelowTop, YOffset};
    let mut map = std::collections::HashMap::new();
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::Acacia,
        PlacedFeature {
            feature: Feature::Named("acacia".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::ACACIA_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::AcaciaChecked,
        PlacedFeature {
            feature: Feature::Named("acacia".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::ACACIA_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::AmethystGeode,
        PlacedFeature {
            feature: Feature::Named("amethyst_geode".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 24u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 6i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 30i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::Bamboo,
        PlacedFeature {
            feature: Feature::Named("bamboo_some_podzol".to_string()),
            placement: vec![
                PlacementModifier::NoiseBasedCount(NoiseBasedCountPlacementModifier {
                    noise_to_count_ratio: 160i32,
                    noise_factor: 80f64,
                    noise_offset: 0.3f64,
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BambooLight,
        PlacedFeature {
            feature: Feature::Named("bamboo_no_podzol".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 4u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BambooVegetation,
        PlacedFeature {
            feature: Feature::Named("bamboo_vegetation".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(30i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(31i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BasaltBlobs,
        PlacedFeature {
            feature: Feature::Named("basalt_blobs".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(75i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BasaltPillar,
        PlacedFeature {
            feature: Feature::Named("basalt_pillar".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BirchBees0002,
        PlacedFeature {
            feature: Feature::Named("birch_bees_0002".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::BIRCH_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BirchBees0002LeafLitter,
        PlacedFeature {
            feature: Feature::Named("birch_bees_0002_leaf_litter".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::BIRCH_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BirchBees002,
        PlacedFeature {
            feature: Feature::Named("birch_bees_002".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::BIRCH_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BirchChecked,
        PlacedFeature {
            feature: Feature::Named("birch".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::BIRCH_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BirchLeafLitter,
        PlacedFeature {
            feature: Feature::Named("birch_leaf_litter".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::BIRCH_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BirchTall,
        PlacedFeature {
            feature: Feature::Named("birch_tall".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(10i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(11i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BlackstoneBlobs,
        PlacedFeature {
            feature: Feature::Named("blackstone_blobs".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(25i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BlueIce,
        PlacedFeature {
            feature: Feature::Named("blue_ice".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 0i32,
                        max_inclusive: 19i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 30i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 61i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BrownMushroomNether,
        PlacedFeature {
            feature: Feature::Named("brown_mushroom".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 2u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BrownMushroomNormal,
        PlacedFeature {
            feature: Feature::Named("brown_mushroom".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 256u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BrownMushroomOldGrowth,
        PlacedFeature {
            feature: Feature::Named("brown_mushroom".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(3i32),
                }),
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 4u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BrownMushroomSwamp,
        PlacedFeature {
            feature: Feature::Named("brown_mushroom".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::BrownMushroomTaiga,
        PlacedFeature {
            feature: Feature::Named("brown_mushroom".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 4u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::CaveVines,
        PlacedFeature {
            feature: Feature::Named("cave_vine".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(188i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::EnvironmentScan(EnvironmentScanPlacementModifier {
                    direction_of_search: BlockDirection::Up,
                    target_condition: BlockPredicate::HasSturdyFace(HasSturdyFacePredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        direction: BlockDirection::Down,
                    }),
                    allowed_search_condition: Some(BlockPredicate::MatchingBlockTag(
                        MatchingBlockTagPredicate {
                            offset: OffsetBlocksBlockPredicate { offset: None },
                            tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                        },
                    )),
                    max_steps: 12i32,
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Constant(0i32),
                    y_spread: IntProvider::Constant(-1i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::CherryBees005,
        PlacedFeature {
            feature: Feature::Named("cherry_bees_005".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::CHERRY_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::CherryChecked,
        PlacedFeature {
            feature: Feature::Named("cherry".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::CHERRY_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::ChorusPlant,
        PlacedFeature {
            feature: Feature::Named("chorus_plant".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 0i32,
                        max_inclusive: 4i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::ClassicVinesCaveFeature,
        PlacedFeature {
            feature: Feature::Named("vines".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(256i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::CrimsonForestVegetation,
        PlacedFeature {
            feature: Feature::Named("crimson_forest_vegetation".to_string()),
            placement: vec![
                PlacementModifier::CountOnEveryLayer(CountOnEveryLayerPlacementModifier {
                    count: IntProvider::Constant(6i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::CrimsonFungi,
        PlacedFeature {
            feature: Feature::Named("crimson_fungus".to_string()),
            placement: vec![
                PlacementModifier::CountOnEveryLayer(CountOnEveryLayerPlacementModifier {
                    count: IntProvider::Constant(8i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::DarkForestVegetation,
        PlacedFeature {
            feature: Feature::Named("dark_forest_vegetation".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(16i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::DarkOakChecked,
        PlacedFeature {
            feature: Feature::Named("dark_oak".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::DARK_OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::DarkOakLeafLitter,
        PlacedFeature {
            feature: Feature::Named("dark_oak_leaf_litter".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::DARK_OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::Delta,
        PlacedFeature {
            feature: Feature::Named("delta".to_string()),
            placement: vec![
                PlacementModifier::CountOnEveryLayer(CountOnEveryLayerPlacementModifier {
                    count: IntProvider::Constant(40i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::DesertWell,
        PlacedFeature {
            feature: Feature::Named("desert_well".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 1000u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::DiskClay,
        PlacedFeature {
            feature: Feature::Named("disk_clay".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        fluids: MatchingBlocksWrapper::Single("minecraft:water".to_string()),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::DiskGrass,
        PlacedFeature {
            feature: Feature::Named("disk_grass".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(1i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Constant(0i32),
                    y_spread: IntProvider::Constant(-1i32),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        blocks: MatchingBlocksWrapper::Single("minecraft:mud".to_string()),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::DiskGravel,
        PlacedFeature {
            feature: Feature::Named("disk_gravel".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        fluids: MatchingBlocksWrapper::Single("minecraft:water".to_string()),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::DiskSand,
        PlacedFeature {
            feature: Feature::Named("disk_sand".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(3i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        fluids: MatchingBlocksWrapper::Single("minecraft:water".to_string()),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::DripstoneCluster,
        PlacedFeature {
            feature: Feature::Named("dripstone_cluster".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 48i32,
                        max_inclusive: 96i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::EndGatewayReturn,
        PlacedFeature {
            feature: Feature::Named("end_gateway_return".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 700u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Constant(0i32),
                    y_spread: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 3i32,
                        max_inclusive: 9i32,
                    })),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::EndIslandDecorated,
        PlacedFeature {
            feature: Feature::Named("end_island".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 14u32 }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(1i32),
                                    weight: 3i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(2i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 55i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 70i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::EndPlatform,
        PlacedFeature {
            feature: Feature::Named("end_platform".to_string()),
            placement: vec![
                PlacementModifier::FixedPlacement(vec![BlockPos::new(100i32, 49i32, 0i32)]),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::EndSpike,
        PlacedFeature {
            feature: Feature::Named("end_spike".to_string()),
            placement: vec![PlacementModifier::Biome(BiomePlacementModifier)],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FallenBirchTree,
        PlacedFeature {
            feature: Feature::Named("fallen_birch_tree".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::BIRCH_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FallenJungleTree,
        PlacedFeature {
            feature: Feature::Named("fallen_jungle_tree".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::JUNGLE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FallenOakTree,
        PlacedFeature {
            feature: Feature::Named("fallen_oak_tree".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FallenSpruceTree,
        PlacedFeature {
            feature: Feature::Named("fallen_spruce_tree".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::SPRUCE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FallenSuperBirchTree,
        PlacedFeature {
            feature: Feature::Named("fallen_super_birch_tree".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::BIRCH_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FancyOakBees,
        PlacedFeature {
            feature: Feature::Named("fancy_oak_bees".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FancyOakBees0002LeafLitter,
        PlacedFeature {
            feature: Feature::Named("fancy_oak_bees_0002_leaf_litter".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FancyOakBees002,
        PlacedFeature {
            feature: Feature::Named("fancy_oak_bees_002".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FancyOakChecked,
        PlacedFeature {
            feature: Feature::Named("fancy_oak".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FancyOakLeafLitter,
        PlacedFeature {
            feature: Feature::Named("fancy_oak_leaf_litter".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FlowerCherry,
        PlacedFeature {
            feature: Feature::Named("flower_cherry".to_string()),
            placement: vec![
                PlacementModifier::NoiseThresholdCount(NoiseThresholdCountPlacementModifier {
                    noise_level: -0.8f64,
                    below_noise: 5i32,
                    above_noise: 10i32,
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -6i32,
                            max_inclusive: 6i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -2i32,
                            max_inclusive: 2i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FlowerDefault,
        PlacedFeature {
            feature: Feature::Named("flower_default".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 32u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FlowerFlowerForest,
        PlacedFeature {
            feature: Feature::Named("flower_flower_forest".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(3i32),
                }),
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 2u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -6i32,
                            max_inclusive: 6i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -2i32,
                            max_inclusive: 2i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FlowerForestFlowers,
        PlacedFeature {
            feature: Feature::Named("forest_flowers".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 7u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Clamped(ClampedIntProvider {
                        source: Box::new(IntProvider::Object(NormalIntProvider::Uniform(
                            UniformIntProvider {
                                min_inclusive: -1i32,
                                max_inclusive: 3i32,
                            },
                        ))),
                        min_inclusive: 0i32,
                        max_inclusive: 3i32,
                    })),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FlowerMeadow,
        PlacedFeature {
            feature: Feature::Named("flower_meadow".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -6i32,
                            max_inclusive: 6i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -2i32,
                            max_inclusive: 2i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FlowerPaleGarden,
        PlacedFeature {
            feature: Feature::Named("flower_pale_garden".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 32u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FlowerPlain,
        PlacedFeature {
            feature: Feature::Named("flower_plain".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(64i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -6i32,
                            max_inclusive: 6i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -2i32,
                            max_inclusive: 2i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FlowerPlains,
        PlacedFeature {
            feature: Feature::Named("flower_plain".to_string()),
            placement: vec![
                PlacementModifier::NoiseThresholdCount(NoiseThresholdCountPlacementModifier {
                    noise_level: -0.8f64,
                    below_noise: 15i32,
                    above_noise: 4i32,
                }),
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 32u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(64i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -6i32,
                            max_inclusive: 6i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -2i32,
                            max_inclusive: 2i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FlowerSwamp,
        PlacedFeature {
            feature: Feature::Named("flower_swamp".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 32u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(64i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -6i32,
                            max_inclusive: 6i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -2i32,
                            max_inclusive: 2i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FlowerWarm,
        PlacedFeature {
            feature: Feature::Named("flower_default".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 16u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::ForestFlowers,
        PlacedFeature {
            feature: Feature::Named("forest_flowers".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 7u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Clamped(ClampedIntProvider {
                        source: Box::new(IntProvider::Object(NormalIntProvider::Uniform(
                            UniformIntProvider {
                                min_inclusive: -3i32,
                                max_inclusive: 1i32,
                            },
                        ))),
                        min_inclusive: 0i32,
                        max_inclusive: 1i32,
                    })),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::ForestRock,
        PlacedFeature {
            feature: Feature::Named("forest_rock".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FossilLower,
        PlacedFeature {
            feature: Feature::Named("fossil_diamonds".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 64u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: -8i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FossilUpper,
        PlacedFeature {
            feature: Feature::Named("fossil_coal".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 64u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 0i16 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::FreezeTopLayer,
        PlacedFeature {
            feature: Feature::Named("freeze_top_layer".to_string()),
            placement: vec![PlacementModifier::Biome(BiomePlacementModifier)],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::GlowLichen,
        PlacedFeature {
            feature: Feature::Named("glow_lichen".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 104i32,
                        max_inclusive: 157i32,
                    })),
                }),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceRelativeThresholdFilter(
                    SurfaceThresholdFilterPlacementModifier {
                        heightmap: HeightMap::OceanFloorWg,
                        min_inclusive: None,
                        max_inclusive: Some(-13i32),
                    },
                ),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::Glowstone,
        PlacedFeature {
            feature: Feature::Named("glowstone_extra".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::GlowstoneExtra,
        PlacedFeature {
            feature: Feature::Named("glowstone_extra".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::BiasedToBottom(
                        BiasedToBottomIntProvider {
                            min_inclusive: 0i32,
                            max_inclusive: 9i32,
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 4i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 4i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::GrassBonemeal,
        PlacedFeature {
            feature: Feature::Named("grass".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::IcePatch,
        PlacedFeature {
            feature: Feature::Named("ice_patch".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Constant(0i32),
                    y_spread: IntProvider::Constant(-1i32),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        blocks: MatchingBlocksWrapper::Single("minecraft:snow_block".to_string()),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::IceSpike,
        PlacedFeature {
            feature: Feature::Named("ice_spike".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(3i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::IcebergBlue,
        PlacedFeature {
            feature: Feature::Named("iceberg_blue".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 200u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::IcebergPacked,
        PlacedFeature {
            feature: Feature::Named("iceberg_packed".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 16u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::JungleBush,
        PlacedFeature {
            feature: Feature::Named("jungle_bush".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::JungleTree,
        PlacedFeature {
            feature: Feature::Named("jungle_tree".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::JUNGLE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::KelpCold,
        PlacedFeature {
            feature: Feature::Named("kelp".to_string()),
            placement: vec![
                PlacementModifier::NoiseBasedCount(NoiseBasedCountPlacementModifier {
                    noise_to_count_ratio: 120i32,
                    noise_factor: 80f64,
                    noise_offset: 0f64,
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::KelpWarm,
        PlacedFeature {
            feature: Feature::Named("kelp".to_string()),
            placement: vec![
                PlacementModifier::NoiseBasedCount(NoiseBasedCountPlacementModifier {
                    noise_to_count_ratio: 80i32,
                    noise_factor: 80f64,
                    noise_offset: 0f64,
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::LakeLavaSurface,
        PlacedFeature {
            feature: Feature::Named("lake_lava".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 200u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::LakeLavaUnderground,
        PlacedFeature {
            feature: Feature::Named("lake_lava".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 9u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 0i16 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::EnvironmentScan(EnvironmentScanPlacementModifier {
                    direction_of_search: BlockDirection::Down,
                    target_condition: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::Not(NotBlockPredicate {
                                predicate: Box::new(BlockPredicate::MatchingBlockTag(
                                    MatchingBlockTagPredicate {
                                        offset: OffsetBlocksBlockPredicate { offset: None },
                                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                                    },
                                )),
                            }),
                            BlockPredicate::InsideWorldBounds(InsideWorldBoundsBlockPredicate {
                                offset: Vector3::new(0i32, -5i32, 0i32),
                            }),
                        ],
                    }),
                    allowed_search_condition: None,
                    max_steps: 32i32,
                }),
                PlacementModifier::SurfaceRelativeThresholdFilter(
                    SurfaceThresholdFilterPlacementModifier {
                        heightmap: HeightMap::OceanFloorWg,
                        min_inclusive: None,
                        max_inclusive: Some(-5i32),
                    },
                ),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::LargeBasaltColumns,
        PlacedFeature {
            feature: Feature::Named("large_basalt_columns".to_string()),
            placement: vec![
                PlacementModifier::CountOnEveryLayer(CountOnEveryLayerPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::LargeDripstone,
        PlacedFeature {
            feature: Feature::Named("large_dripstone".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 10i32,
                        max_inclusive: 48i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::LushCavesCeilingVegetation,
        PlacedFeature {
            feature: Feature::Named("moss_patch_ceiling".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(125i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::EnvironmentScan(EnvironmentScanPlacementModifier {
                    direction_of_search: BlockDirection::Up,
                    target_condition: BlockPredicate::Solid(SolidBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                    }),
                    allowed_search_condition: Some(BlockPredicate::MatchingBlockTag(
                        MatchingBlockTagPredicate {
                            offset: OffsetBlocksBlockPredicate { offset: None },
                            tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                        },
                    )),
                    max_steps: 12i32,
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Constant(0i32),
                    y_spread: IntProvider::Constant(-1i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::LushCavesClay,
        PlacedFeature {
            feature: Feature::Named("lush_caves_clay".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(62i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::EnvironmentScan(EnvironmentScanPlacementModifier {
                    direction_of_search: BlockDirection::Down,
                    target_condition: BlockPredicate::Solid(SolidBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                    }),
                    allowed_search_condition: Some(BlockPredicate::MatchingBlockTag(
                        MatchingBlockTagPredicate {
                            offset: OffsetBlocksBlockPredicate { offset: None },
                            tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                        },
                    )),
                    max_steps: 12i32,
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Constant(0i32),
                    y_spread: IntProvider::Constant(1i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::LushCavesVegetation,
        PlacedFeature {
            feature: Feature::Named("moss_patch".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(125i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::EnvironmentScan(EnvironmentScanPlacementModifier {
                    direction_of_search: BlockDirection::Down,
                    target_condition: BlockPredicate::Solid(SolidBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                    }),
                    allowed_search_condition: Some(BlockPredicate::MatchingBlockTag(
                        MatchingBlockTagPredicate {
                            offset: OffsetBlocksBlockPredicate { offset: None },
                            tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                        },
                    )),
                    max_steps: 12i32,
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Constant(0i32),
                    y_spread: IntProvider::Constant(1i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::MangroveChecked,
        PlacedFeature {
            feature: Feature::Named("mangrove".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("waterlogged".to_string(), "false".to_string());
                            props.insert("stage".to_string(), "0".to_string());
                            props.insert("hanging".to_string(), "false".to_string());
                            props.insert("age".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::MANGROVE_PROPAGULE,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::MegaJungleTreeChecked,
        PlacedFeature {
            feature: Feature::Named("mega_jungle_tree".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::JUNGLE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::MegaPineChecked,
        PlacedFeature {
            feature: Feature::Named("mega_pine".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::SPRUCE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::MegaSpruceChecked,
        PlacedFeature {
            feature: Feature::Named("mega_spruce".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::SPRUCE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::MonsterRoom,
        PlacedFeature {
            feature: Feature::Named("monster_room".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 0i16 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::MonsterRoomDeep,
        PlacedFeature {
            feature: Feature::Named("monster_room".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 6i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: -1i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::MushroomIslandVegetation,
        PlacedFeature {
            feature: Feature::Named("mushroom_island_vegetation".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::NetherSprouts,
        PlacedFeature {
            feature: Feature::Named("nether_sprouts".to_string()),
            placement: vec![
                PlacementModifier::CountOnEveryLayer(CountOnEveryLayerPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::Oak,
        PlacedFeature {
            feature: Feature::Named("oak".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OakBees0002LeafLitter,
        PlacedFeature {
            feature: Feature::Named("oak_bees_0002_leaf_litter".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OakBees002,
        PlacedFeature {
            feature: Feature::Named("oak_bees_002".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OakChecked,
        PlacedFeature {
            feature: Feature::Named("oak".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OakLeafLitter,
        PlacedFeature {
            feature: Feature::Named("oak_leaf_litter".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreAncientDebrisLarge,
        PlacedFeature {
            feature: Feature::Named("ore_ancient_debris_large".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 8i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 24i16 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreAndesiteLower,
        PlacedFeature {
            feature: Feature::Named("ore_andesite".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 0i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 60i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreAndesiteUpper,
        PlacedFeature {
            feature: Feature::Named("ore_andesite".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 6u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 64i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 128i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreBlackstone,
        PlacedFeature {
            feature: Feature::Named("ore_blackstone".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 5i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 31i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreClay,
        PlacedFeature {
            feature: Feature::Named("ore_clay".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(46i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreCoalLower,
        PlacedFeature {
            feature: Feature::Named("ore_coal_buried".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 0i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 192i16 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreCoalUpper,
        PlacedFeature {
            feature: Feature::Named("ore_coal".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(30i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 136i16 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreCopper,
        PlacedFeature {
            feature: Feature::Named("ore_copper_small".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(16i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: -16i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 112i16 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreCopperLarge,
        PlacedFeature {
            feature: Feature::Named("ore_copper_large".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(16i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: -16i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 112i16 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreDebrisSmall,
        PlacedFeature {
            feature: Feature::Named("ore_ancient_debris_small".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 8i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 8i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreDiamond,
        PlacedFeature {
            feature: Feature::Named("ore_diamond_small".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(7i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom {
                            above_bottom: -80i8,
                        }),
                        max_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 80i8 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreDiamondBuried,
        PlacedFeature {
            feature: Feature::Named("ore_diamond_buried".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom {
                            above_bottom: -80i8,
                        }),
                        max_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 80i8 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreDiamondLarge,
        PlacedFeature {
            feature: Feature::Named("ore_diamond_large".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 9u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom {
                            above_bottom: -80i8,
                        }),
                        max_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 80i8 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreDiamondMedium,
        PlacedFeature {
            feature: Feature::Named("ore_diamond_medium".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: -64i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: -4i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreDioriteLower,
        PlacedFeature {
            feature: Feature::Named("ore_diorite".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 0i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 60i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreDioriteUpper,
        PlacedFeature {
            feature: Feature::Named("ore_diorite".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 6u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 64i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 128i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreDirt,
        PlacedFeature {
            feature: Feature::Named("ore_dirt".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(7i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 0i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 160i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreEmerald,
        PlacedFeature {
            feature: Feature::Named("ore_emerald".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(100i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: -16i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 480i16 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreGold,
        PlacedFeature {
            feature: Feature::Named("ore_gold_buried".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: -64i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 32i16 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreGoldDeltas,
        PlacedFeature {
            feature: Feature::Named("ore_nether_gold".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 10i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 10i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreGoldExtra,
        PlacedFeature {
            feature: Feature::Named("ore_gold".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(50i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 32i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreGoldLower,
        PlacedFeature {
            feature: Feature::Named("ore_gold_buried".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 0i32,
                        max_inclusive: 1i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: -64i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: -48i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreGoldNether,
        PlacedFeature {
            feature: Feature::Named("ore_nether_gold".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 10i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 10i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreGraniteLower,
        PlacedFeature {
            feature: Feature::Named("ore_granite".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 0i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 60i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreGraniteUpper,
        PlacedFeature {
            feature: Feature::Named("ore_granite".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 6u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 64i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 128i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreGravel,
        PlacedFeature {
            feature: Feature::Named("ore_gravel".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(14i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreGravelNether,
        PlacedFeature {
            feature: Feature::Named("ore_gravel_nether".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 5i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 41i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreInfested,
        PlacedFeature {
            feature: Feature::Named("ore_infested".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(14i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 63i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreIronMiddle,
        PlacedFeature {
            feature: Feature::Named("ore_iron".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: -24i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 56i16 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreIronSmall,
        PlacedFeature {
            feature: Feature::Named("ore_iron_small".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 72i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreIronUpper,
        PlacedFeature {
            feature: Feature::Named("ore_iron".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(90i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 80i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 384i16 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreLapis,
        PlacedFeature {
            feature: Feature::Named("ore_lapis".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: -32i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 32i16 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreLapisBuried,
        PlacedFeature {
            feature: Feature::Named("ore_lapis_buried".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 64i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreMagma,
        PlacedFeature {
            feature: Feature::Named("ore_magma".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 27i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 36i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreQuartzDeltas,
        PlacedFeature {
            feature: Feature::Named("ore_quartz".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 10i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 10i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreQuartzNether,
        PlacedFeature {
            feature: Feature::Named("ore_quartz".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(16i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 10i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 10i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreRedstone,
        PlacedFeature {
            feature: Feature::Named("ore_redstone".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 15i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreRedstoneLower,
        PlacedFeature {
            feature: Feature::Named("ore_redstone".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(8i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Trapezoid(TrapezoidHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom {
                            above_bottom: -32i8,
                        }),
                        max_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 32i8 }),
                        plateau: None,
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreSoulSand,
        PlacedFeature {
            feature: Feature::Named("ore_soul_sand".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(12i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 31i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::OreTuff,
        PlacedFeature {
            feature: Feature::Named("ore_tuff".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 0i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PaleGardenFlowers,
        PlacedFeature {
            feature: Feature::Named("pale_forest_flower".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 8u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlockingNoLeaves,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PaleGardenVegetation,
        PlacedFeature {
            feature: Feature::Named("pale_garden_vegetation".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(16i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PaleMossPatch,
        PlacedFeature {
            feature: Feature::Named("pale_moss_patch".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(1i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlockingNoLeaves,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PaleOakChecked,
        PlacedFeature {
            feature: Feature::Named("pale_oak".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::PALE_OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PaleOakCreakingChecked,
        PlacedFeature {
            feature: Feature::Named("pale_oak_creaking".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::PALE_OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchBerryBush,
        PlacedFeature {
            feature: Feature::Named("berry_bush".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate {
                                    offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                                },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:grass_block".to_string(),
                                ),
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchBerryCommon,
        PlacedFeature {
            feature: Feature::Named("berry_bush".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 32u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate {
                                    offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                                },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:grass_block".to_string(),
                                ),
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchBerryRare,
        PlacedFeature {
            feature: Feature::Named("berry_bush".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 384u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate {
                                    offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                                },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:grass_block".to_string(),
                                ),
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchBush,
        PlacedFeature {
            feature: Feature::Named("bush".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 4u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(24i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -5i32,
                            max_inclusive: 5i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchCactus,
        PlacedFeature {
            feature: Feature::Named("cactus".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                state: {
                                    let mut props = std::collections::HashMap::new();
                                    props.insert("age".to_string(), "0".to_string());
                                    BlockStateCodec {
                                        name: &pumpkin_data::Block::CACTUS,
                                        properties: Some(props),
                                    }
                                },
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchCactusDecorated,
        PlacedFeature {
            feature: Feature::Named("cactus".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 13u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                state: {
                                    let mut props = std::collections::HashMap::new();
                                    props.insert("age".to_string(), "0".to_string());
                                    BlockStateCodec {
                                        name: &pumpkin_data::Block::CACTUS,
                                        properties: Some(props),
                                    }
                                },
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchCactusDesert,
        PlacedFeature {
            feature: Feature::Named("cactus".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 6u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                state: {
                                    let mut props = std::collections::HashMap::new();
                                    props.insert("age".to_string(), "0".to_string());
                                    BlockStateCodec {
                                        name: &pumpkin_data::Block::CACTUS,
                                        properties: Some(props),
                                    }
                                },
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchCrimsonRoots,
        PlacedFeature {
            feature: Feature::Named("crimson_roots".to_string()),
            placement: vec![
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchDeadBush,
        PlacedFeature {
            feature: Feature::Named("dead_bush".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchDeadBush2,
        PlacedFeature {
            feature: Feature::Named("dead_bush".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchDeadBushBadlands,
        PlacedFeature {
            feature: Feature::Named("dead_bush".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchDryGrassBadlands,
        PlacedFeature {
            feature: Feature::Named("dry_grass".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 6u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(64i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchDryGrassDesert,
        PlacedFeature {
            feature: Feature::Named("dry_grass".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 3u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(64i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchFire,
        PlacedFeature {
            feature: Feature::Named("patch_fire".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 0i32,
                        max_inclusive: 5i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 4i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 4i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate {
                                    offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                                },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:netherrack".to_string(),
                                ),
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchFireflyBushNearWater,
        PlacedFeature {
            feature: Feature::Named("firefly_bush".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlockingNoLeaves,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                state: BlockStateCodec {
                                    name: &pumpkin_data::Block::FIREFLY_BUSH,
                                    properties: None,
                                },
                            }),
                            BlockPredicate::AnyOf(AnyOfBlockPredicate {
                                predicates: vec![
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(-1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, 1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, -1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                ],
                            }),
                        ],
                    }),
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -4i32,
                            max_inclusive: 4i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchFireflyBushNearWaterSwamp,
        PlacedFeature {
            feature: Feature::Named("firefly_bush".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(3i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                state: BlockStateCodec {
                                    name: &pumpkin_data::Block::FIREFLY_BUSH,
                                    properties: None,
                                },
                            }),
                            BlockPredicate::AnyOf(AnyOfBlockPredicate {
                                predicates: vec![
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(-1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, 1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, -1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                ],
                            }),
                        ],
                    }),
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -4i32,
                            max_inclusive: 4i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchFireflyBushSwamp,
        PlacedFeature {
            feature: Feature::Named("firefly_bush".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 8u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -4i32,
                            max_inclusive: 4i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchGrassBadlands,
        PlacedFeature {
            feature: Feature::Named("grass".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchGrassForest,
        PlacedFeature {
            feature: Feature::Named("grass".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchGrassJungle,
        PlacedFeature {
            feature: Feature::Named("grass_jungle".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(25i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::Not(NotBlockPredicate {
                                predicate: Box::new(BlockPredicate::MatchingBlocks(
                                    MatchingBlocksBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                                        },
                                        blocks: MatchingBlocksWrapper::Single(
                                            "minecraft:podzol".to_string(),
                                        ),
                                    },
                                )),
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchGrassMeadow,
        PlacedFeature {
            feature: Feature::Named("grass".to_string()),
            placement: vec![
                PlacementModifier::NoiseThresholdCount(NoiseThresholdCountPlacementModifier {
                    noise_level: -0.8f64,
                    below_noise: 5i32,
                    above_noise: 10i32,
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(16i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchGrassNormal,
        PlacedFeature {
            feature: Feature::Named("grass".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(5i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchGrassPlain,
        PlacedFeature {
            feature: Feature::Named("grass".to_string()),
            placement: vec![
                PlacementModifier::NoiseThresholdCount(NoiseThresholdCountPlacementModifier {
                    noise_level: -0.8f64,
                    below_noise: 5i32,
                    above_noise: 10i32,
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchGrassSavanna,
        PlacedFeature {
            feature: Feature::Named("grass".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchGrassTaiga,
        PlacedFeature {
            feature: Feature::Named("taiga_grass".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(7i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchGrassTaiga2,
        PlacedFeature {
            feature: Feature::Named("taiga_grass".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchLargeFern,
        PlacedFeature {
            feature: Feature::Named("large_fern".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 5u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchLeafLitter,
        PlacedFeature {
            feature: Feature::Named("leaf_litter".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(2i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate {
                                    offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                                },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:grass_block".to_string(),
                                ),
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchMelon,
        PlacedFeature {
            feature: Feature::Named("melon".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 6u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(64i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::Replaceable(ReplaceableBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                            }),
                            BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                fluids: MatchingBlocksWrapper::Single(
                                    "minecraft:empty".to_string(),
                                ),
                            }),
                            BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate {
                                    offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                                },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:grass_block".to_string(),
                                ),
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchMelonSparse,
        PlacedFeature {
            feature: Feature::Named("melon".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 64u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(64i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::Replaceable(ReplaceableBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                            }),
                            BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                fluids: MatchingBlocksWrapper::Single(
                                    "minecraft:empty".to_string(),
                                ),
                            }),
                            BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate {
                                    offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                                },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:grass_block".to_string(),
                                ),
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchPumpkin,
        PlacedFeature {
            feature: Feature::Named("pumpkin".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 300u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate {
                                    offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                                },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:grass_block".to_string(),
                                ),
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchSoulFire,
        PlacedFeature {
            feature: Feature::Named("patch_soul_fire".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 0i32,
                        max_inclusive: 5i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 4i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 4i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate {
                                    offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                                },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:soul_soil".to_string(),
                                ),
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchSugarCane,
        PlacedFeature {
            feature: Feature::Named("sugar_cane".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 6u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -4i32,
                            max_inclusive: 4i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: 0i32,
                            max_inclusive: 0i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                state: {
                                    let mut props = std::collections::HashMap::new();
                                    props.insert("age".to_string(), "0".to_string());
                                    BlockStateCodec {
                                        name: &pumpkin_data::Block::SUGAR_CANE,
                                        properties: Some(props),
                                    }
                                },
                            }),
                            BlockPredicate::AnyOf(AnyOfBlockPredicate {
                                predicates: vec![
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(-1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, 1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, -1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                ],
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchSugarCaneBadlands,
        PlacedFeature {
            feature: Feature::Named("sugar_cane".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 5u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -4i32,
                            max_inclusive: 4i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: 0i32,
                            max_inclusive: 0i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                state: {
                                    let mut props = std::collections::HashMap::new();
                                    props.insert("age".to_string(), "0".to_string());
                                    BlockStateCodec {
                                        name: &pumpkin_data::Block::SUGAR_CANE,
                                        properties: Some(props),
                                    }
                                },
                            }),
                            BlockPredicate::AnyOf(AnyOfBlockPredicate {
                                predicates: vec![
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(-1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, 1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, -1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                ],
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchSugarCaneDesert,
        PlacedFeature {
            feature: Feature::Named("sugar_cane".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -4i32,
                            max_inclusive: 4i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: 0i32,
                            max_inclusive: 0i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                state: {
                                    let mut props = std::collections::HashMap::new();
                                    props.insert("age".to_string(), "0".to_string());
                                    BlockStateCodec {
                                        name: &pumpkin_data::Block::SUGAR_CANE,
                                        properties: Some(props),
                                    }
                                },
                            }),
                            BlockPredicate::AnyOf(AnyOfBlockPredicate {
                                predicates: vec![
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(-1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, 1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, -1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                ],
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchSugarCaneSwamp,
        PlacedFeature {
            feature: Feature::Named("sugar_cane".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 3u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -4i32,
                            max_inclusive: 4i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: 0i32,
                            max_inclusive: 0i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::AllOf(AllOfBlockPredicate {
                        predicates: vec![
                            BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                            }),
                            BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                state: {
                                    let mut props = std::collections::HashMap::new();
                                    props.insert("age".to_string(), "0".to_string());
                                    BlockStateCodec {
                                        name: &pumpkin_data::Block::SUGAR_CANE,
                                        properties: Some(props),
                                    }
                                },
                            }),
                            BlockPredicate::AnyOf(AnyOfBlockPredicate {
                                predicates: vec![
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(-1i32, -1i32, 0i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, 1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                    BlockPredicate::MatchingFluids(MatchingFluidsBlockPredicate {
                                        offset: OffsetBlocksBlockPredicate {
                                            offset: Some(Vector3::new(0i32, -1i32, -1i32)),
                                        },
                                        fluids: MatchingBlocksWrapper::Multiple(vec![
                                            "minecraft:water".to_string(),
                                            "minecraft:flowing_water".to_string(),
                                        ]),
                                    }),
                                ],
                            }),
                        ],
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchSunflower,
        PlacedFeature {
            feature: Feature::Named("sunflower".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 3u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchTaigaGrass,
        PlacedFeature {
            feature: Feature::Named("taiga_grass".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchTallGrass,
        PlacedFeature {
            feature: Feature::Named("tall_grass".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 5u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchTallGrass2,
        PlacedFeature {
            feature: Feature::Named("tall_grass".to_string()),
            placement: vec![
                PlacementModifier::NoiseThresholdCount(NoiseThresholdCountPlacementModifier {
                    noise_level: -0.8f64,
                    below_noise: 0i32,
                    above_noise: 7i32,
                }),
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 32u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PatchWaterlily,
        PlacedFeature {
            feature: Feature::Named("waterlily".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::WorldSurfaceWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PileHay,
        PlacedFeature {
            feature: Feature::Named("pile_hay".to_string()),
            placement: vec![],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PileIce,
        PlacedFeature {
            feature: Feature::Named("pile_ice".to_string()),
            placement: vec![],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PileMelon,
        PlacedFeature {
            feature: Feature::Named("pile_melon".to_string()),
            placement: vec![],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PilePumpkin,
        PlacedFeature {
            feature: Feature::Named("pile_pumpkin".to_string()),
            placement: vec![],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PileSnow,
        PlacedFeature {
            feature: Feature::Named("pile_snow".to_string()),
            placement: vec![],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::Pine,
        PlacedFeature {
            feature: Feature::Named("pine".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::SPRUCE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PineChecked,
        PlacedFeature {
            feature: Feature::Named("pine".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::SPRUCE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PineOnSnow,
        PlacedFeature {
            feature: Feature::Named("pine".to_string()),
            placement: vec![
                PlacementModifier::EnvironmentScan(EnvironmentScanPlacementModifier {
                    direction_of_search: BlockDirection::Up,
                    target_condition: BlockPredicate::Not(NotBlockPredicate {
                        predicate: Box::new(BlockPredicate::MatchingBlocks(
                            MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:powder_snow".to_string(),
                                ),
                            },
                        )),
                    }),
                    allowed_search_condition: None,
                    max_steps: 8i32,
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                        offset: OffsetBlocksBlockPredicate {
                            offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                        },
                        blocks: MatchingBlocksWrapper::Multiple(vec![
                            "minecraft:snow_block".to_string(),
                            "minecraft:powder_snow".to_string(),
                        ]),
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::PointedDripstone,
        PlacedFeature {
            feature: Feature::Named("pointed_dripstone".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 192i32,
                        max_inclusive: 256i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 1i32,
                        max_inclusive: 5i32,
                    })),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::ClampedNormal(
                        ClampedNormalIntProvider {
                            mean: 0f32,
                            deviation: 3f32,
                            min_inclusive: -10i32,
                            max_inclusive: 10i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::ClampedNormal(
                        ClampedNormalIntProvider {
                            mean: 0f32,
                            deviation: 0.6f32,
                            min_inclusive: -2i32,
                            max_inclusive: 2i32,
                        },
                    )),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::RedMushroomNether,
        PlacedFeature {
            feature: Feature::Named("red_mushroom".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 2u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::RedMushroomNormal,
        PlacedFeature {
            feature: Feature::Named("red_mushroom".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 512u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::RedMushroomOldGrowth,
        PlacedFeature {
            feature: Feature::Named("red_mushroom".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 171u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::RedMushroomSwamp,
        PlacedFeature {
            feature: Feature::Named("red_mushroom".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 64u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::RedMushroomTaiga,
        PlacedFeature {
            feature: Feature::Named("red_mushroom".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 256u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(96i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -7i32,
                            max_inclusive: 7i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -3i32,
                            max_inclusive: 3i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::RootedAzaleaTree,
        PlacedFeature {
            feature: Feature::Named("rooted_azalea_tree".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 1i32,
                        max_inclusive: 2i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::EnvironmentScan(EnvironmentScanPlacementModifier {
                    direction_of_search: BlockDirection::Up,
                    target_condition: BlockPredicate::Solid(SolidBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                    }),
                    allowed_search_condition: Some(BlockPredicate::MatchingBlockTag(
                        MatchingBlockTagPredicate {
                            offset: OffsetBlocksBlockPredicate { offset: None },
                            tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                        },
                    )),
                    max_steps: 12i32,
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Constant(0i32),
                    y_spread: IntProvider::Constant(-1i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SculkPatchAncientCity,
        PlacedFeature {
            feature: Feature::Named("sculk_patch_ancient_city".to_string()),
            placement: vec![],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SculkPatchDeepDark,
        PlacedFeature {
            feature: Feature::Named("sculk_patch_deep_dark".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(256i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SculkVein,
        PlacedFeature {
            feature: Feature::Named("sculk_vein".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 204i32,
                        max_inclusive: 250i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SeaPickle,
        PlacedFeature {
            feature: Feature::Named("sea_pickle".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 16u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SeagrassCold,
        PlacedFeature {
            feature: Feature::Named("seagrass_short".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SeagrassDeep,
        PlacedFeature {
            feature: Feature::Named("seagrass_tall".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(48i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SeagrassDeepCold,
        PlacedFeature {
            feature: Feature::Named("seagrass_tall".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(40i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SeagrassDeepWarm,
        PlacedFeature {
            feature: Feature::Named("seagrass_tall".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(80i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SeagrassNormal,
        PlacedFeature {
            feature: Feature::Named("seagrass_short".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(48i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SeagrassRiver,
        PlacedFeature {
            feature: Feature::Named("seagrass_slightly_less_short".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(48i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SeagrassSwamp,
        PlacedFeature {
            feature: Feature::Named("seagrass_mid".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(64i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SeagrassWarm,
        PlacedFeature {
            feature: Feature::Named("seagrass_short".to_string()),
            placement: vec![
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(80i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SmallBasaltColumns,
        PlacedFeature {
            feature: Feature::Named("small_basalt_columns".to_string()),
            placement: vec![
                PlacementModifier::CountOnEveryLayer(CountOnEveryLayerPlacementModifier {
                    count: IntProvider::Constant(4i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SporeBlossom,
        PlacedFeature {
            feature: Feature::Named("spore_blossom".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(25i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::EnvironmentScan(EnvironmentScanPlacementModifier {
                    direction_of_search: BlockDirection::Up,
                    target_condition: BlockPredicate::Solid(SolidBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                    }),
                    allowed_search_condition: Some(BlockPredicate::MatchingBlockTag(
                        MatchingBlockTagPredicate {
                            offset: OffsetBlocksBlockPredicate { offset: None },
                            tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                        },
                    )),
                    max_steps: 12i32,
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Constant(0i32),
                    y_spread: IntProvider::Constant(-1i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SpringClosed,
        PlacedFeature {
            feature: Feature::Named("spring_nether_closed".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(16i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 10i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 10i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SpringClosedDouble,
        PlacedFeature {
            feature: Feature::Named("spring_nether_closed".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(32i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 10i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 10i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SpringDelta,
        PlacedFeature {
            feature: Feature::Named("spring_lava_nether".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(16i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 4i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 4i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SpringLava,
        PlacedFeature {
            feature: Feature::Named("spring_lava_overworld".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::VeryBiasedToBottom(VeryBiasedToBottomHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 8i8 }),
                        inner: std::num::NonZeroU32::new(8u32),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SpringLavaFrozen,
        PlacedFeature {
            feature: Feature::Named("spring_lava_frozen".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(20i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::VeryBiasedToBottom(VeryBiasedToBottomHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 8i8 }),
                        inner: std::num::NonZeroU32::new(8u32),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SpringOpen,
        PlacedFeature {
            feature: Feature::Named("spring_nether_open".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(8i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 4i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 4i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SpringWater,
        PlacedFeature {
            feature: Feature::Named("spring_water".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(25i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 192i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::Spruce,
        PlacedFeature {
            feature: Feature::Named("spruce".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::SPRUCE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SpruceChecked,
        PlacedFeature {
            feature: Feature::Named("spruce".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::SPRUCE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SpruceOnSnow,
        PlacedFeature {
            feature: Feature::Named("spruce".to_string()),
            placement: vec![
                PlacementModifier::EnvironmentScan(EnvironmentScanPlacementModifier {
                    direction_of_search: BlockDirection::Up,
                    target_condition: BlockPredicate::Not(NotBlockPredicate {
                        predicate: Box::new(BlockPredicate::MatchingBlocks(
                            MatchingBlocksBlockPredicate {
                                offset: OffsetBlocksBlockPredicate { offset: None },
                                blocks: MatchingBlocksWrapper::Single(
                                    "minecraft:powder_snow".to_string(),
                                ),
                            },
                        )),
                    }),
                    allowed_search_condition: None,
                    max_steps: 8i32,
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlocks(MatchingBlocksBlockPredicate {
                        offset: OffsetBlocksBlockPredicate {
                            offset: Some(Vector3::new(0i32, -1i32, 0i32)),
                        },
                        blocks: MatchingBlocksWrapper::Multiple(vec![
                            "minecraft:snow_block".to_string(),
                            "minecraft:powder_snow".to_string(),
                        ]),
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SuperBirchBees,
        PlacedFeature {
            feature: Feature::Named("super_birch_bees".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::BIRCH_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::SuperBirchBees0002,
        PlacedFeature {
            feature: Feature::Named("super_birch_bees_0002".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::BIRCH_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TallMangroveChecked,
        PlacedFeature {
            feature: Feature::Named("tall_mangrove".to_string()),
            placement: vec![PlacementModifier::BlockPredicateFilter(
                BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("waterlogged".to_string(), "false".to_string());
                            props.insert("stage".to_string(), "0".to_string());
                            props.insert("hanging".to_string(), "false".to_string());
                            props.insert("age".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::MANGROVE_PROPAGULE,
                                properties: Some(props),
                            }
                        },
                    }),
                },
            )],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesBadlands,
        PlacedFeature {
            feature: Feature::Named("trees_badlands".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(5i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(6i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesBirch,
        PlacedFeature {
            feature: Feature::Named("trees_birch".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(10i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(11i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::BIRCH_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesBirchAndOakLeafLitter,
        PlacedFeature {
            feature: Feature::Named("trees_birch_and_oak_leaf_litter".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(10i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(11i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesCherry,
        PlacedFeature {
            feature: Feature::Named("cherry_bees_005".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(10i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(11i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::CHERRY_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesFlowerForest,
        PlacedFeature {
            feature: Feature::Named("trees_flower_forest".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(6i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(7i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesGrove,
        PlacedFeature {
            feature: Feature::Named("trees_grove".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(10i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(11i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesJungle,
        PlacedFeature {
            feature: Feature::Named("trees_jungle".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(50i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(51i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesMangrove,
        PlacedFeature {
            feature: Feature::Named("mangrove_vegetation".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(25i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 5i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesMeadow,
        PlacedFeature {
            feature: Feature::Named("meadow_trees".to_string()),
            placement: vec![
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 100u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesOldGrowthPineTaiga,
        PlacedFeature {
            feature: Feature::Named("trees_old_growth_pine_taiga".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(10i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(11i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesOldGrowthSpruceTaiga,
        PlacedFeature {
            feature: Feature::Named("trees_old_growth_spruce_taiga".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(10i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(11i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesPlains,
        PlacedFeature {
            feature: Feature::Named("trees_plains".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(0i32),
                                    weight: 19i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(1i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesSavanna,
        PlacedFeature {
            feature: Feature::Named("trees_savanna".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(1i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(2i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesSnowy,
        PlacedFeature {
            feature: Feature::Named("trees_snowy".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(0i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(1i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::SPRUCE_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesSparseJungle,
        PlacedFeature {
            feature: Feature::Named("trees_sparse_jungle".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(2i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(3i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesSwamp,
        PlacedFeature {
            feature: Feature::Named("swamp_oak".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(2i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(3i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 2i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::WouldSurvive(WouldSurviveBlockPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        state: {
                            let mut props = std::collections::HashMap::new();
                            props.insert("stage".to_string(), "0".to_string());
                            BlockStateCodec {
                                name: &pumpkin_data::Block::OAK_SAPLING,
                                properties: Some(props),
                            }
                        },
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesTaiga,
        PlacedFeature {
            feature: Feature::Named("trees_taiga".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(10i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(11i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesWater,
        PlacedFeature {
            feature: Feature::Named("trees_water".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(0i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(1i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesWindsweptForest,
        PlacedFeature {
            feature: Feature::Named("trees_windswept_hills".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(3i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(4i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesWindsweptHills,
        PlacedFeature {
            feature: Feature::Named("trees_windswept_hills".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(0i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(1i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TreesWindsweptSavanna,
        PlacedFeature {
            feature: Feature::Named("trees_savanna".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::WeightedList(
                        WeightedListIntProvider {
                            distribution: vec![
                                WeightedEntry {
                                    data: IntProvider::Constant(2i32),
                                    weight: 9i32,
                                },
                                WeightedEntry {
                                    data: IntProvider::Constant(3i32),
                                    weight: 1i32,
                                },
                            ],
                        },
                    )),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::SurfaceWaterDepthFilter(
                    SurfaceWaterDepthFilterPlacementModifier {
                        max_water_depth: 0i32,
                    },
                ),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloor,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::TwistingVines,
        PlacedFeature {
            feature: Feature::Named("twisting_vines".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::UnderwaterMagma,
        PlacedFeature {
            feature: Feature::Named("underwater_magma".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Object(NormalIntProvider::Uniform(UniformIntProvider {
                        min_inclusive: 44i32,
                        max_inclusive: 52i32,
                    })),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 256i16 }),
                    }),
                }),
                PlacementModifier::SurfaceRelativeThresholdFilter(
                    SurfaceThresholdFilterPlacementModifier {
                        heightmap: HeightMap::OceanFloorWg,
                        min_inclusive: None,
                        max_inclusive: Some(-2i32),
                    },
                ),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::Vines,
        PlacedFeature {
            feature: Feature::Named("vines".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(127i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::Absolute(Absolute { absolute: 64i16 }),
                        max_inclusive: YOffset::Absolute(Absolute { absolute: 100i16 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::VoidStartPlatform,
        PlacedFeature {
            feature: Feature::Named("void_start_platform".to_string()),
            placement: vec![PlacementModifier::Biome(BiomePlacementModifier)],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::WarmOceanVegetation,
        PlacedFeature {
            feature: Feature::Named("warm_ocean_vegetation".to_string()),
            placement: vec![
                PlacementModifier::NoiseBasedCount(NoiseBasedCountPlacementModifier {
                    noise_to_count_ratio: 20i32,
                    noise_factor: 400f64,
                    noise_offset: 0f64,
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::OceanFloorWg,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::WarpedForestVegetation,
        PlacedFeature {
            feature: Feature::Named("warped_forest_vegetation".to_string()),
            placement: vec![
                PlacementModifier::CountOnEveryLayer(CountOnEveryLayerPlacementModifier {
                    count: IntProvider::Constant(5i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::WarpedFungi,
        PlacedFeature {
            feature: Feature::Named("warped_fungus".to_string()),
            placement: vec![
                PlacementModifier::CountOnEveryLayer(CountOnEveryLayerPlacementModifier {
                    count: IntProvider::Constant(8i32),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::WeepingVines,
        PlacedFeature {
            feature: Feature::Named("weeping_vines".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(10i32),
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::HeightRange(HeightRangePlacementModifier {
                    height: HeightProvider::Uniform(UniformHeightProvider {
                        min_inclusive: YOffset::AboveBottom(AboveBottom { above_bottom: 0i8 }),
                        max_inclusive: YOffset::BelowTop(BelowTop { below_top: 0i8 }),
                    }),
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::WildflowersBirchForest,
        PlacedFeature {
            feature: Feature::Named("wildflower".to_string()),
            placement: vec![
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(3i32),
                }),
                PlacementModifier::RarityFilter(RarityFilterPlacementModifier { chance: 2u32 }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(64i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -6i32,
                            max_inclusive: 6i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -2i32,
                            max_inclusive: 2i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map.insert(
        pumpkin_data::placed_feature::PlacedFeature::WildflowersMeadow,
        PlacedFeature {
            feature: Feature::Named("wildflower".to_string()),
            placement: vec![
                PlacementModifier::NoiseThresholdCount(NoiseThresholdCountPlacementModifier {
                    noise_level: -0.8f64,
                    below_noise: 5i32,
                    above_noise: 10i32,
                }),
                PlacementModifier::InSquare(SquarePlacementModifier),
                PlacementModifier::Heightmap(HeightmapPlacementModifier {
                    heightmap: HeightMap::MotionBlocking,
                }),
                PlacementModifier::Biome(BiomePlacementModifier),
                PlacementModifier::Count(CountPlacementModifier {
                    count: IntProvider::Constant(8i32),
                }),
                PlacementModifier::RandomOffset(RandomOffsetPlacementModifier {
                    xz_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -6i32,
                            max_inclusive: 6i32,
                            plateau: 0i32,
                        },
                    )),
                    y_spread: IntProvider::Object(NormalIntProvider::Trapezoid(
                        TrapezoidIntProvider {
                            min_inclusive: -2i32,
                            max_inclusive: 2i32,
                            plateau: 0i32,
                        },
                    )),
                }),
                PlacementModifier::BlockPredicateFilter(BlockFilterPlacementModifier {
                    predicate: BlockPredicate::MatchingBlockTag(MatchingBlockTagPredicate {
                        offset: OffsetBlocksBlockPredicate { offset: None },
                        tag: pumpkin_data::tag::Block::MINECRAFT_AIR,
                    }),
                }),
            ],
        },
    );
    map
}
