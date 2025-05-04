use godot::classes::{IStaticBody2D, Sprite2D, StaticBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
struct Enemy {
    base: Base<StaticBody2D>,
    sprite: OnReady<Gd<Sprite2D>>,
}

#[godot_api]
impl IStaticBody2D for Enemy {
    fn init(base: Base<StaticBody2D>) -> Self {
        Self {
            base,
            sprite: OnReady::node("Sprite2D"),
        }
    }
    fn ready(&mut self) {}
}

#[godot_api]
impl Enemy {}

impl Enemy {}
