use std::sync::LazyLock;

use anyhow::Context;
use regex::Regex;
use twilight_model::application::command::{CommandOption, CommandOptionChoice, CommandOptionType};
use twilight_model::channel::ChannelType;

const DESCRIPTION_LENGTH: usize = 100;
const NAME_LENGTH: usize = 32;
const CHOICE_COUNT: usize = 25;
const OPTION_COUNT: usize = 25;
const MAX_LENGTH: u16 = 6000;

static NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[-_'\pL\pN\p{sc=Deva}\p{sc=Thai}]{1,32}$")
        .expect("Failed to compile regex pattern")
});

/// Builder to create a [`CommandOption`].
#[derive(Debug, Clone)]
#[must_use = "must be built into a command option"]
pub struct CommandOptionBuilder(CommandOption);

impl CommandOptionBuilder {
    /// Create a new [`CommandOption`] builder.
    pub fn new<S>(name: S, description: S, kind: CommandOptionType) -> Self
    where
        S: Into<String>,
    {
        Self(CommandOption {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description: description.into(),
            description_localizations: None,
            kind,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: name.into(),
            name_localizations: None,
            options: None,
            required: None,
        })
    }

    /// Set whether the option is an autocomplete interaction.
    pub const fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.0.autocomplete = Some(autocomplete);
        self
    }

    /// Set the channel types for the option.
    pub fn channel_types<I>(mut self, channel_types: I) -> Self
    where
        I: IntoIterator<Item = ChannelType>,
    {
        self.0.channel_types = Some(channel_types.into_iter().collect());
        self
    }

    /// Set the choices for the option.
    pub fn choices<I>(mut self, choices: I) -> Self
    where
        I: IntoIterator<Item = CommandOptionChoice>,
    {
        self.0.choices = Some(choices.into_iter().collect());
        self
    }

    /// Set the description localizations for the option.
    pub fn description_localizations<I>(mut self, localizations: I) -> Self
    where
        I: IntoIterator<Item = (String, String)>,
    {
        self.0.description_localizations = Some(localizations.into_iter().collect());
        self
    }

    /// Set the name localizations for the option.
    pub fn name_localizations<I>(mut self, localizations: I) -> Self
    where
        I: IntoIterator<Item = (String, String)>,
    {
        self.0.name_localizations = Some(localizations.into_iter().collect());
        self
    }

    /// Set the options for the option.
    pub fn options<I>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = CommandOption>,
    {
        self.0.options = Some(options.into_iter().collect());
        self
    }

    /// Set the maximum length of the input.
    pub const fn max_length(mut self, max_length: u16) -> Self {
        self.0.max_length = Some(max_length);
        self
    }

    /// Set the minimum length of the input.
    pub const fn min_length(mut self, min_length: u16) -> Self {
        self.0.min_length = Some(min_length);
        self
    }

    /// Set whether the option is required.
    ///
    /// Defaults to `false`.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = Some(required);
        self
    }

    /// Validate the option.
    fn validate(&self) -> anyhow::Result<()> {
        // Ensure the name is not empty
        if self.0.name.is_empty() {
            anyhow::bail!("Name must not be empty");
        }

        // Ensure the name does not exceed the maximum length
        if self.0.name.len() > NAME_LENGTH {
            anyhow::bail!("Name must not exceed 32 characters");
        }

        // Ensure the name matches the regex pattern
        if !NAME_REGEX.is_match(&self.0.name) {
            anyhow::bail!("Name must match the regex pattern: {}", NAME_REGEX.as_str());
        }

        // Ensure the description is not empty
        if self.0.description.is_empty() {
            anyhow::bail!("Description must not be empty");
        }

        // Ensure the description does not exceed the maximum length
        if self.0.description.len() > DESCRIPTION_LENGTH {
            anyhow::bail!("Description must not exceed 100 characters");
        }

        // Ensure 'required' is not set for types that don't support it
        if self.0.required.is_some() {
            match self.0.kind {
                CommandOptionType::SubCommand | CommandOptionType::SubCommandGroup => {
                    anyhow::bail!("'required' is not supported for this option type");
                },
                _ => {},
            }
        }

        if let Some(choices) = &self.0.choices {
            // Ensure 'choices' is not set for types that don't support it
            match self.0.kind {
                CommandOptionType::String
                | CommandOptionType::Integer
                | CommandOptionType::Number => {},
                _ => anyhow::bail!("'choices' is not supported for this option type"),
            }
            // Ensure the number of choices does not exceed the maximum
            if choices.len() > CHOICE_COUNT {
                anyhow::bail!("Option must not have more than {} choices", CHOICE_COUNT);
            }
        }

        if let Some(options) = &self.0.options {
            // Ensure 'options' is not set for types that don't support it
            match self.0.kind {
                CommandOptionType::SubCommand | CommandOptionType::SubCommandGroup => {},
                _ => anyhow::bail!("'options' is not supported for this option type"),
            }
            // Ensure the number of options does not exceed the maximum
            if options.len() > OPTION_COUNT {
                anyhow::bail!("Option must not have more than {} options", OPTION_COUNT);
            }
        }

        // Ensure 'channel_types' is not set for types that don't support it
        if self.0.channel_types.is_some() {
            match self.0.kind {
                CommandOptionType::Channel => {},
                _ => anyhow::bail!("'channel_types' is not supported for this option type"),
            }
        }

        // Ensure 'min_value' is not set for types that don't support it
        if self.0.min_value.is_some() {
            match self.0.kind {
                CommandOptionType::Integer | CommandOptionType::Number => {},
                _ => anyhow::bail!("'min_value' is not supported for this option type"),
            }
        }

        // Ensure 'max_value' is not set for types that don't support it
        if self.0.max_value.is_some() {
            match self.0.kind {
                CommandOptionType::Integer | CommandOptionType::Number => {},
                _ => anyhow::bail!("'max_value' is not supported for this option type"),
            }
        }

        if let Some(max_length) = self.0.max_length {
            // Ensure 'max_length' is not set for types that don't support it
            match self.0.kind {
                CommandOptionType::String => {},
                _ => anyhow::bail!("'max_length' is not supported for this option type"),
            }
            // Ensure the maximum length does not exceed the maximum
            if max_length > MAX_LENGTH {
                anyhow::bail!("Max length must not exceed {} characters", MAX_LENGTH);
            }
        }

        if let Some(min_length) = self.0.min_length {
            // Ensure 'min_length' is not set for types that don't support it
            match self.0.kind {
                CommandOptionType::String => {},
                _ => anyhow::bail!("'min_length' is not supported for this option type"),
            }
            // Ensure the minimum length does not exceed the maximum
            if min_length > MAX_LENGTH {
                anyhow::bail!("Min length must not exceed {} characters", MAX_LENGTH);
            }
        }

        // Ensure 'min_length' is not greater than 'max_length'
        if let (Some(min_length), Some(max_length)) = (self.0.min_length, self.0.max_length) {
            if min_length > max_length {
                anyhow::bail!("Min length must not be greater than max length");
            }
        }

        if self.0.autocomplete.is_some() {
            // Ensure 'autocomplete' is not set for types that don't support it
            match self.0.kind {
                CommandOptionType::String
                | CommandOptionType::Integer
                | CommandOptionType::Number => {},
                _ => anyhow::bail!("'autocomplete' is not supported for this option type"),
            }
            // Ensure 'autocomplete' is not present if 'choices' is set
            if self.0.choices.is_some() {
                anyhow::bail!("'autocomplete' is not supported with 'choices'");
            }
        }

        Ok(())
    }

    /// Consume the builder, returning a [`CommandOption`].
    pub fn build(self) -> anyhow::Result<CommandOption> {
        self.validate().context("validate command option")?;
        Ok(self.0)
    }

    /// Consume the builder, returning a [`CommandOption`] without validation.
    pub fn build_unchecked(self) -> CommandOption {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use twilight_model::application::command::{
        CommandOptionChoice, CommandOptionChoiceValue, CommandOptionType,
    };

    use crate::command_option::CommandOptionBuilder;

    #[test]
    fn command_option() {
        let option =
            CommandOptionBuilder::new("test-name", "Test description", CommandOptionType::String)
                .description_localizations(vec![(
                    "en-US".to_string(),
                    "Test description".to_string(),
                )])
                .name_localizations(vec![("en-US".to_string(), "test-name".to_string())])
                .choices(vec![
                    CommandOptionChoice {
                        name: "Choice 1".to_string(),
                        value: CommandOptionChoiceValue::Integer(1),
                        name_localizations: None,
                    },
                    CommandOptionChoice {
                        name: "Choice 2".to_string(),
                        value: CommandOptionChoiceValue::String("Value".into()),
                        name_localizations: None,
                    },
                ])
                .build()
                .unwrap();

        assert_eq!(option.name, "test-name");
        assert_eq!(option.description, "Test description");
        assert_eq!(option.kind, CommandOptionType::String);
        assert_eq!(option.choices.as_ref().unwrap().len(), 2);
        assert_eq!(option.choices.as_ref().unwrap()[0].name, "Choice 1");
        assert_eq!(
            option.choices.as_ref().unwrap()[0].value,
            CommandOptionChoiceValue::Integer(1)
        );
        assert_eq!(option.choices.as_ref().unwrap()[1].name, "Choice 2");
        assert_eq!(
            option.choices.as_ref().unwrap()[1].value,
            CommandOptionChoiceValue::String("Value".into())
        );
        assert_eq!(option.description_localizations.as_ref().unwrap().len(), 1);
        assert_eq!(
            option.description_localizations.as_ref().unwrap()["en-US"],
            "Test description"
        );
        assert_eq!(option.name_localizations.as_ref().unwrap().len(), 1);
        assert_eq!(
            option.name_localizations.as_ref().unwrap()["en-US"],
            "test-name"
        );
    }
}
