# Discord Interactions

A simple library for declaring Discord interactions.

## Serde Support

Add the `serde_support` to your `Cargo.toml` to enable serde support.

```toml
discordinteractions = { version = "1", features = ["serde_support"] }
```

## Example

```rust
DiscordInteraction {
    name: "test".to_string(),
    name_localizations: None,
    description: Some("test".to_string()),
    description_localizations: None,
    options: Some(vec![
        DiscordInteractionOption {
            name: "test".to_string(),
            description: "test".to_string(),
            autocomplete: Some(false),
            required: Some(true),
            channel_types: None,
            choices: None,
            max_length: None,
            min_length: None,
            max_value: None,
            min_value: None,
            description_localizations: None,
            name_localizations: None,
            option_type: command_creator::DiscordInteractionOptionType::String,
            options: None,
        }
    ]),
    default_member_permissions: None,
    dm_permission: None,
    default_permission: true,
    nsfw: false,
};
```

Using reqwest to send the interaction:

```rust
client.post("https://discord.com/api/v10/applications/{app.id}/commands")
    .header("User-Agent", "DiscordBot (url, version)")
    .header("Authorization", format!("Bot {}", BOT_TOKEN))
    .json(&declared_interaction).send();
```
