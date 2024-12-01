use crate::{
    error::Rs,
    mevx::ChatingWithMevx,
    msg::{MevxMessage, Order},
    MEVX,
};
use grammers_client::Client;

pub trait Goni {
    async fn make_a_buy_order(&self, token: String, amount: f64) -> Rs<()>;
    async fn make_a_sell_order(&self, token: String, amount: f64) -> Rs<()>;
}

impl Goni for Client {
    async fn make_a_buy_order(&self, token: String, amount: f64) -> Rs<()> {
        let (popup, msg) = self.paste_token_address(&token).await?;

        let msg_id = msg.id();

        self.press_a_callback_btn(msg_id, &popup.buy).await?;
        self.press_a_callback_btn(msg_id, &popup.swap).await?;

        let buy_x_btn = msg.get_typing_amount_btn(Order::Buy).await?;

        self.type_amount(msg_id, buy_x_btn, amount).await?;
        self.delete_messages(MEVX, &[msg_id]).await?;

        Ok(())
    }

    async fn make_a_sell_order(&self, token: String, amount: f64) -> Rs<()> {
        let (popup, msg) = self.paste_token_address(&token).await?;

        let msg_id = msg.id();

        self.press_a_callback_btn(msg_id, &popup.sell).await?;
        self.press_a_callback_btn(msg_id, &popup.swap).await?;

        let sell_x_btn = msg.get_typing_amount_btn(Order::Sell).await?;

        self.type_amount(msg_id, sell_x_btn, amount).await?;
        self.delete_messages(MEVX, &[msg_id]).await?;

        Ok(())
    }
}
