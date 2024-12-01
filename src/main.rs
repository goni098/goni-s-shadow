mod error;
mod goni;
mod mevx;
mod msg;

use error::Rs;
use goni::Goni;
use grammers_client::{
    session::{PackedType, Session},
    types::{Message, PackedChat, Update},
    Client, Config, InitParams,
};
use msg::{Auto007Message, Order, OrderParams};

pub const MEVX: PackedChat = PackedChat {
    id: 7294318663,
    ty: PackedType::Bot,
    access_hash: Some(-7644159981601515537),
};

pub const AUTO007: PackedChat = PackedChat {
    id: 7294318663,
    ty: PackedType::Bot,
    access_hash: Some(-7644159981601515537),
};

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

    while let Ok(update) = goni.next_update().await {
        if let Update::NewMessage(message) = update {
            process_incoming_message(&goni, &message)
                .await
                .unwrap_or_else(|error| eprintln!("{:#?}", error));
        }
    }

    Ok(())
}

async fn process_incoming_message(goni: &Client, message: &Message) -> Rs<()> {
    if !message.is_auto007_message() {
        return Ok(());
    }

    let OrderParams {
        amount,
        order,
        token,
    } = message.get_order_params();

    match order {
        Order::Buy => goni.make_a_buy_order(token, amount).await,
        Order::Sell => goni.make_a_sell_order(token, amount).await,
    }
}
