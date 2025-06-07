use godot::{
    classes::{CanvasLayer, ICanvasLayer, ProgressBar},
    prelude::*,
};

use crate::{components::generic_attribute::GenericAttribute, player::Player};

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Hud {
    base: Base<CanvasLayer>,
    #[export]
    player: OnEditor<Gd<Player>>,
    #[export]
    health_bar: OnEditor<Gd<ProgressBar>>,
    #[export]
    stamina_bar: OnEditor<Gd<ProgressBar>>,
}

#[godot_api]
impl ICanvasLayer for Hud {
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            base,
            player: OnEditor::default(),
            health_bar: OnEditor::default(),
            stamina_bar: OnEditor::default(),
        }
    }

    fn ready(&mut self) {
        godot_print!("HUD is ready.");
    }

    fn process(&mut self, _delta: f64) {
        let p = self.player.bind();
        let health = self
            .player
            .find_child("Health")
            .and_then(|n| n.try_cast::<GenericAttribute>().ok());
        if let Some(h) = health {
            self.health_bar.set_value(h.bind().get_value());
        } else {
            godot_print!("Health node not found in player.");
        }

        let stamina = self
            .player
            .find_child("Stamina")
            .and_then(|n| n.try_cast::<GenericAttribute>().ok());
        if let Some(h) = stamina {
            self.stamina_bar.set_value(h.bind().get_value());
        } else {
            godot_print!("Health node not found in player.");
        }
    }
}
