use crate::error::{Error, Rs};
use grammers_client::{
    grammers_tl_types::{
        enums::{KeyboardButton, KeyboardButtonRow, ReplyMarkup},
        types::KeyboardButtonCallback,
    },
    types::{Chat, Message},
};

pub trait MevxMessage {
    fn get_mevx_buy_sell_popup(&self) -> Option<MexvBuySellPopUp>;
    async fn get_typing_amount_btn(&self, option: Order) -> Rs<KeyboardButtonCallback>;
}

impl MevxMessage for Message {
    fn get_mevx_buy_sell_popup(&self) -> Option<MexvBuySellPopUp> {
        self.reply_markup()
            .and_then(|reply_markup| match reply_markup {
                ReplyMarkup::ReplyInlineMarkup(inline_markup) => Some(inline_markup.rows),
                _ => None,
            })
            .map(|rows| {
                rows.into_iter().map(|row| match row {
                    KeyboardButtonRow::Row(row) => row.buttons.into_iter(),
                })
            })
            .map(Iterator::flatten)
            .map(|buttons| {
                buttons.filter_map(|button| match button {
                    KeyboardButton::Callback(callback) => Some(callback),
                    _ => None,
                })
            })
            .and_then(|mut btn_callbacks| {
                btn_callbacks
                    .find(|btn_callback| btn_callback.text.contains("🎯 Buy"))
                    .zip(btn_callbacks.find(|btn_callback| btn_callback.text.contains("🎯 Sell")))
                    .zip(btn_callbacks.find(|btn_callback| btn_callback.text.contains("Swap")))
                    .zip(btn_callbacks.find(|btn_callback| btn_callback.text.contains("Limit")))
            })
            .map(|(((buy, sell), swap), limit)| MexvBuySellPopUp {
                buy,
                sell,
                swap,
                limit,
            })
    }

    async fn get_typing_amount_btn(&self, option: Order) -> Rs<KeyboardButtonCallback> {
        let button = self
            .refetch()
            .await?
            .reply_markup()
            .and_then(|reply_markup| match reply_markup {
                ReplyMarkup::ReplyInlineMarkup(inline_markup) => Some(inline_markup.rows),
                _ => None,
            })
            .and_then(|rows| {
                rows.into_iter().find_map(|row| match row {
                    KeyboardButtonRow::Row(row) => row.buttons.into_iter().find(|button| {
                        button
                            .text()
                            .to_lowercase()
                            .contains(option.to_query_display())
                    }),
                })
            })
            .and_then(|button| match button {
                KeyboardButton::Callback(callback) => Some(callback),
                _ => None,
            })
            .ok_or(Error::Custom("not found buy x sol button".to_string()))?;

        Ok(button)
    }
}

pub trait Auto007Message {
    fn is_auto007_message(&self, auto007: &Chat) -> bool;
    fn get_order_params(&self) -> OrderParams;
}

impl Auto007Message for Message {
    fn is_auto007_message(&self, auto007: &Chat) -> bool {
        self.sender()
            .is_some_and(|sender| sender.id() == auto007.id())
    }

    fn get_order_params(&self) -> OrderParams {
        OrderParams {
            amount: 1f64,
            order: Order::Buy,
            token: "c6ab7c1b524d470fbd24099971a3d0e7".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MexvBuySellPopUp {
    pub buy: KeyboardButtonCallback,
    pub sell: KeyboardButtonCallback,
    pub swap: KeyboardButtonCallback,
    pub limit: KeyboardButtonCallback,
}

#[derive(Debug)]
pub struct OrderParams {
    pub token: String,
    pub amount: f64,
    pub order: Order,
}

#[derive(Debug)]
pub enum Order {
    Buy,
    Sell,
}

impl Order {
    fn to_query_display(&self) -> &str {
        match self {
            Self::Buy => "buy x",
            Self::Sell => "sell x",
        }
    }
}
