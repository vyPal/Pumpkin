use crate::plugin::api::events::entity::entity_damage::EntityDamageEvent;
use crate::plugin::api::events::entity::entity_regain_health::EntityRegainHealthEvent;
use crate::plugin::api::events::entity::player_death::PlayerDeathEvent;
use crate::plugin::loader::wasm::wasm_host::{
    state::PluginHostState,
    wit::v0_1::{
        events::{
            ToFromWasmEvent, consume_entity, consume_item_stack, consume_player,
            consume_text_component, from_wasm_damage_type, from_wasm_heal_reason,
            from_wasm_position, to_wasm_damage_type, to_wasm_heal_reason, to_wasm_position,
        },
        pumpkin::plugin::event::{
            Event, EntityDamageEventData, EntityRegainHealthEventData, PlayerDeathEventData,
        },
    },
};

impl ToFromWasmEvent for EntityDamageEvent {
    fn to_wasm_event(&self, state: &mut PluginHostState) -> Event {
        let victim = state
            .add_entity(self.victim.clone())
            .expect("failed to add entity resource");
        let source = self
            .source
            .clone()
            .map(|s| state.add_entity(s).expect("failed to add entity resource"));
        let cause = self
            .cause
            .clone()
            .map(|c| state.add_entity(c).expect("failed to add entity resource"));

        Event::EntityDamageEvent(EntityDamageEventData {
            victim,
            amount: self.amount,
            damage_type: to_wasm_damage_type(self.damage_type),
            source,
            cause,
            position: self.position.map(to_wasm_position),
            cancelled: self.cancelled,
        })
    }

    fn from_wasm_event(event: Event, state: &mut PluginHostState) -> Self {
        match event {
            Event::EntityDamageEvent(data) => Self {
                victim: consume_entity(state, &data.victim),
                amount: data.amount,
                damage_type: from_wasm_damage_type(&data.damage_type),
                source: data.source.map(|s| consume_entity(state, &s)),
                cause: data.cause.map(|c| consume_entity(state, &c)),
                position: data.position.map(from_wasm_position),
                cancelled: data.cancelled,
            },
            _ => panic!("unexpected event type"),
        }
    }
}

impl ToFromWasmEvent for PlayerDeathEvent {
    fn to_wasm_event(&self, state: &mut PluginHostState) -> Event {
        let player = state
            .add_player(self.player.clone())
            .expect("failed to add player resource");
        let killer = self
            .killer
            .clone()
            .map(|k| state.add_entity(k).expect("failed to add entity resource"));
        let death_message = state
            .add_text_component(self.death_message.clone())
            .expect("failed to add text-component resource");
        let drops = self
            .drops
            .iter()
            .map(|stack| {
                state
                    .add_item_stack(std::sync::Arc::new(tokio::sync::Mutex::new(stack.clone())))
                    .expect("failed to add item-stack resource")
            })
            .collect();

        Event::PlayerDeathEvent(PlayerDeathEventData {
            player,
            killer,
            death_message,
            drops,
        })
    }

    fn from_wasm_event(event: Event, state: &mut PluginHostState) -> Self {
        match event {
            Event::PlayerDeathEvent(data) => Self {
                player: consume_player(state, &data.player),
                killer: data.killer.map(|k| consume_entity(state, &k)),
                death_message: consume_text_component(state, &data.death_message),
                drops: data
                    .drops
                    .iter()
                    .map(|stack| consume_item_stack(state, stack))
                    .collect(),
            },
            _ => panic!("unexpected event type"),
        }
    }
}

impl ToFromWasmEvent for EntityRegainHealthEvent {
    fn to_wasm_event(&self, state: &mut PluginHostState) -> Event {
        let victim = state
            .add_entity(self.victim.clone())
            .expect("failed to add entity resource");

        Event::EntityRegainHealthEvent(EntityRegainHealthEventData {
            victim,
            amount: self.amount,
            reason: to_wasm_heal_reason(self.reason),
            cancelled: self.cancelled,
        })
    }

    fn from_wasm_event(event: Event, state: &mut PluginHostState) -> Self {
        match event {
            Event::EntityRegainHealthEvent(data) => Self {
                victim: consume_entity(state, &data.victim),
                amount: data.amount,
                reason: from_wasm_heal_reason(data.reason),
                cancelled: data.cancelled,
            },
            _ => panic!("unexpected event type"),
        }
    }
}
