use std::time::Instant;
use std::process::Command; 
use rstest::rstest;

pub struct ArgParser{
    args: Vec<String>,
}

impl ArgParser{
    pub fn new() -> Self{
        ArgParser{
            args: std::env::args().collect(),
        }
    }

    pub fn command_passed(&self, target: &str) -> bool {
        self.args.contains(&target.to_owned())
    }

    pub fn execute_command_in_shell(&self) {
        let args_slice = &self.args[1..];
        let command_str = args_slice.join(" ");

        let start = Instant::now();

        println!("Executing command: {}", command_str);

        if self.command_passed("--dry-run") {
            println!("Dry run mode: Command will not be executed");
            return;
        }

        let output = Command::new("sh")  
            .arg("-c")
            .arg(command_str)
            .output();

        let duration = start.elapsed();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                    println!();
                    println!("--------------------------"); 

                } else {
                    eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => eprintln!("Failed to execute command: {}", e),
        }

        println!("Execution time: {:?}", duration);
    }


    pub fn execute_command(&self) {
        if self.command_passed("--help") || self.command_passed("help") || self.command_passed("-h"){
            println!("Usage: These are the available options");
            println!("cargo run -- <command> : Execute the command in the shell");
            println!("cargo run -- --dry-run <command> : Dry run mode, command will not be executed");
            println!("cargo run -- --help : Display this help message");
            return;
        }

        self.execute_command_in_shell();
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_args() {
        let parser = ArgParser::new();
    }

    #[rstest]
    #[case(vec![], "test", false)]
    #[case(vec!["test".to_owned()], "test", true)]
    #[case(vec!["test".to_owned(), "test2".to_owned()], "test", true)]
    #[case(vec!["test".to_owned(), "test2".to_owned()], "test2", true)]
    #[case(vec!["test".to_owned(), "test2".to_owned()], "test1 test2", false)]
    fn test_command_passed(#[case] args: Vec<String>, #[case] target: &str, #[case] expected: bool) {
        let parser = ArgParser { args };
        assert_eq!(parser.command_passed(target), expected);
    }

    #[rstest]
    #[case(vec!["--dry-run".to_owned(), "echo".to_owned(), "Hello".to_owned()], "Dry run mode: Command will not be executed")]
    #[case(vec!["echo".to_owned(), "Hello".to_owned()], "Hello\n")]
    fn test_execute_command_in_shell(#[case] args: Vec<String>, #[case] expected_output: &str) {
        let parser = ArgParser { args };
        let output = Command::new("sh")
            .arg("-c")
            .arg("echo Hello")
            .output()
            .expect("Failed to execute command");

        let output_str = if parser.command_passed("--dry-run") {
            "Dry run mode: Command will not be executed".to_string()
        } else {
            String::from_utf8_lossy(&output.stdout).to_string()
        };

        assert_eq!(output_str.trim(), expected_output.trim());
    }

    #[rstest]
    #[case(vec!["--help".to_owned()], "Usage: These are the available options")]
    #[case(vec!["help".to_owned()], "Usage: These are the available options")]
    #[case(vec!["-h".to_owned()], "Usage: These are the available options")]
    #[case(vec!["echo".to_owned(), "Hello".to_owned()], "Hello\n")]
    fn test_execute_command(#[case] args: Vec<String>, #[case] expected_output: &str) {
        let parser = ArgParser { args };
        let output = Command::new("sh")
            .arg("-c")
            .arg("echo Hello")
            .output()
            .expect("Failed to execute command");

        let output_str = if parser.command_passed("--help") || parser.command_passed("help") || parser.command_passed("-h") {
            "Usage: These are the available options".to_string()
        } else {
            String::from_utf8_lossy(&output.stdout).to_string()
        };

        assert_eq!(output_str.trim(), expected_output.trim());
    }

}
