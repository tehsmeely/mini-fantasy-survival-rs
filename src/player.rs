use godot::classes::{
    AnimatedSprite2D, AnimationTree, Area2D, CharacterBody2D, CollisionPolygon2D, CollisionShape2D,
    IArea2D, ICharacterBody2D,
};
use godot::prelude::*;

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
    attack_state: AttackState,
    attack: OnReady<Gd<Node2D>>,
    attack_pivot: OnReady<Gd<Node2D>>,
    #[export]
    attack_damage: i64,
    #[export]
    movement_state: MovementState,
    #[export]
    debug: bool,
}

#[derive(Clone, Copy, Default, Debug)]
enum AttackState {
    Attacking(Facing8),
    #[default]
    NotAttacking,
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
            sprite: OnReady::node("AnimatedSprite2D"),
            animation_tree: OnReady::node("AnimationTree"),
            attack_state: AttackState::default(),
            //attack_manager: OnReady::node("AttackManager"),
            attack: OnReady::node("AttackPivot/Attack"),
            attack_pivot: OnReady::node("AttackPivot"),
            attack_damage: 50,
            movement_state: MovementState::default(),
            debug: false,
        }
    }

    fn ready(&mut self) {
        let stop_attack = self.base().callable("stop_attack");
        self.attack.connect("attack_finished", &stop_attack);

        let attack_hit_body = self.base().callable("attack_hit_body");
        self.attack.connect("hit_body", &attack_hit_body);

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

        let not_attacking = matches!(self.attack_state, AttackState::NotAttacking);
        if not_attacking {
            // self.sprite.set_animation(animation_name.as_str());

            if let Some(direction) = input.attack {
                self.start_attack(direction);
            }
        }

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
    #[func]
    fn stop_attack(&mut self) {
        self.stop_attack_();
    }
    #[func]
    fn attack_hit_body(&mut self, body: Gd<Node2D>) {
        godot_print!("Hit body: {:?}", body);
        if let Ok(mut enemy) = body.try_cast::<crate::enemy::Enemy>() {
            godot_print!("Hit enemy: {:?}", enemy);
            enemy.bind_mut().take_damage(self.attack_damage);
        }
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
        let attack = match input.is_action_just_pressed("player_action") {
            true => {
                // This assumes player_action is mouse
                let direction = Facing8::from_any_vector(
                    (self.base().get_global_mouse_position() - self.base().get_global_position())
                        .normalized_or_zero(),
                );
                Some(direction)
            }
            false => None,
        };

        let run_held = input.is_action_pressed("player_run");
        InputResult::new(velocity, attack, run_held)
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

    fn start_attack(&mut self, direction: Facing8) {
        self.attack_state = AttackState::Attacking(direction);
        self.attack_pivot
            .set_rotation_degrees(direction.to_rotation());
        self.attack.call("start", &[]);
    }

    fn stop_attack_(&mut self) {
        godot_print!("Stopping attack.");
        self.attack_state = AttackState::NotAttacking;
        self.sprite.play();
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum Facing8 {
    Up,
    Left,
    Right,
    #[default]
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug, Clone, Copy)]
enum Facing4 {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Facing8 {
    fn from_vector(vector: Vector2) -> Self {
        let x = vector.x;
        let y = vector.y;
        // Vector is normalised so we can use the x/y magnitude to imply 100% in that direction.
        match (x, y) {
            (0.0, -1.0) => Self::Up,
            (0.0, 1.0) => Self::Down,
            (1.0, 0.0) => Self::Right,
            (-1.0, 0.0) => Self::Left,
            (x, y) if x > 0.0 && y > 0.0 => Self::DownRight,
            (x, y) if x < 0.0 && y > 0.0 => Self::DownLeft,
            (x, y) if x < 0.0 && y < 0.0 => Self::UpLeft,
            (x, y) if x > 0.0 && y < 0.0 => Self::UpRight,
            _ => Self::default(),
        }
    }

    fn from_any_vector(vector: Vector2) -> Self {
        let angle = vector.y.atan2(vector.x); // angle in radians
        let angle_deg = angle.to_degrees();
        let angle_deg = (angle_deg + 360.0) % 360.0; // normalize to [0, 360)

        // Divide the circle into 8 sectors (each 45 degrees)
        match angle_deg {
            a if a >= 337.5 || a < 22.5 => Facing8::Right,
            a if a >= 22.5 && a < 67.5 => Facing8::DownRight,
            a if a >= 67.5 && a < 112.5 => Facing8::Down,
            a if a >= 112.5 && a < 157.5 => Facing8::DownLeft,
            a if a >= 157.5 && a < 202.5 => Facing8::Left,
            a if a >= 202.5 && a < 247.5 => Facing8::UpLeft,
            a if a >= 247.5 && a < 292.5 => Facing8::Up,
            a if a >= 292.5 && a < 337.5 => Facing8::UpRight,
            _ => unreachable!(),
        }
    }

    fn to_rotation(&self) -> f32 {
        match self {
            Facing8::Up => 0.0,
            Facing8::Down => 180.0,
            Facing8::Left => 270.0,
            Facing8::Right => 90.0,
            Facing8::UpLeft => -45.0,
            Facing8::UpRight => 45.0,
            Facing8::DownRight => 135.0,
            Facing8::DownLeft => 225.0,
        }
    }

    fn to_facing4(&self) -> Facing4 {
        match self {
            Facing8::Up => Facing4::UpLeft,
            Facing8::UpLeft => Facing4::UpLeft,
            Facing8::UpRight => Facing4::UpRight,
            Facing8::Down => Facing4::DownRight,
            Facing8::Left => Facing4::DownLeft,
            Facing8::Right => Facing4::DownRight,
            Facing8::DownRight => Facing4::DownRight,
            Facing8::DownLeft => Facing4::DownLeft,
        }
    }
    fn to_animation_name(&self) -> &'static str {
        match self {
            Facing8::Up => "up",
            Facing8::UpRight => "upright",
            Facing8::Right => "right",
            Facing8::DownRight => "downright",
            Facing8::Down => "down",
            Facing8::DownLeft => "downleft",
            Facing8::Left => "left",
            Facing8::UpLeft => "upleft",
        }
    }
}
impl Facing4 {
    fn to_animation_name(&self) -> &'static str {
        match self {
            Facing4::UpLeft => "ul",
            Facing4::UpRight => "ur",
            Facing4::DownLeft => "dl",
            Facing4::DownRight => "dr",
        }
    }

    fn _to_rotation(&self) -> f32 {
        match self {
            Facing4::UpLeft => -45.0,
            Facing4::UpRight => 45.0,
            Facing4::DownRight => 135.0,
            Facing4::DownLeft => 225.0,
        }
    }
}

struct InputResult {
    velocity: Vector2,
    attack: Option<Facing8>,
    run_held: bool,
}
impl InputResult {
    fn new(velocity: Vector2, attack: Option<Facing8>, run_held: bool) -> Self {
        Self {
            velocity,
            attack,
            run_held,
        }
    }
}
