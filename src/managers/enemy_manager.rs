use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct EnemyManager {
    base: Base<Node2D>,
    enemies_container: OnReady<Gd<Node2D>>,
    drops_container: OnReady<Gd<Node2D>>,
    #[export]
    enemy_scene: Option<Gd<PackedScene>>,
    #[export]
    drop_scene: Option<Gd<PackedScene>>,
}

#[godot_api]
impl INode2D for EnemyManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            enemies_container: OnReady::from_node("Enemies"),
            drops_container: OnReady::from_node("Drops"),
            enemy_scene: None,
            drop_scene: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("Enemy manager ready!");
    }

    fn process(&mut self, _delta: f64) {
        // godot_print!("Enemy manager processing!");
    }
}

#[godot_api]
impl EnemyManager {
    #[func]
    pub fn spawn_enemy(&mut self) {}

    #[func]
    pub fn spawn_drop(&mut self, global_position: Vector2, value: f32) {
        let spawned_scene = match self.drop_scene.as_mut() {
            Some(drop_scene) => {
                let new_scene = drop_scene.instantiate();
                if let Some(new_scene) = new_scene {
                    Some(new_scene)
                } else {
                    godot_print!("Failed to instantiate drop scene!");
                    return;
                }
            }
            None => None,
        };

        if let Some(mut spawned_scene) = spawned_scene {
            self.drops_container.add_child(&spawned_scene);
            spawned_scene.set("value", &Variant::from(value));
            spawned_scene.set("global_position", &Variant::from(global_position));
        } else {
            godot_print!("Failed to instantiate drop scene!");
        }
    }
}
