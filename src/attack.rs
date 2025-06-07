use std::collections::HashSet;

use godot::{
    classes::{AnimatedSprite2D, Area2D},
    prelude::*,
};

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy)]
#[godot(via = i32)]
pub enum CostKind {
    Health,
    Stamina,
    Mana,
}

impl CostKind {
    pub fn to_attribute_name(&self) -> &str {
        match self {
            CostKind::Health => "Health",
            CostKind::Stamina => "Stamina",
            CostKind::Mana => "Mana",
        }
    }
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Attack {
    base: Base<Node2D>,
    enabled: bool,
    #[export]
    name: OnEditor<StringName>,
    #[export]
    animation_name: StringName,
    #[export]
    damage: i32,
    #[export]
    cost_kind: CostKind,
    #[export]
    cost_value: f32,
    seen_bodies: HashSet<InstanceId>,
}

#[godot_api]
impl INode2D for Attack {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            enabled: false,
            name: OnEditor::from_sentinel(StringName::from("Attack")),
            animation_name: StringName::from("default"),
            damage: 50i32,
            cost_kind: CostKind::Mana,
            cost_value: 0.0,
            seen_bodies: HashSet::new(),
        }
    }

    fn ready(&mut self) {
        godot_print!("Attack component ready!");

        let mut hurtbox: Gd<Area2D> = self.base().get_node_as::<Area2D>("Hurtbox");
        let main = self.to_gd();
        hurtbox
            .signals()
            .body_entered()
            .connect_other(&main, Self::on_hurtbox_body_entered);
        hurtbox.set_monitoring(false);
        let mut sprite: Gd<AnimatedSprite2D> = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        sprite
            .signals()
            .animation_finished()
            .connect_other(&main, Self::on_sprite_finished);
        sprite.set_visible(false);
    }
}

#[godot_api]
impl Attack {
    #[signal]
    pub fn hit_body(body: Gd<Node2D>);

    #[signal]
    pub fn attack_finished();

    pub fn start(&mut self) {
        self.enabled = true;
        self.seen_bodies.clear();

        // Enable hurtbox
        let mut hurtbox: Gd<Area2D> = self.base().get_node_as::<Area2D>("Hurtbox");
        hurtbox.set_monitoring(true);

        // Show and start sprite animation
        let mut sprite: Gd<AnimatedSprite2D> = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        sprite.set_visible(true);
        sprite.play_ex().name(&self.animation_name).done();
    }

    pub fn get_cost(&self) -> (CostKind, f32) {
        (self.cost_kind, self.cost_value)
    }

    fn on_hurtbox_body_entered(&mut self, mut body: Gd<Node2D>) {
        if !self.enabled || self.seen_bodies.contains(&body.instance_id()) {
            return;
        }

        // Add body to seen bodies
        self.seen_bodies.insert(body.instance_id());

        // Emit hit body signal
        self.signals().hit_body().emit(&body.clone());

        // Call hit method on the body if it has one
        if body.has_method("take_damage") {
            body.call("take_damage", &[Variant::from(self.damage)]); // Example damage value
        }
    }

    fn on_sprite_finished(&mut self) {
        self.enabled = false;

        // Disable hurtbox
        let mut hurtbox: Gd<Area2D> = self.base().get_node_as::<Area2D>("Hurtbox");
        hurtbox.set_monitoring(false);

        // Hide sprite
        let mut sprite: Gd<AnimatedSprite2D> = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        sprite.set_visible(false);

        // Emit attack finished signal
        self.signals().attack_finished().emit();
    }
}
