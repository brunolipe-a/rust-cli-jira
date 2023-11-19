use anyhow::Error;
use cli_jira::ui::Prompts;

fn main() -> Result<(), Error> {
    let prompts = Prompts::new();

    let result = (prompts.create_epic)();

    println!("{:?}", result);

    Ok(())
}
