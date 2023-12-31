use serde::{Deserialize, Serialize};

//single prompt in "prompts" field for query
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Prompt {
    id: u32,
    code: String,
    prompt: String,
    promptDescription: String,
    promptType: String,
    defaultValue: String,
    isPromptUser: bool,
    sortOrder: u32,
}