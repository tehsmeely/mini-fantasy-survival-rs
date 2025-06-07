use godot::{classes::Input, prelude::*};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    attack::{Attack, CostKind},
    components::generic_attribute::GenericAttribute,
    core::Facing8,
    player::Player,
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct AttackManager {
    base: Base<Node2D>,
    attacks: Vec<Gd<Attack>>,
    player: OnReady<Gd<Player>>,
}

#[godot_api]
impl INode2D for AttackManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            attacks: Vec::new(),
            player: OnReady::from_base_fn(|base| base.get_parent().unwrap().cast::<Player>()),
        }
    }

    fn ready(&mut self) {
        godot_print!("Attack manager ready!");
        self.attacks = self
            .base()
            .get_children()
            .iter_shared()
            // TODO jheiser: Cast to some Attack type? once one exists.
            .filter_map(|child| child.try_cast::<Attack>().ok())
            .collect();
    }

    fn process(&mut self, _delta: f64) {
        if let Some(attack_input) = self.handle_input() {
            godot_print!("Attack input: {:?}", attack_input);
            self.handle_attack(attack_input);
        }
    }
}

impl AttackManager {
    fn handle_input(&self) -> Option<AttackInput> {
        let input = Input::singleton();
        for attack_type in AttackType::iter() {
            let input_name = attack_type.to_input_name();
            if input.is_action_just_pressed(input_name) {
                let attack_direction = Facing8::from_any_vector(
                    (self.base().get_global_mouse_position() - self.base().get_global_position())
                        .normalized_or_zero(),
                );
                return Some(AttackInput {
                    attack: attack_type,
                    attack_direction,
                });
            }
        }
        None
    }

    fn can_afford_attack(&self, cost_kind: CostKind, cost_value: f32) -> bool {
        if cost_value == 0.0 {
            return true;
        }
        let attribute = self
            .player
            .find_child(cost_kind.to_attribute_name())
            .and_then(|n| n.try_cast::<GenericAttribute>().ok());

        if let Some(mut attr) = attribute {
            attr.bind_mut().take(cost_value as f64)
        } else {
            godot_print!(
                "Attribute '{}' not found for cost kind {:?}",
                cost_kind.to_attribute_name(),
                cost_kind
            );
            false
        }
    }

    fn handle_attack(&mut self, attack_input: AttackInput) {
        let attack_node = self
            .attacks
            .get(attack_input.attack.to_node_index())
            // Gd<Node2D> is a ref counted pointer, so we can clone it for ~free
            .cloned();
        if let Some(mut attack_node) = attack_node {
            let (cost_kind, cost_value) = attack_node.bind().get_cost();
            if self.can_afford_attack(cost_kind, cost_value) {
                self.base_mut()
                    .set_rotation_degrees(attack_input.attack_direction.to_rotation());
                attack_node.bind_mut().start();
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum AttackType {
    Main,
    Secondary,
    Slot1,
    Slot2,
    Slot3,
    Slot4,
}

impl AttackType {
    fn to_input_name(&self) -> &str {
        match self {
            AttackType::Main => "attack_main",
            AttackType::Secondary => "attack_secondary",
            AttackType::Slot1 => "attack_slot_1",
            AttackType::Slot2 => "attack_slot_2",
            AttackType::Slot3 => "attack_slot_3",
            AttackType::Slot4 => "attack_slot_4",
        }
    }

    /// Index of an array of children that represent attacks
    fn to_node_index(&self) -> usize {
        match self {
            AttackType::Main => 0,
            AttackType::Secondary => 1,
            AttackType::Slot1 => 2,
            AttackType::Slot2 => 3,
            AttackType::Slot3 => 4,
            AttackType::Slot4 => 5,
        }
    }
}

#[derive(Debug, Clone)]
struct AttackInput {
    attack: AttackType,
    attack_direction: Facing8,
}
