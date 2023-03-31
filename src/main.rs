#[path = "cli/methods.rs"]
mod methods;

use methods::create_cli_app;

fn main() {
    let matches = create_cli_app();

    methods::cli_command_line(&matches);
}
