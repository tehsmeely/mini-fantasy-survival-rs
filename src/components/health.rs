use godot::{classes::AnimationPlayer, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Health {
    base: Base<Node2D>,
    #[export]
    target: Option<Gd<Node2D>>,
    #[export]
    health: i64,
    animation_player: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl INode2D for Health {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            target: None,
            health: 100,
            animation_player: OnReady::node("AnimationPlayer"),
        }
    }

    fn ready(&mut self) {
        godot_print!("Health component ready!");
        let stop_effect = self.base().callable("stop_effect");
        self.animation_player
            .connect("animation_finished", &stop_effect);
    }

    fn process(&mut self, _delta: f64) {
        let scale = self.base().get_scale();
        match self.target.as_mut() {
            Some(target) => {
                target.set_scale(scale);
            }
            None => {
                godot_print!("No target set for health component!");
            }
        }
    }
}

#[godot_api]
impl Health {
    #[func]
    pub fn stop_effect(&mut self, _animation_name: StringName) {
        if let Some(target) = self.target.as_mut() {
            target.set_modulate(Color::WHITE);
        }
    }

    #[signal]
    pub fn died();
}

impl Health {
    pub fn take_damage(&mut self, damage: i64) -> bool {
        self.health -= damage;
        self.animation_player.set_current_animation("hurt");
        self.animation_player.play();

        if let Some(target) = self.target.as_mut() {
            target.set_modulate(Color::RED);
        }

        if self.health <= 0 {
            // Emits this signal, but actually using it in the parent results in re-entrancy issues
            // so try to use the returned dead bool instead
            self.base_mut().emit_signal("died", &[]);
            true
        } else {
            false
        }
    }
}
