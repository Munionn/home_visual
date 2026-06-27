use crate::config::AppConfig;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use tracing::{error, info};

#[derive(Clone)]
pub struct MqttClient {
    pub client: AsyncClient,
}

impl MqttClient {
    pub fn new(client: AsyncClient) -> Self {
        Self { client }
    }

    pub fn connect(config: &AppConfig) -> Result<Self, anyhow::Error> {
        let mut mqttoptions = MqttOptions::new(
            &config.mqtt_client_id,
            &config.mqtt_broker_host,
            config.mqtt_broker_port,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        info!(
            "Connecting to MQTT Broker at {}:{}",
            config.mqtt_broker_host, config.mqtt_broker_port
        );

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // Spawn the background MQTT event loop
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(notification) => {
                        info!("Received MQTT notification: {:?}", notification);
                    }
                    Err(e) => {
                        error!("MQTT Event Loop Error: {:?}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });

        Ok(MqttClient { client })
    }
}

pub async fn init_mqtt(config: &AppConfig) -> Result<MqttClient, anyhow::Error> {
    MqttClient::connect(config)
}

impl MqttClient {
    pub async fn publish(&self, topic: &str, payload: &str) -> Result<(), anyhow::Error> {
        info!("Publishing to {}: {}", topic, payload);
        self.client
            .publish(topic, QoS::AtLeastOnce, false, payload.as_bytes())
            .await?;
        Ok(())
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), anyhow::Error> {
        info!("Subscribing to {}", topic);
        self.client.subscribe(topic, QoS::AtLeastOnce).await?;
        Ok(())
    }
}
