use std::process::Command;

pub fn run_args(args: Vec<String>) {
    let (program, args) = args.split_first().expect("required = true");

    let status = Command::new(program).args(args).status();

    match status {
        Ok(status) => match status.code() {
            Some(0) => {}
            Some(c) => {
                tracing::error!(code = c, "command failed");
            }
            None => {
                tracing::error!("terminated by signal");
            }
        },
        Err(e) => {
            tracing::error!("Failed to execute command: {}", e);
        }
    }
}
