use godot::classes::{
    AnimatedSprite2D, AnimationTree, CollisionShape2D, IStaticBody2D, StaticBody2D, Timer,
};
use godot::prelude::*;

use crate::components::Health;

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct Enemy {
    base: Base<StaticBody2D>,
    #[export]
    action_state: ActionState,
    health: OnReady<Gd<Health>>,
    #[export]
    speed: f32,
    sprite: OnReady<Gd<AnimatedSprite2D>>,
    animation_tree: OnReady<Gd<AnimationTree>>,
    nav_agent: OnReady<Gd<Node>>,
    player: OnReady<Gd<crate::player::Player>>,
    alive: bool,
}

#[godot_api]
impl IStaticBody2D for Enemy {
    fn init(base: Base<StaticBody2D>) -> Self {
        Self {
            base,
            sprite: OnReady::node("AnimatedSprite2D"),
            health: OnReady::node("Health"),
            speed: 10.0,
            action_state: ActionState::default(),
            animation_tree: OnReady::node("AnimationTree"),
            nav_agent: OnReady::node("NavigationAgent2D"),
            player: OnReady::node("/root/Level/Player"),
            alive: true,
        }
    }

    fn ready(&mut self) {
        // let die = self.base().callable("die");
        // self.health.connect("died", &die);
    }

    fn physics_process(&mut self, delta: f64) {
        if self.alive {
            let target = self.player.get_global_position();
            self.nav_agent
                .set("target_position", &Variant::from(target));
            let target = self
                .nav_agent
                .call("get_next_path_position", &[])
                .to::<Vector2>();

            let direction = (target - self.base().get_global_position()).normalized_or_zero();
            // godot_print!("Direction: {:?}", direction);

            let speed = self.speed;
            let mut base = self.base_mut();
            base.move_and_collide(direction * (delta as f32) * speed);
        }
    }
}

#[godot_api]
impl Enemy {
    #[func]
    pub fn take_damage(&mut self, damage: i64) {
        let died = {
            let mut health = self.health.bind_mut();
            health.take_damage(damage)
        };
        if died {
            self.die();
        }
    }
}

impl Enemy {
    fn die(&mut self) {
        self.alive = false;
        self.action_state = ActionState::Dead;
        self.animation_tree.set(
            "parameters/StateMachine/conditions/death",
            &Variant::from(true),
        );
        godot_print!("Enemy died!");
        self.base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .call_deferred("set_disabled", &[Variant::from(true)]);
        let on_death_timeout = self.base().callable("queue_free");
        match self
            .base()
            .find_child("DeathClearTimer")
            .and_then(|child| child.try_cast::<Timer>().ok())
        {
            Some(mut timer) => {
                timer.connect("timeout", &on_death_timeout);
                timer.start();
            }
            None => godot_print!("No timer found!"),
        }
        self.base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .call(
                "spawn_drop",
                &[
                    Variant::from(self.base().get_global_position()),
                    Variant::from(10.0),
                ],
            );
    }
}

#[derive(GodotConvert, Var, Export, Default, Debug, Clone, Copy)]
#[godot(via = GString)]
pub enum ActionState {
    Idle,
    #[default]
    Walking,
    Dead,
}
