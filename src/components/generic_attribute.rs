use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GenericAttribute {
    base: Base<Node2D>,
    #[export]
    attribute_name: OnEditor<StringName>,
    #[export]
    value: f64,
    #[export]
    max_value: f64,
    #[export]
    regen: f64,
}

#[godot_api]
impl INode2D for GenericAttribute {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            attribute_name: OnEditor::from_sentinel(StringName::from("__attribute_name")),
            value: 100.0,
            max_value: 100.0,
            regen: 0.0,
        }
    }

    fn ready(&mut self) {
        godot_print!(
            "GenericAttribute({}) component ready!",
            self.attribute_name.to_string()
        );
    }

    fn process(&mut self, delta: f64) {
        if !self.regen.is_zero_approx() {
            self.change(delta * self.regen);
        }
    }
}

#[godot_api]
impl GenericAttribute {
    pub fn change(&mut self, delta: f64) {
        self.value = (self.value + delta).clamp(0.0, self.max_value);
    }

    pub fn take(&mut self, amount: f64) -> bool {
        if amount > self.value {
            false
        } else {
            self.value -= amount;
            true
        }
    }
}
