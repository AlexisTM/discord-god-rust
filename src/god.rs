use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::ai21::{Intellect, AI21};
use std::clone::Clone;
use std::env;
use std::fmt;

// By using a tuple, I can implement Display for Vec<DiscussionKind>
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Discussion(pub Vec<DiscussionKind>);


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GodMemoryConfig {
    pub botname: String,
    pub context: String,
    pub thursdayism: Discussion,
}

#[derive(Clone)]
pub struct AIMemory {
    // Always there, on top of the AI prompt, such as: "This is the discussion between xxx and yyy."
    context: String,
    // We were created last thursday, this is the discussion the bot is born with.
    thursdayism: Discussion,
    // The actual live memory of the bot.
    recollections: Discussion,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DiscussionKind {
    // Splitting them allows to put some different parsing (one extra \n) for responses.
    // Another implementation would have been to use a NewLine type and have only Prompts.
    Prompt { author: String, prompt: String },
    Response { author: String, prompt: String },
}

impl fmt::Display for DiscussionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscussionKind::Prompt { author, prompt } => {
                writeln!(f, "{}: {}", author, prompt)
            }
            DiscussionKind::Response { author, prompt } => {
                writeln!(f, "{}: {}\n\n---", author, prompt)
            }
        }
    }
}

impl fmt::Display for Discussion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for val in &self.0[0..self.0.len()] {
            write!(f, "{}", val)?
        }
        Ok(())
    }
}

impl Discussion {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn push(&mut self, prompt: DiscussionKind) {
        self.0.push(prompt)
    }

    pub fn init(&mut self) {
        self.0 = Vec::new();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// trait Bot, for God
#[derive(Debug)]
pub struct God {
    botname: String,
    pub brain: Box<dyn Intellect + Sync + Send>,
    pub memory: Box<AIMemory>,
}

impl AIMemory {
    pub fn new(context: String, thursdayism: Discussion) -> AIMemory {
        AIMemory {
            context,
            thursdayism,
            recollections: Discussion(Vec::new()),
        }
    }

    pub fn get_prompt(&self, author: &str, prompt: &str, botname: &str) -> String {
        let prompt = DiscussionKind::Prompt {
            author: author.to_string(),
            prompt: prompt.to_string(),
        };
        return format!("{}{}{}:", self, prompt, botname);
    }

    pub fn set_prompt(&mut self, author: &str, prompt: &str) {
        self.recollections.push(DiscussionKind::Prompt {
            author: author.to_string(),
            prompt: prompt.to_string(),
        });
    }

    pub fn set_response(&mut self, author: &str, prompt: &str) {
        self.recollections.push(DiscussionKind::Response {
            author: author.to_string(),
            prompt: prompt.to_string(),
        });
        self.clean();
    }

    pub fn clear(&mut self) {
        self.recollections.clear();
    }

    pub fn clean(&mut self) {
        if self.recollections.len() > 12 {
            self.recollections.0 = self.recollections.0
                [self.recollections.len() - 12..self.recollections.len()]
                .to_vec();
        }
    }

    pub fn clear_interactions(&mut self) {
        self.thursdayism.clear();
        self.recollections.clear();
    }

    pub fn add_interaction(&mut self, author: &str, prompt: &str, botname: &str, response: &str) {
        self.thursdayism.push(DiscussionKind::Prompt {
            author: author.to_string(),
            prompt: prompt.to_string(),
        });
        self.thursdayism.push(DiscussionKind::Response {
            author: botname.to_string(),
            prompt: response.to_string(),
        });
    }
}

impl fmt::Display for AIMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}\n\n---\n\n{}{}",
            self.context, self.thursdayism, self.recollections
        )
    }
}

impl Default for GodMemoryConfig {
    fn default() -> Self {
        let initial_prompt: Discussion = Discussion(vec![
            {
                DiscussionKind::Prompt {
                    author: "Alexis".to_string(),
                    prompt: "Oh! Look there! What is that?".to_string(),
                }
            },
            {
                DiscussionKind::Response {
                    author: "Kirby".to_string(),
                    prompt: "Oh, that is king Dedede! I'm soooo scared!".to_string(),
                }
            },
            {
                DiscussionKind::Prompt {
                    author: "Alexis".to_string(),
                    prompt: "Let's fight this ennemy!".to_string(),
                }
            },
            {
                DiscussionKind::Response {
                    author: "Kirby".to_string(),
                    prompt: "But i have no sword!?!".to_string(),
                }
            },
            {
                DiscussionKind::Prompt {
                    author: "Alexis".to_string(),
                    prompt: "Here, take this minion.".to_string(),
                }
            },
            {
                DiscussionKind::Response {
                    author: "Kirby".to_string(),
                    prompt: "Oof! Thanks for that! I can now fight!".to_string(),
                }
            },
        ]);

        Self { botname: "Kirby".to_string(), context: "Kirby is as one of the most legendary video game characters of all time. In virtually all his appearances, Kirby is depicted as cheerful, innocent and food-loving; however, he becomes fearless, bold and clever in the face of danger.".to_string(),
        thursdayism: initial_prompt }
    }
}

