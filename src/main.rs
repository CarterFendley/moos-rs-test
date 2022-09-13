use moos::async_client::AsyncClient;
use simple_logger::SimpleLogger;
use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).without_timestamps().init().unwrap();
    let mut client = AsyncClient::new("CartersClient");

    client.set_on_connect(|| {
        log::info!("Client CONNECTED!");
    });
    client.set_on_disconnect(|| {
        log::info!("Client DISCONNECTED!");
    });

    if let Ok(()) = client.connect_to("localhost", 9000).await {
        log::info!("Connected to community: {}", client.get_community());
    }

    // Setup subscriptions
    let mut sub_vars = Vec::<String>::new();
    sub_vars.push("NAV_X".into());
    sub_vars.push("NAV_Y".into());
    sub_vars.push("NAV_HEADING".into());
    for s in sub_vars {
        if let Err(e) = client.subscribe(&s, 0.) {
            log::error!("Subscription failed: {:?}", e);
        }
    }

    let inbox = client.start_consuming();
    let comms = tokio::spawn(async move {
        loop {
            for message in inbox.try_iter() {
                log::info!("Received message: {}", message);
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    });
    
    let _ = tokio::join!(comms);

    return Ok(());
}
