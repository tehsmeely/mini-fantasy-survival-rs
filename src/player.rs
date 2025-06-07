use godot::classes::{
    AnimatedSprite2D, AnimationTree, CharacterBody2D, CollisionShape2D, ICharacterBody2D, Input,
};
use godot::prelude::*;

use crate::core::Facing8;

const MOVEMENT_BLEND_PROPS: [&str; 4] = [
    "parameters/MainSM/Walking/blend_position",
    "parameters/MainSM/Running/blend_position",
    "parameters/MainSM/Sitting Down/blend_position",
    "parameters/MainSM/Idle/blend_position",
];

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    facing: Facing8,
    sprite: OnReady<Gd<AnimatedSprite2D>>,
    animation_tree: OnReady<Gd<AnimationTree>>,
    #[export]
    movement_state: MovementState,
    #[export]
    debug: bool,
    // TODO jheiser: Move health and stamina to a component.
    health: i32,
    stamina: i32,
}

#[derive(GodotConvert, Var, Export, Default, Debug, Clone, Copy)]
#[godot(via = GString)]
pub enum MovementState {
    #[default]
    Idle,
    Walking,
    Running,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            facing: Facing8::default(),
            sprite: OnReady::from_node("AnimatedSprite2D"),
            animation_tree: OnReady::from_node("AnimationTree"),
            // TODO jheiser: Consider mirroring attack state from attack manager?
            // attack_state: AttackState::default(),
            movement_state: MovementState::default(),
            debug: false,
            health: 100,
            stamina: 200,
        }
    }

    fn ready(&mut self) {
        // Init Debug Label
        let label = self
            .base()
            .try_get_node_as::<godot::classes::Label>("DebugLabel");
        match (label, self.debug) {
            (Some(mut label), false) => {
                godot_print!("Player debug display disabled.");
                label.set_visible(false);
            }
            _ => {}
        }
    }

    fn process(&mut self, _delta: f64) {
        let label = self
            .base()
            .try_get_node_as::<godot::classes::Label>("DebugLabel");
        match (label, self.debug) {
            (Some(mut label), true) => {
                label.set_text(format!("{:?}", self.facing).as_str());
            }
            _ => {}
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let input = self.handle_input();

        self.movement_state = if input.velocity != Vector2::ZERO {
            if input.run_held {
                MovementState::Running
            } else {
                MovementState::Walking
            }
        } else {
            MovementState::Idle
        };

        let speed = match self.movement_state {
            MovementState::Idle => 0.0,
            MovementState::Walking => 100.0,
            MovementState::Running => 200.0,
        };

        self.update_facing(input.velocity);
        // TODO: Update collision shape based on facing direction.

        if let Some(velocity) = input.velocity.try_normalized() {
            // Don't update facing if velocity is zero.
            for property in MOVEMENT_BLEND_PROPS {
                self.animation_tree.set(property, &Variant::from(velocity));
            }
        }

        let mut base = self.base_mut();
        base.set_velocity(input.velocity.normalized_or_zero() * speed);
        base.move_and_slide();
    }
}

#[godot_api]
impl Player {
    pub fn get_health(&self) -> i32 {
        self.health
    }
    pub fn get_stamina(&self) -> i32 {
        self.stamina
    }
}

impl Player {
    fn handle_input(&mut self) -> InputResult {
        let mut velocity = Vector2::ZERO;
        let input = Input::singleton();
        if input.is_action_pressed("player_right") {
            velocity.x += 1.0;
        }
        if input.is_action_pressed("player_left") {
            velocity.x -= 1.0;
        }
        if input.is_action_pressed("player_down") {
            velocity.y += 1.0;
        }
        if input.is_action_pressed("player_up") {
            velocity.y -= 1.0;
        }

        let run_held = input.is_action_pressed("player_run");
        InputResult::new(velocity, run_held)
    }
    fn update_facing(&mut self, velocity: Vector2) {
        if velocity != Vector2::ZERO {
            self.facing = Facing8::from_vector(velocity);
            if let Some(mut shape) = self
                .base()
                .try_get_node_as::<CollisionShape2D>("CollisionShape2D")
            {
                shape.set_rotation_degrees(self.facing.to_rotation());
            }
        }
    }
}

struct InputResult {
    velocity: Vector2,
    run_held: bool,
}
impl InputResult {
    fn new(velocity: Vector2, run_held: bool) -> Self {
        Self { velocity, run_held }
    }
}
