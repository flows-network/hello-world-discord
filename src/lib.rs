use discord_flows::{
    model::Message,
    ProvidedBot, Bot,
    message_handler,
};
use flowsnet_platform_sdk::logger;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    let token = std::env::var("discord_token").unwrap();
    let bot = ProvidedBot::new(token);
    bot.listen_to_messages().await;
}

#[message_handler]
async fn handler(msg: Message) {
    logger::init();
    let token = std::env::var("discord_token").unwrap();
    let bot = ProvidedBot::new(token);
    let discord = bot.get_client();

    if msg.author.bot {
        log::debug!("ignored bot message");
        return;
    }
    if msg.member.is_some() {
        log::debug!("ignored channel message");
        return;
    }

    let channel_id = msg.channel_id;
    let resp = format!("Welcome to flows.network.\nYou just said: '{}'.\nLearn more at: https://github.com/flows-network/hello-world\n", msg.content);

    _ = discord.send_message(
        channel_id.into(),
        &serde_json::json!({
            "content": resp
        }),
    ).await;
}
