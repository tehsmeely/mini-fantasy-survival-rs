use godot::prelude::*;

struct PurranormalDefence;

mod attack;
mod components;
mod core;
mod enemy;
mod hud;
mod managers;
mod player;

#[gdextension]
unsafe impl ExtensionLibrary for PurranormalDefence {}

/*

https://pop-shop-packs.itch.io/cats-pixel-asset-pack

*/
