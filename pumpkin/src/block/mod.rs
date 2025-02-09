use blocks::chest::ChestBlock;
use blocks::furnace::FurnaceBlock;
use properties::BlockPropertiesManager;
use pumpkin_data::entity::EntityType;
use pumpkin_data::item::Item;
use pumpkin_util::math::position::BlockPos;
use pumpkin_util::math::vector3::Vector3;
use pumpkin_world::block::registry::Block;
use pumpkin_world::item::ItemStack;
use rand::Rng;

use crate::block::blocks::crafting_table::CraftingTableBlock;
use crate::block::blocks::jukebox::JukeboxBlock;
use crate::block::registry::BlockRegistry;
use crate::entity::item::ItemEntity;
use crate::server::Server;
use crate::world::World;
use std::sync::Arc;

mod blocks;
pub mod properties;
pub mod pumpkin_block;
pub mod registry;

#[must_use]
pub fn default_registry() -> Arc<BlockRegistry> {
    let mut manager = BlockRegistry::default();

    manager.register(JukeboxBlock);
    manager.register(CraftingTableBlock);
    manager.register(FurnaceBlock);
    manager.register(ChestBlock);

    Arc::new(manager)
}

pub async fn drop_loot(server: &Server, world: &Arc<World>, block: &Block, pos: &BlockPos) {
    // TODO: Currently only the item block is droped, We should drop the loop table
    let height = EntityType::ITEM.dimension[1] / 2.0;
    let pos = Vector3::new(
        f64::from(pos.0.x) + 0.5 + rand::thread_rng().gen_range(-0.25..0.25),
        f64::from(pos.0.y) + 0.5 + rand::thread_rng().gen_range(-0.25..0.25) - f64::from(height),
        f64::from(pos.0.z) + 0.5 + rand::thread_rng().gen_range(-0.25..0.25),
    );

    let entity = server.add_entity(pos, EntityType::ITEM, world);
    let item_entity = Arc::new(ItemEntity::new(
        entity,
        &ItemStack::new(1, Item::from_id(block.item_id).unwrap()),
    ));
    world.spawn_entity(item_entity.clone()).await;
    item_entity.send_meta_packet().await;
}

#[must_use]
pub fn default_block_properties_manager() -> Arc<BlockPropertiesManager> {
    let mut manager = BlockPropertiesManager::default();

    manager.build_properties_registry();

    Arc::new(manager)
}
