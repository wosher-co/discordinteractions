#[cfg(feature = "serde_support")]
use serde::{Serialize, Serializer};
#[cfg(feature = "serde_support")]
use serde_json::Result as JsonResult;
use std::collections::HashMap;

/// Represents an option for a slash command in Discord.
///
/// Each option provides a different kind of argument for the command, such as a string or a number.
#[cfg_attr(feature = "serde_support", derive(Serialize))]
pub struct DiscordInteractionOption {
    /// Type of option.
    #[cfg_attr(feature = "serde_support", serde(serialize_with = "serialize_discord_interaction_option_type", rename = "type"))]
    pub option_type: DiscordInteractionOptionType,

    /// 1-32 character name.
    pub name: String,

    /// Localization dictionary for the name field.
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub name_localizations: Option<HashMap<String, String>>,

    /// 1-100 character description.
    pub description: String,

    /// Localization dictionary for the description field.
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub description_localizations: Option<HashMap<String, String>>,

    /// If the parameter is required or optional--default false.
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub required: Option<bool>,

    /// Choices for STRING, INTEGER, and NUMBER types for the user to pick from, max 25.
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub choices: Option<Vec<DiscordInteractionChoice>>,

    /// If the option is a subcommand or subcommand group type, these nested options will be the parameters.
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub options: Option<Vec<DiscordInteractionOption>>,

    /// If the option is a channel type, the channels shown will be restricted to these types.
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_discord_interaction_channel_type"))]
    pub channel_types: Option<Vec<DiscordInteractionChannelType>>,

    /// If the option is an INTEGER or NUMBER type, the minimum value permitted.
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub min_value: Option<f64>,

    /// If the option is an INTEGER or NUMBER type, the maximum value permitted.
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub max_value: Option<f64>,

    /// For option type STRING, the minimum allowed length (minimum of 0, maximum of 6000).
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub min_length: Option<u32>,

    /// For option type STRING, the maximum allowed length (minimum of 1, maximum of 6000).
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub max_length: Option<u32>,

    /// If autocomplete interactions are enabled for this STRING, INTEGER, or NUMBER type option.
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub autocomplete: Option<bool>,
}

#[derive(Copy, Clone)]
pub enum DiscordInteractionOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
}

#[cfg(feature = "serde_support")]
fn serialize_discord_interaction_option_type<S>(
    value: &DiscordInteractionOptionType,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u8(*value as u8)
}

#[derive(Copy, Clone)]
pub enum DiscordInteractionChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
}

#[cfg(feature = "serde_support")]
fn serialize_discord_interaction_channel_type<S>(
    value: &Option<Vec<DiscordInteractionChannelType>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.is_none() {
        return serializer.serialize_none();
    }
    
    let mut vec = Vec::new();
    for channel_type in value.as_ref().unwrap() {
        vec.push(*channel_type as u8);
    }
    serializer.serialize_some(&vec)
}

#[cfg_attr(feature = "serde_support", derive(Serialize))]
pub enum InteractionArgument {
    SubCommand(String),
    SubCommandGroup(String),
    String(String),
    Integer(i64),
    Boolean(bool),
    User(String),
    Channel(String),
    Role(String),
    Mentionable(String),
    Number(f64),
}

#[cfg_attr(feature = "serde_support", derive(Serialize))]
pub enum ChoiceValue {
    Str(String),
    Int(i32),
    Float(f64),
}

#[cfg_attr(feature = "serde_support", derive(Serialize))]
pub struct DiscordInteractionChoice {
    pub name: String,
    pub value: ChoiceValue,
}

#[cfg_attr(feature = "serde_support", derive(Serialize))]
pub struct DiscordInteraction {
    pub name: String,
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub name_localizations: Option<HashMap<String, String>>,
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub description_localizations: Option<HashMap<String, String>>,
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub options: Option<Vec<DiscordInteractionOption>>,
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub default_member_permissions: Option<String>,
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none"))]
    pub dm_permission: Option<bool>,
    pub default_permission: bool,
    #[cfg_attr(feature = "serde_support", serde(skip_serializing_if = "Option::is_none", serialize_with="serialize_discord_interaction_type", rename="type"))]
    pub interaction_type: Option<DiscordInteractionType>,
    pub nsfw: bool,
}

#[derive(Copy, Clone)]
pub enum DiscordInteractionType {
    /// Slash commands; a text-based command that shows up when a user types /
    ChatInput = 1,
    /// A UI-based command that shows up when you right click or tap on a user
    User = 2,
    /// A UI-based command that shows up when you right click or tap on a message
    Message = 3,
}

#[cfg(feature = "serde_support")]
fn serialize_discord_interaction_type<S>(
    value: &Option<DiscordInteractionType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.is_none() {
        return serializer.serialize_none();
    }

    serializer.serialize_u8(value.unwrap() as u8)
}


#[cfg(feature = "serde_support")]
pub fn base_command_to_json(base_command: &DiscordInteraction) -> JsonResult<String> {
    serde_json::to_string(base_command)
}
