#![allow(dead_code)]
#![allow(unused_imports)]

use std::sync::Arc;

use async_trait::async_trait;
use pumpkin::{
    plugin::{player::player_join::PlayerJoinEvent, Context, EventHandler, EventPriority},
    server::Server,
};
use pumpkin_api_macros::with_runtime;
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::text::{color::NamedColor, TextComponent};
mod bone;
mod data;
mod utils;

struct MyJoinHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEvent> for MyJoinHandler {
    async fn handle_blocking(&self, _server: &Arc<Server>, event: &mut PlayerJoinEvent) {
        event.join_message =
            TextComponent::text(format!("Welcome, {}!", event.player.gameprofile.name))
                .color_named(NamedColor::Green);
    }
}

#[plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    pumpkin::init_log!();
    log::info!("Hello, Pumpkin");
    Ok(())
}

#[plugin_impl]
pub struct MyPlugin {}

impl MyPlugin {
    pub fn new() -> Self {
        MyPlugin {}
    }
}
impl Default for MyPlugin {
    fn default() -> Self {
        Self::new()
    }
}
