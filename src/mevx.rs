use crate::{
    error::Rs,
    msg::{MevxMessage, MexvBuySellPopUp},
};
use grammers_client::{
    grammers_tl_types::{
        enums::InputPeer, functions::messages::GetBotCallbackAnswer, types::KeyboardButtonCallback,
    },
    types::{Chat, Message},
    Client,
};
use std::time::Duration;

pub trait ChatingWithMevx {
    async fn press_a_callback_btn(&self, msg_id: i32, btn: &KeyboardButtonCallback) -> Rs<()>;
    async fn paste_token_address(
        &self,
        mevx: &Chat,
        token: &str,
    ) -> Rs<(MexvBuySellPopUp, Message)>;
    async fn type_amount(
        &self,
        mevx: &Chat,
        msg_id: i32,
        typing_btn: KeyboardButtonCallback,
        amount: f64,
    ) -> Rs<()>;
}

impl ChatingWithMevx for Client {
    async fn paste_token_address(
        &self,
        mevx: &Chat,
        token: &str,
    ) -> Rs<(MexvBuySellPopUp, Message)> {
        let token_msg = self.send_message(mevx, token).await?;

        loop {
            if let Some(msg) = self.iter_messages(mevx).limit(1).next().await? {
                if let Some(popup) = msg.get_buy_sell_popup() {
                    self.delete_messages(mevx, &[token_msg.id()]).await?;
                    return Ok((popup, msg));
                } else {
                    tokio::time::sleep(Duration::from_millis(250)).await;
                }
            }
        }
    }

    async fn press_a_callback_btn(&self, msg_id: i32, btn: &KeyboardButtonCallback) -> Rs<()> {
        self.invoke(&GetBotCallbackAnswer {
            data: Some(btn.data.clone()),
            game: false,
            msg_id,
            peer: InputPeer::PeerSelf,
            password: None,
        })
        .await?;

        Ok(())
    }

    async fn type_amount(
        &self,
        mevx: &Chat,
        msg_id: i32,
        typing_btn: KeyboardButtonCallback,
        amount: f64,
    ) -> Rs<()> {
        let typing_amount_msg = loop {
            if let Some(msg) = self.iter_messages(mevx).limit(1).next().await? {
                if msg.text().contains("Enter") && msg.text().contains("amount") {
                    break msg;
                } else {
                    let _ = tokio::time::timeout(
                        Duration::from_millis(500),
                        self.invoke(&GetBotCallbackAnswer {
                            data: Some(typing_btn.data.clone()),
                            game: false,
                            msg_id,
                            peer: InputPeer::PeerSelf,
                            password: None,
                        }),
                    )
                    .await;
                }
            }
        };

        typing_amount_msg.reply(amount.to_string()).await?;

        Ok(())
    }
}
