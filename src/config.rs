use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub regex_pairs: Vec<RegexPair>,
    pub commands: Vec<SlashCommand>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RegexPair {
    pub regex: String,
    pub response: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SlashCommand {
    pub name: String,
    pub sub_commands: Vec<SubCommand>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SubCommand {
    pub name: String,
    pub response: String,
}
