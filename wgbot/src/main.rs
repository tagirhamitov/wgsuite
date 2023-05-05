mod commands;
mod macros;

use clap::Parser;
use commands::CommandProcessor;
use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(Parser)]
pub struct Cli {
    #[arg(long)]
    token: String,
    #[arg(long)]
    device: String,
    #[arg(long)]
    config_path: std::path::PathBuf,
    #[arg(long)]
    admin_id: i64,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Up,
    Down,
    Reboot,
    AddClient { name: String },
    RemoveClient { id: usize },
    ListClients,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let bot = Bot::new(&cli.token);
    Command::repl(bot, |bot: Bot, msg: Message, cmd: Command| async move {
        let cli = Cli::parse();
        let processor =
            CommandProcessor::new(bot, msg, cli.device, cli.config_path, ChatId(cli.admin_id));
        match cmd {
            Command::Up => processor.up().await,
            Command::Down => processor.down().await,
            Command::Reboot => processor.reboot().await,
            Command::AddClient { name } => processor.add_client(name).await,
            Command::RemoveClient { id } => processor.remove_client(id).await,
            Command::ListClients => processor.list_clients().await,
        }
    })
    .await;
}
