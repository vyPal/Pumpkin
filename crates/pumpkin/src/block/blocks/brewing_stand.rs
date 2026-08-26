use std::sync::Arc;

use crate::block::{GetComparatorOutputArgs, PlacedArgs};
use crate::block::{
    registry::BlockActionResult,
    {BlockBehaviour, NormalUseArgs},
};

use crate::block::entities::brewing_stand::BrewingStandBlockEntity;
use pumpkin_data::translation;
use pumpkin_inventory::player::player_inventory::PlayerInventory;
use pumpkin_inventory::screen_handler::{BoxFuture, ScreenHandlerFactory, SharedScreenHandler};
use pumpkin_macros::pumpkin_block;
use pumpkin_util::text::TextComponent;
use pumpkin_world::inventory::Inventory;

struct BrewingScreenFactory(
    Arc<dyn Inventory>,
    Arc<dyn crate::block::entities::PropertyDelegate>,
);

impl ScreenHandlerFactory for BrewingScreenFactory {
    fn create_screen_handler<'a>(
        &'a self,
        sync_id: u8,
        player_inventory: &'a Arc<PlayerInventory>,
        _player: &'a dyn pumpkin_inventory::screen_handler::InventoryPlayer,
    ) -> BoxFuture<'a, Option<SharedScreenHandler>> {
        let inventory = self.0.clone();
        let property_delegate = self.1.clone();
        let pi = player_inventory.clone();
        Box::pin(async move {
            // Delegate to pumpkin-inventory brewing creator
            pumpkin_inventory::brewing::create_brewing(sync_id, pi, inventory, property_delegate)
                .await
                .map(|handler| Arc::new(tokio::sync::Mutex::new(handler)) as SharedScreenHandler)
        })
    }

    fn get_display_name(&self) -> TextComponent {
        pumpkin_macros::translate_cross!(
            translation::java::CONTAINER_BREWING,
            translation::bedrock::CONTAINER_BREWING
        )
    }
}

#[pumpkin_block("minecraft:brewing_stand")]
pub struct BrewingStandBlock;

impl BlockBehaviour for BrewingStandBlock {
    fn normal_use(&self, args: NormalUseArgs<'_>) -> BlockActionResult {
        if let Some(block_entity) = args.world.get_block_entity(args.position)
            && let Some(inventory) = block_entity.clone().get_inventory()
            && let Some(pd) = block_entity.clone().to_property_delegate()
        {
            args.player.increment_stat(
                pumpkin_data::statistic::StatisticCategory::Custom,
                pumpkin_data::statistic::CustomStatistic::InteractWithBrewingstand as i32,
                1,
            );
            let player = Arc::clone(args.player);
            let pos = *args.position;
            tokio::spawn(async move {
                player
                    .open_handled_screen(&BrewingScreenFactory(inventory, pd), Some(pos))
                    .await;
            });
        }

        BlockActionResult::Success
    }

    fn placed(&self, args: PlacedArgs<'_>) {
        let be = BrewingStandBlockEntity::new(*args.position);
        args.world.add_block_entity(Arc::new(be));
    }

    fn get_comparator_output(&self, args: GetComparatorOutputArgs<'_>) -> Option<u8> {
        if let Some(block_entity) = args.world.get_block_entity(args.position)
            && let Some(inventory) = block_entity.get_inventory()
        {
            let mut bottles = 0u8;
            // Bottle slots are 0, 1, 2 in brewing stands
            for slot in 0..3 {
                let stack = futures::executor::block_on(inventory.get_stack(slot));
                if !stack.is_empty() {
                    bottles += 1;
                }
            }
            Some(bottles)
        } else {
            None
        }
    }
}
