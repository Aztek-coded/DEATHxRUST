use std::sync::{Arc, Mutex};
use poise::serenity_prelude::{
    GuildId, ChannelId, UserId,
    Http, Cache,
};
use poise::CreateReply;
use tracing::debug;
use crate::bot::data::{Data, Error};
use crate::testing::response_validator::EmbedData;

#[derive(Clone)]
pub struct MockUser {
    pub id: UserId,
    pub name: String,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub bot: bool,
}

impl MockUser {
    pub fn new(id: u64, name: impl Into<String>) -> Self {
        Self {
            id: UserId::new(id),
            name: name.into(),
            discriminator: None,
            avatar: None,
            bot: false,
        }
    }
}

#[derive(Clone)]
pub struct MockGuild {
    pub id: GuildId,
    pub name: String,
}

impl MockGuild {
    pub fn new(id: u64, name: impl Into<String>) -> Self {
        Self {
            id: GuildId::new(id),
            name: name.into(),
        }
    }
}

pub struct CapturedResponse {
    pub embed_data: Vec<EmbedData>,
    pub content: Option<String>,
    pub ephemeral: bool,
}

pub struct MockContext {
    pub responses: Arc<Mutex<Vec<CapturedResponse>>>,
    pub command_name: String,
    pub author: MockUser,
    pub guild: Option<MockGuild>,
    pub channel_id: ChannelId,
    pub data: Arc<Data>,
    pub http: Arc<Http>,
    pub cache: Arc<Cache>,
}

impl MockContext {
    pub async fn send_reply(&self, reply: CreateReply) -> Result<(), Error> {
        let mut responses = self.responses.lock().unwrap();
        
        // For now, we'll create simple EmbedData from the reply
        // In a real implementation, we'd need to extract the data from CreateReply
        let embed_data = Vec::new(); // Simplified for now
        
        let captured = CapturedResponse {
            embed_data,
            content: reply.content.clone(),
            ephemeral: reply.ephemeral.unwrap_or(false),
        };
        
        responses.push(captured);
        debug!("Mock context captured response for command: {}", self.command_name);
        
        Ok(())
    }
    
    pub async fn send_embed_data(&self, embed_data: EmbedData) -> Result<(), Error> {
        let mut responses = self.responses.lock().unwrap();
        
        let captured = CapturedResponse {
            embed_data: vec![embed_data],
            content: None,
            ephemeral: false,
        };
        
        responses.push(captured);
        debug!("Mock context captured embed data for command: {}", self.command_name);
        
        Ok(())
    }
    
    pub fn get_responses(&self) -> Vec<CapturedResponse> {
        let responses = self.responses.lock().unwrap();
        responses.iter().map(|r| CapturedResponse {
            embed_data: r.embed_data.clone(),
            content: r.content.clone(),
            ephemeral: r.ephemeral,
        }).collect()
    }
    
    pub fn clear_responses(&self) {
        self.responses.lock().unwrap().clear();
    }
}

pub struct MockContextBuilder {
    command_name: String,
    author: Option<MockUser>,
    guild: Option<MockGuild>,
    channel_id: Option<ChannelId>,
    data: Option<Arc<Data>>,
    http: Option<Arc<Http>>,
    cache: Option<Arc<Cache>>,
}

impl MockContextBuilder {
    pub fn new(command_name: impl Into<String>) -> Self {
        Self {
            command_name: command_name.into(),
            author: None,
            guild: None,
            channel_id: None,
            data: None,
            http: None,
            cache: None,
        }
    }
    
    pub fn with_author(mut self, author: MockUser) -> Self {
        self.author = Some(author);
        self
    }
    
    pub fn with_guild(mut self, guild: MockGuild) -> Self {
        self.guild = Some(guild);
        self
    }
    
    pub fn with_channel(mut self, channel_id: u64) -> Self {
        self.channel_id = Some(ChannelId::new(channel_id));
        self
    }
    
    pub fn with_data(mut self, data: Arc<Data>) -> Self {
        self.data = Some(data);
        self
    }
    
    pub fn with_http(mut self, http: Arc<Http>) -> Self {
        self.http = Some(http);
        self
    }
    
    pub fn with_cache(mut self, cache: Arc<Cache>) -> Self {
        self.cache = Some(cache);
        self
    }
    
    pub fn build(self) -> MockContext {
        debug!("Mock context created for command: {}", self.command_name);
        
        MockContext {
            responses: Arc::new(Mutex::new(Vec::new())),
            command_name: self.command_name,
            author: self.author.unwrap_or_else(|| MockUser::new(123456789, "TestUser")),
            guild: self.guild,
            channel_id: self.channel_id.unwrap_or_else(|| ChannelId::new(987654321)),
            data: self.data.expect("Data must be provided"),
            http: self.http.expect("HTTP client must be provided"),
            cache: self.cache.expect("Cache must be provided"),
        }
    }
}