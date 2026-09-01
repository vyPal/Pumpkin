use crate::entity::player::Player;
use crate::item::{ItemBehaviour, ItemMetadata};
use crate::server::Server;
use pumpkin_data::BlockDirection;
use pumpkin_data::item::Item;
use pumpkin_data::item_stack::ItemStack;
use pumpkin_data::sound::{Sound, SoundCategory};
use pumpkin_data::{Block, tag};
use pumpkin_util::GameMode;
use pumpkin_util::math::position::BlockPos;
use pumpkin_util::math::vector3::Vector3;
use pumpkin_world::world::BlockFlags;

pub struct HoeItem;

impl ItemMetadata for HoeItem {
    fn ids() -> Box<[u16]> {
        tag::Item::MINECRAFT_HOES.1.into()
    }
}

impl ItemBehaviour for HoeItem {
    fn use_on_block(
        &self,
        item: &mut ItemStack,
        player: &Player,
        location: BlockPos,
        face: BlockDirection,
        _cursor_pos: Vector3<f32>,
        block: &Block,
        _server: &Server,
    ) {
        let world = player.world();

        let only_if_air_above =
            || face != BlockDirection::Down && world.get_block_state(&location.up()).is_air();

        let (future_block, drop_item) = if block == &Block::GRASS_BLOCK
            || block == &Block::DIRT_PATH
            || block == &Block::DIRT
        {
            if only_if_air_above() {
                (Some(&Block::FARMLAND), None)
            } else {
                (None, None)
            }
        } else if block == &Block::COARSE_DIRT {
            if only_if_air_above() {
                (Some(&Block::DIRT), None)
            } else {
                (None, None)
            }
        } else if block == &Block::ROOTED_DIRT {
            (Some(&Block::DIRT), Some(&Item::HANGING_ROOTS))
        } else {
            (None, None)
        };

        if let Some(target_block) = future_block {
            world.play_sound(
                Sound::ItemHoeTill,
                SoundCategory::Blocks,
                &location.to_f64(),
            );

            world.set_block_state(
                &location,
                target_block.default_state.id,
                BlockFlags::NOTIFY_ALL,
            );

            if let Some(drop_item) = drop_item {
                world.drop_stack_from_face(&location, face, ItemStack::new(1, drop_item));
            }

            if player.gamemode.load() != GameMode::Creative {
                // TODO: Handle DamageResult::Broken to broadcast item break and update player slot.
                let _ = item.damage_item(1);
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
