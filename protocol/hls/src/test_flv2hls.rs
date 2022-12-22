#[cfg(test)]
mod tests {
    use crate::errors::MediaError;
    use crate::flv2hls::Flv2HlsRemuxer;

    use bytes::BytesMut;
    use tokio::sync::{broadcast, mpsc};
    use xflv::define::FlvData;

    use xflv::demuxer::FlvDemuxer;

    use std::fs::File;
    use std::io::prelude::*;
    use std::time::Instant;

    #[allow(dead_code)]
    pub fn print(data: BytesMut) {
        println!("==========={}", data.len());
        let mut idx = 0;
        for i in data {
            print!("{:02X} ", i);
            idx += 1;
            match idx % 16 {
                0 => {
                    println!()
                }
                _ => {}
            }
        }

        println!("===========")
    }
    #[allow(dead_code)]
    pub fn print_flv_data(data: FlvData) {
        match data {
            FlvData::Audio { timestamp, data } => {
                println! {"==audio data begin=="};
                println! {"timestamp: {}",timestamp};
                println! {"data :"};
                print(data);
                println! {"==audio data end=="};
            }
            FlvData::Video { timestamp, data } => {
                println! {"==video data begin=="};
                println! {"timestamp: {}",timestamp};
                println! {"data :"};
                print(data);
                println! {"==video data end=="};
            }
            _ => {
                println!("not video or audio ")
            }
        }
    }

    //#[test]
    #[allow(dead_code)]
    fn test_flv2hls() -> Result<(), MediaError> {
        let mut file =
            File::open("/Users/phin/misc/gms/protocol/hls/src/xgplayer_demo.flv").unwrap();
        let mut contents = Vec::new();

        file.read_to_end(&mut contents)?;

        let mut data = BytesMut::new();
        data.extend(contents);

        let mut demuxer = FlvDemuxer::new(data);
        demuxer.read_flv_header()?;

        let start = Instant::now();

        let (tx, _rx) = broadcast::channel(2);
        let tx2 = tx;
        let (_m3u8_tx, m3u8_rx) = mpsc::channel(1);

        let mut media_demuxer = Flv2HlsRemuxer::new(
            tx2,
            m3u8_rx,
            5,
            200,
            String::from("live"),
            String::from("test"),
        );

        loop {
            let data_ = demuxer.read_flv_tag();

            match data_ {
                Ok(data) => {
                    if let Some(real_data) = data {
                        media_demuxer.process_flv_data(real_data)?;
                    }
                }

                Err(err) => {
                    media_demuxer.flush_remaining_data()?;
                    log::error!("readd err: {}", err);
                    break;
                }
            }
        }

        let elapsed = start.elapsed();
        println!("Debug: {:?}", elapsed);

        Ok(())
    }
}
