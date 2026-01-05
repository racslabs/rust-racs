use crate::command::Command;
use crate::pack::Type;

pub struct Pipeline<'a> {
    command: &'a Command,
    commands: Vec<String>
}

impl<'a> Pipeline<'a> {
    pub fn new(command: &'a Command) -> Self {
        Self { command, commands: vec![] }
    }

    pub fn range(&mut self, stream_id: &str, start: f32, duration: f32) -> &mut Self {
        let command = format!("RANGE '{}' {:.6} {:.6}", stream_id, start, duration);
        self.commands.push(command);
        self
    }

    pub fn encode(&mut self, mime_type: &str) -> &mut Self {
        let command = format!("ENCODE '{}'", mime_type);
        self.commands.push(command);
        self
    }

    pub fn create(&mut self, stream_id: &str, sample_rate: u32, channels: u16, bit_depth: u16) -> &mut Self {
        let command = format!("CREATE '{}' {} {} {}", stream_id, sample_rate, channels, bit_depth);
        self.commands.push(command);
        self
    }

    pub fn meta(&mut self, stream_id: &str, attr: &str) -> &mut Self {
        let command = format!("META '{}' '{}'", stream_id, attr);
        self.commands.push(command);
        self
    }

    pub fn list(&mut self, pattern: &str) -> &mut Self {
        let command = format!("LIST '{}'", pattern);
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

