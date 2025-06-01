use godot::prelude::*;

struct MiniFantasySurvival;

mod components;
mod enemy;
mod managers;
mod player;

#[gdextension]
unsafe impl ExtensionLibrary for MiniFantasySurvival {}

/*

https://pop-shop-packs.itch.io/cats-pixel-asset-pack

*/
