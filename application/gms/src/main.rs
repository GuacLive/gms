use {
    //https://rustcc.cn/article?id=6dcbf032-0483-4980-8bfe-c64a7dfb33c7
    anyhow::Result,
    gms::config::{config, config::Config},
    hls::server as hls_server,
    httpflv::server as httpflv_server,

    rtmp::{
        channels::channels::ChannelsManager,
        relay::{pull_client::PullClient, push_client::PushClient},
        rtmp::RtmpServer,
    },
    std::env,
    tokio::signal,
};

use hls::hls_event_manager::HlsEventManager;
use hls::rtmp_event_processor::RtmpEventProcessor;

#[tokio::main]

async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        tracing::error!("please input config file path");
        return Ok(());
    }

    let cfg_path = &args[1];
    let config = config::load(cfg_path);

    match config {
        Ok(val) => {
            /*set log level*/
            if let Some(log_config_value) = &val.log {
                env::set_var("RUST_LOG", log_config_value.level.clone());
            } else {
                env::set_var("RUST_LOG", "info");
            }

            // install global collector configured based on RUST_LOG env var.
            tracing_subscriber::fmt::init();
            /*run the service*/
            let mut service = Service::new(val);
            service.run().await?;
        }
        Err(_) => {
            tracing::error!("load config file failed")
        }
    }

    signal::ctrl_c().await?;
    Ok(())
}

pub struct Service {
    cfg: Config,
}

impl Service {
    pub fn new(cfg: Config) -> Self {
        Service { cfg }
    }

    async fn run(&mut self) -> Result<()> {
        let mut channel = ChannelsManager::new();

        self.start_httpflv(&mut channel).await?;
        self.start_hls(&mut channel).await?;
        self.start_rtmp(&mut channel).await?;

        tokio::spawn(async move { channel.run().await });

        Ok(())
    }

    async fn start_rtmp(&mut self, channel: &mut ChannelsManager) -> Result<()> {
        let rtmp_cfg = &self.cfg.rtmp;

        if let Some(rtmp_cfg_value) = rtmp_cfg {
            if !rtmp_cfg_value.enabled {
                return Ok(());
            }

            let producer = channel.get_session_event_producer();

            /*static push */
            if let Some(push_cfg_values) = &rtmp_cfg_value.push {
                for push_value in push_cfg_values {
                    if !push_value.enabled {
                        continue;
                    }
                    tracing::info!("start rtmp push client..");
                    let address = format!(
                        "{ip}:{port}",
                        ip = push_value.address,
                        port = push_value.port
                    );

                    let mut push_client = PushClient::new(
                        address,
                        channel.get_client_event_consumer(),
                        producer.clone(),
                    );
                    tokio::spawn(async move {
                        if let Err(err) = push_client.run().await {
                            tracing::error!("push client error {}\n", err);
                        }
                    });

                    channel.set_rtmp_push_enabled(true);
                }
            }
            /*static pull*/
            if let Some(pull_cfg_value) = &rtmp_cfg_value.pull {
                if pull_cfg_value.enabled {
                    let address = format!(
                        "{ip}:{port}",
                        ip = pull_cfg_value.address,
                        port = pull_cfg_value.port
                    );
                    tracing::info!("start rtmp pull client from address: {}", address);
                    let mut pull_client = PullClient::new(
                        address,
                        channel.get_client_event_consumer(),
                        producer.clone(),
                    );

                    tokio::spawn(async move {
                        if let Err(err) = pull_client.run().await {
                            tracing::error!("pull client error {}\n", err);
                        }
                    });

                    channel.set_rtmp_pull_enabled(true);
                }
            }

            let listen_port = rtmp_cfg_value.port;
            let address = format!("0.0.0.0:{listen_port}");

            /*static pull*/
            if let Some(webhooks_cfg_value) = &rtmp_cfg_value.webhooks {
                let mut rtmp_server =
                    RtmpServer::new(address, producer, webhooks_cfg_value.clone());
                tokio::spawn(async move {
                    if let Err(err) = rtmp_server.run().await {
                        //print!("rtmp server  error {}\n", err);
                        tracing::error!("rtmp server error: {}\n", err);
                    }
                });
            }
        }

        Ok(())
    }

    async fn start_httpflv(&mut self, channel: &mut ChannelsManager) -> Result<()> {
        let httpflv_cfg = &self.cfg.httpflv;

        if let Some(httpflv_cfg_value) = httpflv_cfg {
            if !httpflv_cfg_value.enabled {
                return Ok(());
            }
            let port = httpflv_cfg_value.port;
            let event_producer = channel.get_session_event_producer();

            tokio::spawn(async move {
                if let Err(err) = httpflv_server::run(event_producer, port).await {
                    //print!("push client error {}\n", err);
                    tracing::error!("httpflv server error: {}\n", err);
                }
            });
        }

        Ok(())
    }

    async fn start_hls(&mut self, channel: &mut ChannelsManager) -> Result<()> {
        let hls_cfg = &self.cfg.hls;

        if let Some(hls_cfg_value) = hls_cfg {
            if !hls_cfg_value.enabled {
                return Ok(());
            }

            let hls_manager = HlsEventManager::new();
            let hls_dispatch = hls_manager.setup_dispatch_channel();

            let event_producer = channel.get_session_event_producer();
            let client_event_consumer = channel.get_client_event_consumer();
            let mut rtmp_event_processor =
                RtmpEventProcessor::new(client_event_consumer, event_producer, hls_dispatch);

            tokio::spawn(async move {
                if let Err(err) = rtmp_event_processor.run().await {
                    // print!("push client error {}\n", err);
                    tracing::error!("rtmp event processor error: {}\n", err);
                }
            });

            let port = hls_cfg_value.port;

            tokio::spawn(async move {
                if let Err(err) = hls_server::run(port, hls_manager).await {
                    tracing::error!("hls server error: {}\n", err);
                }
            });
            channel.set_hls_enabled(true);
        }

        Ok(())
    }
}
