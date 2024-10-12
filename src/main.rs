use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use guildtrader::apis::configuration::Configuration;
use guildtrader::apis::*;
use guildtrader::models::*;
use tokio::time::sleep;

const USERNAME: &str = "whyando100";
// Add a delay for clock skew
const DELAY: Duration = Duration::milliseconds(50);

#[tokio::main]
async fn main() {
    let data_dir = format!("./data/{}", USERNAME);
    std::fs::create_dir_all(&data_dir).expect("unable to create data directory");

    // Register, or load token
    let mut c = Configuration::default();
    let token_path = format!("{}/token.txt", data_dir);
    let token = if std::path::Path::new(&token_path).exists() {
        std::fs::read_to_string(&token_path).expect("unable to load token")
    } else {
        let req = RegisterRequest::new(USERNAME.into());
        let resp = user_api::register(&c, req)
            .await
            .expect("unable to register");
        std::fs::write(&token_path, &resp.token).expect("unable to save token");
        resp.token
    };
    c.bearer_access_token = Some(token);

    // Get user
    let user = user_api::profile(&c).await.expect("unable to get user");
    println!("Loaded user {:?}", user);

    tokio::join!(scavenge_loop(&c), guild_loop(&c));
}

fn get_market_entry<'a>(
    market: &'a Assets200Response,
    item: &str,
) -> &'a Assets200ResponseTradesInner {
    market
        .trades
        .iter()
        .find(|&i| i.name == item)
        .expect("unable to find item price")
}

async fn guild_loop(c: &Configuration) {
    let mut contracts = guild_api::contracts(&c)
        .await
        .expect("unable to get contracts");
    loop {
        // Fetch up to date data
        let user = user_api::profile(&c).await.expect("unable to get user");
        let market = trade_api::assets(&c).await.expect("unable to get market");
        let guilds = guild_api::list(&c).await.expect("unable to get guild");
        //println!("User {:?}", user);
        //println!("Guilds {:?}", guilds);
        //println!("Contracts {:?}", contracts);

        // For each guild, complete contract if possible
        let mut completed: Vec<String> = Vec::new();
        for guild in &guilds.data {
            let contract = contracts.data.iter().find(|&c| c.guild == guild.id);
            if let Some(contract) = contract {
                // println!("Guild {} has active contract {:?}", guild.id, contract);
                let completion: DateTime<Utc> = contract.completes_at.parse().unwrap();
                if completion + DELAY <= Utc::now() {
                    let req = CancelRequest::new(guild.id.clone());
                    let complete_result = guild_api::collect(&c, req)
                        .await
                        .expect("unable to complete contract");
                    println!(
                        "Completed T{} {} contract {}x {}: {:?}",
                        contract.tier,
                        contract.guild,
                        contract.quantity,
                        contract.item,
                        complete_result.rewards
                    );
                    completed.push(guild.id.clone());
                }
            }
        }
        for guild in &completed {
            contracts.data.retain(|c| c.guild != *guild);
        }

        // Sell full inventory
        let mut gold = user.gold;
        for (item, quantity) in user.inventory {
            let market_entry = get_market_entry(&market, &item);
            if market_entry.price_status == "low" {
                continue;
            }

            let req = BuyRequest::new(item.clone(), quantity);
            let sell_result = trade_api::sell(&c, req).await.expect("unable to sell");
            let trade = sell_result.trade;
            gold = sell_result.gold;
            println!(
                "Sold {}x {} for {}g",
                trade.quantity, trade.item, trade.price
            );
        }

        // For each guild, take new contract if possible
        for guild in &guilds.data {
            let contract = contracts.data.iter().find(|&c| c.guild == guild.id);
            if contract.is_some() {
                continue;
            }
            // Now, pick the highest tier item that we can afford
            let filtered: Vec<&GuildRequestItemsInner> = guild
                .request_items
                .iter()
                .filter_map(|req| {
                    let market_entry = get_market_entry(&market, &req.item);
                    if gold / 10 >= market_entry.price {
                        Some(req)
                    } else {
                        None
                    }
                })
                .collect();
            let best = filtered.iter().max_by_key(|r| r.tier);
            if let Some(best) = best {
                let req = BuyRequest::new(best.item.clone(), 1);
                trade_api::buy(&c, req).await.expect("unable to buy");
                let req = ContractRequest::new(guild.id.clone(), best.item.clone(), 1);
                let contract = guild_api::contract(&c, req)
                    .await
                    .expect("unable to start contract");
                contracts.data.push(contract);
            }
        }

        // Sleep until next contract completion
        let next_completion = contracts
            .data
            .iter()
            .map(|c| {
                let completion: DateTime<Utc> = c.completes_at.parse().unwrap();
                completion
            })
            .min()
            .unwrap_or(Utc::now() + Duration::seconds(60));
        let sleep_duration = next_completion - Utc::now() + DELAY;
        let sleep_duration = sleep_duration.to_std().unwrap_or_default();
        sleep(sleep_duration).await;
    }
}

async fn scavenge_loop(c: &Configuration) {
    loop {
        // Scavenge
        let scavenge = scavenge_api::scavenge(&c)
            .await
            .expect("unable to scavenge");
        // println!("Scavenged {:?}", scavenge);

        // Sleep
        let cooldown: DateTime<Utc> = scavenge.cooldown.parse().expect("unable to parse cooldown");
        let sleep_duration = cooldown - Utc::now() + DELAY;
        sleep(sleep_duration.to_std().unwrap()).await;
    }
}
