use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Timestamp};

#[derive(Clone, Copy)]
pub enum EmbedColor {
    Success,
    Error,
    Warning,
    Info,
    Primary,
    Secondary,
    Custom(u32),
}

impl EmbedColor {
    pub fn value(&self) -> u32 {
        match self {
            EmbedColor::Success => 0x62CB77,   // Light green for success responses
            EmbedColor::Error => 0x853535,     // Dark red for error responses
            EmbedColor::Warning => 0xFFE209,   // Yellow for warning/help responses
            EmbedColor::Info => 0xFFE209,      // Yellow for info responses (same as warning)
            EmbedColor::Primary => 0xC6AC80,   // Beige/tan for general responses
            EmbedColor::Secondary => 0x95A5A6, // Gray (unchanged)
            EmbedColor::Custom(color) => *color,
        }
    }
}

pub struct EmbedBuilder;

impl EmbedBuilder {
    pub fn success(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
        CreateEmbed::new()
            .title(format!("✅ {}", title.into()))
            .description(description)
            .color(EmbedColor::Success.value())
            .timestamp(Timestamp::now())
    }

    pub fn error(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
        CreateEmbed::new()
            .title(format!("❌ {}", title.into()))
            .description(description)
            .color(EmbedColor::Error.value())
            .timestamp(Timestamp::now())
    }

    pub fn warning(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
        CreateEmbed::new()
            .title(format!("⚠️ {}", title.into()))
            .description(description)
            .color(EmbedColor::Warning.value())
            .timestamp(Timestamp::now())
    }

    pub fn info(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
        CreateEmbed::new()
            .title(format!("ℹ️ {}", title.into()))
            .description(description)
            .color(EmbedColor::Info.value())
            .timestamp(Timestamp::now())
    }

    pub fn primary(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
        CreateEmbed::new()
            .title(title)
            .description(description)
            .color(EmbedColor::Primary.value())
            .timestamp(Timestamp::now())
    }

    pub fn custom(
        title: impl Into<String>,
        description: impl Into<String>,
        color: EmbedColor,
    ) -> CreateEmbed {
        CreateEmbed::new()
            .title(title)
            .description(description)
            .color(color.value())
            .timestamp(Timestamp::now())
    }

    pub fn with_author(
        embed: CreateEmbed,
        name: impl Into<String>,
        icon_url: Option<String>,
    ) -> CreateEmbed {
        let mut author = CreateEmbedAuthor::new(name);
        if let Some(url) = icon_url {
            author = author.icon_url(url);
        }
        embed.author(author)
    }

    pub fn with_footer(embed: CreateEmbed, text: impl Into<String>) -> CreateEmbed {
        embed.footer(CreateEmbedFooter::new(text))
    }

    pub fn with_fields(
        mut embed: CreateEmbed,
        fields: Vec<(impl Into<String>, impl Into<String>, bool)>,
    ) -> CreateEmbed {
        for (name, value, inline) in fields {
            embed = embed.field(name, value, inline);
        }
        embed
    }

    pub fn simple_text_to_embed(text: impl Into<String>) -> CreateEmbed {
        let text = text.into();

        if text.starts_with("❌")
            || text.starts_with("Error")
            || text.to_lowercase().contains("error")
        {
            Self::error("Error", text.replace("❌", "").trim())
        } else if text.starts_with("⚠️")
            || text.starts_with("Warning")
            || text.to_lowercase().contains("warning")
        {
            Self::warning("Warning", text.replace("⚠️", "").trim())
        } else if text.starts_with("✅")
            || text.starts_with("Success")
            || text.to_lowercase().contains("success")
        {
            Self::success("Success", text.replace("✅", "").trim())
        } else if text.starts_with("ℹ️") || text.starts_with("Info") {
            Self::info("Information", text.replace("ℹ️", "").trim())
        } else {
            Self::primary("Response", text)
        }
    }
}
