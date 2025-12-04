use crate::block::registry::BlockActionResult;
use crate::block::{BlockBehaviour, BlockFuture, NormalUseArgs};
use async_trait::async_trait;
use pumpkin_inventory::crafting::crafting_screen_handler::CraftingTableScreenHandler;
use pumpkin_inventory::player::player_inventory::PlayerInventory;
use pumpkin_inventory::screen_handler::{InventoryPlayer, ScreenHandler, ScreenHandlerFactory};
use pumpkin_macros::pumpkin_block;
use pumpkin_util::text::TextComponent;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pumpkin_block("minecraft:crafting_table")]
pub struct CraftingTableBlock;

impl BlockBehaviour for CraftingTableBlock {
    fn normal_use<'a>(&'a self, args: NormalUseArgs<'a>) -> BlockFuture<'a, BlockActionResult> {
        Box::pin(async move {
            args.player
                .open_handled_screen(&CraftingTableScreenFactory)
                .await;

            BlockActionResult::Success
        })
    }
}

struct CraftingTableScreenFactory;

#[async_trait]
impl ScreenHandlerFactory for CraftingTableScreenFactory {
    async fn create_screen_handler(
        &self,
        sync_id: u8,
        player_inventory: &Arc<PlayerInventory>,
        _player: &dyn InventoryPlayer,
    ) -> Option<Arc<Mutex<dyn ScreenHandler>>> {
        Some(Arc::new(Mutex::new(
            CraftingTableScreenHandler::new(sync_id, player_inventory).await,
        )))
    }

    fn get_display_name(&self) -> TextComponent {
        TextComponent::translate("container.crafting", &[])
    }
}
