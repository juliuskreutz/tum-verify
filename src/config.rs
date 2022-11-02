use std::fs;

use crate::serenity;
use anyhow::Result;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters)]
pub struct Config {
    token: String,
    email: EmailConfig,
}

impl Config {
    pub fn read() -> Result<Self> {
        let config = toml::from_str(&fs::read_to_string("config.toml")?)?;

        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Getters)]
pub struct EmailConfig {
    name: String,
    email: String,
    host: String,
    user: String,
    password: String,
}

#[derive(Serialize, Deserialize, Getters)]
pub struct GuildConfig {
    add: Option<serenity::RoleId>,
    remove: Option<serenity::RoleId>,
}

impl GuildConfig {
    pub fn new(add: Option<serenity::RoleId>, remove: Option<serenity::RoleId>) -> Self {
        Self { add, remove }
    }

    pub fn read(guild_id: serenity::GuildId) -> Result<Self> {
        let config = toml::from_str(&fs::read_to_string(format!("{}.toml", guild_id))?)?;

        Ok(config)
    }

    pub fn write(&self, guild_id: serenity::GuildId) -> Result<()> {
        fs::write(format!("{}.toml", guild_id), toml::to_string(&self)?)?;

        Ok(())
    }
}
