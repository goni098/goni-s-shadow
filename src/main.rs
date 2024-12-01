mod error;
mod goni;
mod mevx;
mod msg;

use error::Rs;
use goni::Goni;
use grammers_client::{
    session::Session, types::Chat, types::Message, types::Update, Client, Config, InitParams,
};
use msg::{Auto007Message, Order, OrderParams};

#[tokio::main]
async fn main() -> Rs<()> {
    dotenv::dotenv()?;

    let config = Config {
        api_hash: std::env::var("API_HASH")?,
        api_id: std::env::var("API_ID")?
            .parse()
            .expect("expect i32 string api id"),
        params: InitParams::default(),
        session: Session::load_file_or_create("session")?,
    };

    let goni = Client::connect(config).await?;

    if !goni.is_authorized().await? {
        println!("type the phone number: ");

        let mut phone = String::new();
        std::io::stdin().read_line(&mut phone)?;

        let token = goni.request_login_code(&phone).await?;

        println!("type the login code: ");

        let mut code = String::new();
        std::io::stdin().read_line(&mut code)?;

        goni.sign_in(&token, &code).await.map_err(Box::from)?;

        goni.session().save_to_file("session")?;
    }

    let Some(mevx) = goni.resolve_username("MevxTradingBot").await? else {
        return Ok(());
    };

    let Some(auto007) = goni.resolve_username("Auto007").await? else {
        return Ok(());
    };

    while let Ok(Update::NewMessage(message)) = goni.next_update().await {
        process_incoming_message(&goni, &message, &mevx, &auto007)
            .await
            .unwrap_or_else(|error| eprintln!("{:#?}", error));
    }

    Ok(())
}

async fn process_incoming_message(
    goni: &Client,
    message: &Message,
    mevx: &Chat,
    auto007: &Chat,
) -> Rs<()> {
    if !message.is_auto007_message(auto007) {
        return Ok(());
    }

    let OrderParams {
        amount,
        order,
        token,
    } = message.get_order_params();

    match order {
        Order::Buy => goni.make_a_buy_order(mevx, token, amount).await,
        Order::Sell => goni.make_a_sell_order(mevx, token, amount).await,
    }
}
