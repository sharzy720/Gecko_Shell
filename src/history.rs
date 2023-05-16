pub struct History {
    commands: Vec<Vec<String>>,
}
impl History {
    pub const fn new() -> Self {
        History {
            commands: Vec::new(),
        }
    }

    /// Adds new command to the tracked history
    ///
    /// # Arguments
    ///
    /// * 'command' - A new command to save to the history
    pub fn add_to_history(&mut self, command: &Vec<String>) {
        self.commands.push(command.clone());
    }

    /// Prints the complete history
    pub fn display_full_history(&self) {
        // Used to display what number a command is in the history
        let mut count: usize = 1;

        for command in &self.commands {
            println!("{} > {:?}", count, &command.join(" "));

            count = &count + 1;
        }
    }

    /// Prints the last n commands in the history
    ///
    /// # Arguments
    ///
    /// * 'num' - The number of commands to display
    pub fn display_num_commands(&self, num: usize) {
        // Number of commands to display
        let mut num_commands: usize = num;

        // Check if the received value is to large
        if num > self.commands.len() {
            num_commands = self.commands.len();
        }

        // Index of the commands to be printed
        let mut current_index: usize = self.commands.len() - num_commands;

        // Number of commands that have been printed
        let mut count: usize = 0;

        // Display commands
        while count != num_commands {
            println!(
                "{} > {:?}",
                current_index,
                &self.commands[current_index].join(" ")
            );

            current_index = &current_index + 1;
            count = &count + 1;
        }
    }
}