impl God {
    pub fn new(botname: &str) -> God {
        let token_ai21 =
            env::var("GOD_AI21_TOKEN").expect("Expected a token in the environment for AI21");

        let initial_prompt: Discussion = Discussion(
            vec![
                DiscussionKind::Prompt{
                    author: "Alexis".to_string(),
                    prompt: "Who is god?".to_string()
                },
                DiscussionKind::Response{
                    author: "Kirby".to_string(),
                    prompt: "Well, now that you ask, I can tell you. I, God is the great goddess is the god of everybody!".to_string()
                }],
        );
        let memory = AIMemory::new(String::from("God is the god of all beings. Yet, he is the most lovely god and answers in a very complete manner."), initial_prompt);

        God {
            botname: botname.to_string(),
            brain: Box::new(AI21 {
                token: token_ai21,
                stop_sequences: vec!["Kirby:".to_string(), "---".to_string(), "\n".to_string()],
                max_tokens: 250,
                temperature: 0.7,
                top_p: 1.0,
            }),
            memory: Box::new(memory),
        }
    }

    pub fn get_prompt(&self, author: &str, prompt: &str) -> String {
        self.memory.get_prompt(author, prompt, &self.botname)
    }

    pub fn set_prompt_response(&mut self, author: &str, prompt: &str, response: &str) {
        self.memory.set_prompt(author, prompt);
        self.memory.set_response(&self.botname, response)
    }

    pub fn set_context(&mut self, context: &str) {
        self.memory.context = context.to_string();
    }

    pub fn set_botname(&mut self, name: &str) {
        self.botname = name.to_string();
    }

    pub fn get_botname(&self) -> String {
        self.botname.clone()
    }

    pub fn clear(&mut self) {
        self.memory.clear();
    }

    pub fn clear_interactions(&mut self) {
        self.memory.clear_interactions();
    }

    pub fn add_interaction(&mut self, author: &str, prompt: &str, response: &str) {
        self.memory
            .add_interaction(author, prompt, &self.botname, response);
    }

    pub fn export_json(&self) -> serde_json::Value {
        let config = GodMemoryConfig {
            botname: self.botname.clone(),
            context: self.memory.context.clone(),
            thursdayism: self.memory.thursdayism.clone(),
        };
        json!(config)
    }

    pub fn import_json(val: &str) -> Option<Self> {
        if let Ok(config) = serde_json::from_str::<GodMemoryConfig>(val) {
            let mut this = Self::new(config.botname.as_str());
            this.memory.thursdayism = config.thursdayism;
            this.memory.context = config.context;
            Some(this)
        } else {
            None
        }
    }

    pub fn update_from_config(&mut self, config: &GodMemoryConfig) {
        self.botname = config.botname.clone();
        self.memory.thursdayism = config.thursdayism.clone();
        self.memory.context = config.context.clone();
    }

    pub fn from_config(config: &GodMemoryConfig) -> Self {
        let mut this = Self::new(config.botname.as_str());
        this.memory.thursdayism = config.thursdayism.clone();
        this.memory.context = config.context.clone();
        this
    }

    pub fn get_config(&self) -> String {
        format!(
            "{botname} config.
===========
Context:
--------
{context}
Initial memory:
---------------
{memory}
Current memory:
---------------
{current_memory}\n",
            botname = self.botname,
            context = self.memory.context,
            memory = self.memory.thursdayism,
            current_memory = self
                .memory
                .get_prompt("Username", "Some question", &self.botname)
        )
    }
}

impl std::fmt::Debug for Box<AIMemory> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ohoh, that is a Box<AIMemory!>")
    }
}

impl std::fmt::Debug for Box<dyn Intellect + Sync + Send> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ohoh, that is a Box<AIMemory!>")
    }
}
