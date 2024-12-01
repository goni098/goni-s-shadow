use crate::{
    error::Rs,
    mevx::ChatingWithMevx,
    msg::{MevxMessage, Order},
};
use grammers_client::{types::Chat, Client};

pub trait Goni {
    async fn make_a_buy_order(&self, mevx: &Chat, token: String, amount: f64) -> Rs<()>;
    async fn make_a_sell_order(&self, mevx: &Chat, token: String, amount: f64) -> Rs<()>;
}

impl Goni for Client {
    async fn make_a_buy_order(&self, mevx: &Chat, token: String, amount: f64) -> Rs<()> {
        let (popup, msg) = self.paste_token(mevx, &token).await?;

        let msg_id = msg.id();

        self.press_a_btn_callback(msg_id, &popup.buy).await?;
        self.press_a_btn_callback(msg_id, &popup.swap).await?;

        let buy_x_btn = msg.get_typing_amount_btn(Order::Buy).await?;

        self.type_amount(mevx, msg_id, buy_x_btn, amount).await?;
        self.delete_messages(mevx, &[msg_id]).await?;

        Ok(())
    }

    async fn make_a_sell_order(&self, mevx: &Chat, token: String, amount: f64) -> Rs<()> {
        let (popup, msg) = self.paste_token(mevx, &token).await?;

        let msg_id = msg.id();

        self.press_a_btn_callback(msg_id, &popup.sell).await?;
        self.press_a_btn_callback(msg_id, &popup.swap).await?;

        let sell_x_btn = msg.get_typing_amount_btn(Order::Sell).await?;

        self.type_amount(mevx, msg_id, sell_x_btn, amount).await?;
        self.delete_messages(mevx, &[msg_id]).await?;

        Ok(())
    }
}
