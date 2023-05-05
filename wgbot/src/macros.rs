#[macro_export]
macro_rules! check_admin {
    ($self:ident) => {
        if $self.msg.chat.id != $self.admin_id {
            $self
                .bot
                .send_message($self.msg.chat.id, "access denied")
                .await?;
            return Ok(());
        }
    };
}
