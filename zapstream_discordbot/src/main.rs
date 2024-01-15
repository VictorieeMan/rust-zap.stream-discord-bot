use std::time::Duration;
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // println!("Start");
    // Generate new keys
    let my_keys: Keys = Keys::generate();

    let client = Client::new(&my_keys);
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://relay.snort.social").await?;
    client.add_relay("wss://nos.lol/").await?;

    client.connect().await;

    // The kind to fetch
    let nostr_kind: Kind = Kind::LiveEventMessage;

    //The IDs to fetch
    let public_key = XOnlyPublicKey::from_bech32(
        "npub1lgqktg83jtasayw8mpcsu46hptf8kdc4effps8mv20ymyqeewpes09l4ps",
    )?;

    let event_id = EventId::from_hex("dddaaac21ec1718ea8e927a3d09c04a2669d42891fe7c57b2897feca714e6552")?;

    //Fetching / filtering content from relays
    let filter = Filter::new().id(event_id).kind(nostr_kind);
    let events = client
        .get_events_of(vec![filter], Some(Duration::from_secs(10)))
        .await;

    //Considering json serializing for the fetched content
    // let content = JsonUtil::from_json(events[0]);
    println!("{events:#?}");

    Ok(())
}