use chrono::{DateTime, Utc};
use crate::command::Command;
use crate::pack::Type;
use crate::utils;


pub struct Pipeline<'a> {
    command: &'a Command,
    commands: Vec<String>
}

impl<'a> Pipeline<'a> {
    pub fn new(command: &'a Command) -> Self {
        Self { command, commands: vec![] }
    }

    pub fn extract(&mut self, stream_id: &str, from: DateTime<Utc>, to: DateTime<Utc>) -> &mut Self {
        let command = format!("EXTRACT '{}' {} {}", stream_id, utils::rfc3339(from), utils::rfc3339(to));
        self.commands.push(command);
        self
    }

    pub fn format(&mut self, mime_type: &str, sample_rate: u32, channels: u16, bit_depth: u16) -> &mut Self {
        let command = format!("FORMAT '{}' {} {} {}", mime_type, sample_rate, channels, bit_depth);
        self.commands.push(command);
        self
    }

    pub fn create(&mut self, stream_id: &str, sample_rate: u32, channels: u16, bit_depth: u16) -> &mut Self {
        let command = format!("CREATE '{}' {} {} {}", stream_id, sample_rate, channels, bit_depth);
        self.commands.push(command);
        self
    }

    pub fn info(&mut self, stream_id: &str, attr: &str) -> &mut Self {
        let command = format!("INFO '{}' '{}'", stream_id, attr);
        self.commands.push(command);
        self
    }

    pub fn search(&mut self, pattern: &str) -> &mut Self {
        let command = format!("SEARCH '{}'", pattern);
        self.commands.push(command);
        self
    }

    pub fn open(&mut self, stream_id: &str) -> &mut Self {
        let command = format!("OPEN '{}'", stream_id);
        self.commands.push(command);
        self
    }

    pub fn close(&mut self, stream_id: &str) -> &mut Self {
        let command = format!("CLOSE '{}'", stream_id);
        self.commands.push(command);
        self
    }

    pub fn eval(&mut self, expr: &str) -> &mut Self {
        let command = format!("EVAL '{}'", expr);
        self.commands.push(command);
        self
    }

    pub fn ping(&mut self) -> &mut Self {
        self.commands.push("PING".to_string());
        self
    }

    pub fn shutdown(&mut self) -> &mut Self {
        self.commands.push("SHUTDOWN".to_string());
        self
    }

    pub fn execute(&mut self) -> Result<Type, String> {
        let command = self.commands.join(" |> ");
        println!("{}", command);
        self.command.execute_command(command.as_str())
    }

    pub fn reset(&mut self) {
        self.commands.clear();
    }

}

