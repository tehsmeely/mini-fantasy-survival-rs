use godot::classes::{
    AnimatedSprite2D, AnimationTree, Area2D, CharacterBody2D, CollisionPolygon2D, IArea2D,
    ICharacterBody2D,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>,
    facing: Facing8,
    sprite: OnReady<Gd<AnimatedSprite2D>>,
    animation_tree: OnReady<Gd<AnimationTree>>,
    attack_state: AttackState,
    attack_area: OnReady<Gd<AttackArea>>,
    #[export]
    debug: bool,
}

#[derive(Clone, Copy, Default, Debug)]
enum AttackState {
    Attacking(Facing8),
    #[default]
    NotAttacking,
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
            attack_area: OnReady::node("AttackArea"),
            debug: false,
        }
    }

    fn ready(&mut self) {
        // Init AnimatedSprite2D
        let on_animation_finished = self.base_mut().callable("on_sprite_animation_finished");

        self.sprite
            .connect("animation_finished", &on_animation_finished);

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
        let input = Input::singleton();
        let speed = 200.0;
        let mut velocity = Vector2::ZERO;

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

        self.update_facing(velocity);

        let animation_name = {
            let base = if velocity == Vector2::ZERO {
                "idle"
            } else {
                "walking"
            };
            // TODO(jonty): Mildly sad about hot-loop string allocation/concat here.
            format!("{}_{}", base, self.facing.to_animation_name())
        };
        let not_attacking = matches!(self.attack_state, AttackState::NotAttacking);
        if not_attacking {
            // self.sprite.set_animation(animation_name.as_str());

            if input.is_action_just_pressed("player_action") {
                self.start_attack();
            }
        }

        self.animation_tree.set(
            "parameters/MainSM/Idle/blend_position",
            &Variant::from(velocity.normalized_or_zero()),
        );
        self.animation_tree.set(
            "parameters/MainSM/Running/blend_position",
            &Variant::from(velocity.normalized_or_zero()),
        );
        self.animation_tree.set(
            "parameters/MainSM/Walking/blend_position",
            &Variant::from(velocity.normalized_or_zero()),
        );

        let mut base = self.base_mut();
        base.set_velocity(velocity.normalized_or_zero() * speed);
        base.move_and_slide();
    }
}

#[godot_api]
impl Player {
    #[func]
    fn on_sprite_animation_finished(&mut self) {
        godot_print!("Animation finished.");
        self.stop_attack();
    }
}

impl Player {
    fn update_facing(&mut self, velocity: Vector2) {
        if velocity != Vector2::ZERO {
            self.facing = Facing8::from_vector(velocity);
        }
    }

    fn start_attack(&mut self) {
        let attack_animation = format!("attack_{}", self.facing.to_facing4().to_animation_name());
        // self.sprite.set_animation(attack_animation.as_str());
        self.attack_area.bind_mut().enable(self.facing);
        self.attack_state = AttackState::Attacking(self.facing);
    }

    fn stop_attack(&mut self) {
        self.attack_state = AttackState::NotAttacking;
        self.attack_area.bind_mut().disable();
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

#[derive(GodotClass)]
#[class(base=Area2D)]
struct AttackArea {
    base: Base<Area2D>,
    segment: OnReady<Gd<CollisionPolygon2D>>,
}

#[godot_api]
impl IArea2D for AttackArea {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            segment: OnReady::node("AttackSegment"),
        }
    }
    fn ready(&mut self) {
        self.segment.set_disabled(true);
        let on_area_entered = self.base_mut().callable("on_area_entered");
        self.base_mut().connect("body_entered", &on_area_entered);
    }
}

#[godot_api]
impl AttackArea {
    #[func]
    fn on_area_entered(&mut self, body: Gd<Node2D>) {
        godot_print!("Entered attack area. {:?}", body);
    }
}
impl AttackArea {
    fn enable(&mut self, facing: Facing8) {
        self.segment.set_rotation_degrees(facing.to_rotation());
        self.segment.set_disabled(false);
    }

    fn disable(&mut self) {
        self.segment.set_disabled(true);
    }
}
