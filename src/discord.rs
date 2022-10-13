use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize_repr, Serialize_repr, Clone, Debug)]
#[repr(u8)]
pub enum ApplicationCommandOptionType { // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-type
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
    Attachment = 11,
}
#[derive(Deserialize, Serialize)]
pub struct ApplicationCommandOption {// https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub option_type: ApplicationCommandOptionType,
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    pub options: Option<Vec<ApplicationCommandOption>>,
    pub autocomplete: Option<bool>,
    pub required: Option<bool>,
    pub min_value: Option<u64>,
    pub max_value: Option<u64>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub channel_types: Option<Vec<ChannelType>>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ApplicationCommandOptionChoice<T> { // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-choice-structure
    pub name: String,
    pub value: T, 

}

#[derive(Deserialize, Serialize)]
pub struct AutocompleteInteractionCallbackData { //https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-autocomplete
    pub choices: Vec<ApplicationCommandOptionChoice>,
}

#[derive(Deserialize, Serialize)]
pub struct MessagesInteractionCallbackData {//https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-messages
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub components: Option<Vec<Component>>, 
    pub attachment: Option<Vec<Attachment>>
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ChannelType { //https://discord.com/developers/docs/resources/channel#channel-object-channel-types
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncmentThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
}

#[derive(Deserialize, Serialize)]
pub struct Interaction { //https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-structure
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub data: Option<InteractionData>,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub member: Option<Member>,
    pub user: Option<User>,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InteractionData { //combination of ApplicationCommandData, MessageComponentData, ModalSubmitData, and ApplicationCommandInteractionDataOption
    pub id: Option<String>,
    pub name: String,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    #[serde(rename = "type")]
    pub command_type: Option<ApplicationCommandOptionType>,
    pub value: Option<String>, //can be string int double, parse in command handler
    pub focused: Option<bool>, 
}

#[derive(Deserialize_repr, Serialize)]
#[repr(u8)]
pub enum InteractionType { //https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-type
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Deserialize, Serialize)]
pub struct ApplicationCommandData { //https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-application-command-data-structure
    pub id: String,
    pub name: String,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}

#[derive(Deserialize, Serialize)]
pub struct MessageComponentData { //https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-message-component-data-structure

}

#[derive(Deserialize, Serialize)]
pub struct ModalSubmitData { //https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-message-component-data-structure

}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ApplicationCommandInteractionDataOption { //https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-application-command-data-structure
    pub name: String,
    #[serde(rename = "type")]
    pub command_type: ApplicationCommandOptionType,
    pub value: Option<String>, //can be string int double, parse in command handler
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub focused: Option<bool>, 

}

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub bot: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Member {
    pub user: Option<User>,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum EmbedType {
    Rich,
    Image,
    Video,
    Gifv,
    Article,
    Link,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EmbedFooter { //https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure
    pub text: String,
    pub icon_url: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EmbedImage { //https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure
    pub url: String,
    pub height: Option<String>,
    pub width: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EmbedThumbnail { //https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure
    pub url: String,
    pub height: Option<String>,
    pub width: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EmbedField { // https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Embed { // https://discord.com/developers/docs/resources/channel#embed-object
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub embed_type: Option<EmbedType>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u64>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>, 
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Component { 
    
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Attachment {

}

#[derive(Deserialize, Serialize)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub interaction_callback_type: InteractionCallbackType,
    pub data: Option<CallbackData>,
}

#[derive(Deserialize, Serialize)]
pub struct CallbackData { //combine MessagesInteractionCallbackData and AutocompleteInteractionCallbackData
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub components: Option<Vec<Component>>, 
    pub attachment: Option<Vec<Attachment>>
}

impl From<MessagesInteractionCallbackData> for CallbackData {
    fn from(data: MessagesInteractionCallbackData) -> CallbackData {
        CallbackData { 
            choices: None, 
            content: data.content.clone(), 
            embeds: data.embeds.clone(), 
            components: data.components.clone(), 
            attachment: data.attachment.clone(), 
        }
    }
}

impl From<Option<AutocompleteInteractionCallbackData>> for CallbackData {
    fn from(data: Option<AutocompleteInteractionCallbackData>) -> CallbackData {
        CallbackData {
            choices: match data {
                Some(data) => Some(data.choices.clone()),
                None => None,
            },
            content: None,
            embeds: None,
            components: None,
            attachment: None,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum InteractionCallbackType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutocompleteResult = 8,
    Modal = 9,
}


#[derive(Deserialize, Serialize)]
pub struct RegisteredCommand {
    pub name: String,
    pub description: String,
    pub options: Option<Vec<ApplicationCommandOption>>
}
