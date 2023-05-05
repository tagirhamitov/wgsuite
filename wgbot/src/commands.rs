use std::path::PathBuf;

use teloxide::prelude::*;

use crate::check_admin;

pub struct CommandProcessor {
    bot: Bot,
    msg: Message,
    device: String,
    config_path: PathBuf,
    admin_id: ChatId,
}

impl CommandProcessor {
    pub fn new(
        bot: Bot,
        msg: Message,
        device: String,
        config_path: PathBuf,
        admin_id: ChatId,
    ) -> Self {
        Self {
            bot,
            msg,
            device,
            config_path,
            admin_id,
        }
    }

    pub async fn up(&self) -> ResponseResult<()> {
        check_admin!(self);
        match wglib::actions::up(&self.device, &self.config_path) {
            Ok(()) => {
                self.bot
                    .send_message(self.msg.chat.id, "wg server started")
                    .await?;
            }
            Err(err) => self.report_to_admin(err).await?,
        }
        Ok(())
    }

    pub async fn down(&self) -> ResponseResult<()> {
        check_admin!(self);
        match wglib::actions::down(&self.device) {
            Ok(()) => {
                self.bot
                    .send_message(self.msg.chat.id, "wg server stopped")
                    .await?;
            }
            Err(err) => self.report_to_admin(err).await?,
        }
        Ok(())
    }

    pub async fn reboot(&self) -> ResponseResult<()> {
        check_admin!(self);
        match wglib::actions::reboot(&self.device, &self.config_path) {
            Ok(()) => {
                self.bot
                    .send_message(self.msg.chat.id, "wg server restarted")
                    .await?;
            }
            Err(err) => self.report_to_admin(err).await?,
        }
        Ok(())
    }

    pub async fn add_client(&self, name: String) -> ResponseResult<()> {
        match wglib::actions::add_client(&self.device, &self.config_path, name) {
            Ok(id) => {
                self.bot
                    .send_message(self.admin_id, format!("added client with id: {}", id))
                    .await?;
            }
            Err(err) => self.report_to_admin(err).await?,
        }
        Ok(())
    }

    pub async fn remove_client(&self, id: usize) -> ResponseResult<()> {
        check_admin!(self);
        match wglib::actions::remove_client(&self.device, &self.config_path, id) {
            Ok(()) => {
                self.bot
                    .send_message(self.admin_id, format!("removed client with id: {}", id))
                    .await?;
            }
            Err(err) => self.report_to_admin(err).await?,
        }
        Ok(())
    }

    pub async fn list_clients(&self) -> ResponseResult<()> {
        check_admin!(self);
        match wglib::actions::list_clients(&self.config_path) {
            Ok(clients) => {
                let clients: Vec<String> = clients
                    .iter()
                    .map(|client| format!("{}: {}", client.id, client.name))
                    .collect();
                let text = clients.join("\n");
                self.bot.send_message(self.msg.chat.id, text).await?;
            }
            Err(err) => self.report_to_admin(err).await?,
        }
        Ok(())
    }

    async fn report_to_admin(&self, err: anyhow::Error) -> ResponseResult<()> {
        let username = self.msg.chat.username().unwrap_or("-");
        self.bot
            .send_message(
                self.admin_id,
                format!(
                    "chat_id: {}, username: {}, error: |{}|",
                    self.msg.chat.id, username, err
                ),
            )
            .await?;
        Ok(())
    }
}
