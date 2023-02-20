## GMS (Guac Media Server)

gms is a simple and secure live media server written by pure Rust, it now supports popular live protocols like RTMP/HLS/LLHLS/HTTP-FLV (and maybe other protocols in the future), you can deploy it as a stand-alone server or a cluster using the relay feature.

## Features

- [x] Support multiple platforms(Linux/MacOS/Windows).
- [x] Support RTMP as a 
stand-alone server or cluster(RTMP relay).
   - [x] Support GOP cache.
- [x] Support HTTP-FLV/HLS protocols(Transferred from RTMP).
- [x] Support configuring the service using command line or a configuration file.
- [ ] Support HTTP API/Notifications.
  - [ ] Support querying stream/machine information and so on.
  - [x] Support notify stream status.
- [ ] Support token authentications.
- [ ] Support RTSP.
## Preparation

#### Install Rust and Cargo

[Document](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Install and run

There are two ways to install gms:

- Using cargo to install
- Building from source

### Install using cargo

Issue the following command to install gms:

    cargo install gms

Start the service with the following command to get help:

    gms -h
    A secure and easy to use live media server

    Usage: gms [OPTIONS] <--config <path>|--rtmp <port>>

    Options:
      -c, --config <path>   Specify the gms server configuration file path.
      -r, --rtmp <port>     Specify the RTMP listening port(e.g.:1935).
      -f, --httpflv <port>  Specify the HTTP-FLV listening port(e.g.:8080).
      -s, --hls <port>      Specify the HLS listening port(e.g.:8081).
      -l, --log <level>     Specify the log level. [possible values: trace, debug, info, warn, error, debug]
      -h, --help            Print help
      -V, --version         Print version
    
### Build from source

#### Clone gms

    git clone https://github.com/guaclive/gms.git

#### Build

    cd ./gms/application/gms
    cargo build --release

#### Run

    cd ./gms/target/release
    ./gms -h
    
## CLI

#### Instructions

You can use command line to configure the gms server easily. You can specify to configure gms using configuration file or from the command lines.

##### Configure using file

    gms -c configuration_file_path

##### Configure using command line

    gms -r 1935 -f 8080 -s 8081 -l info


#### How to Configure the configuration file

##### RTMP

    [rtmp]
    enabled = true
    port = 1935

    # send webhook on publish and publish done
    [rtmp.webhooks]
    enabled = true
    publish_url = "http://localhost:8080/api/v1/publish"
    publish_done_url = "http://localhost:8080/api/v1/publish_done"

    # pull streams from other server node.
    [rtmp.pull]
    enabled = false
    address = "192.168.0.1"
    port = 1935

    # push streams to other server node.
    [[rtmp.push]]
    enabled = true
    address = "localhost"
    port = 1936
    [[rtmp.push]]
    enabled = true
    address = "192.168.0.3"
    port = 1935

##### HTTPFLV

    [httpflv]
    # true or false to enable or disable the feature
    enabled = true
    # listening port
    port = 8081

##### HLS

    [hls]
    # true or false to enable or disable the feature
    enabled = true
    # listening port
    port = 8080

##### Log

    [log]
    level = "info"
    [log.file]
    # write log to file or not（Writing logs to file or console cannot be satisfied at the same time）.
    enabled = true
    # set the rotate
    rotate = "hour" #[day,hour,minute]
    # set the path where the logs are saved
    path = "./logs"
    
### Configuration examples

I edit some configuration files under the following path which can be used directly:

    gms/application/gms/src/config

It contains the following 4 files:

    config_rtmp.toml //enable rtmp only
    config_rtmp_hls.toml //enable rtmp and hls
    config_rtmp_httpflv.toml //enable rtmp and httpflv
    config_rtmp_httpflv_hls.toml //enable all the 3 protocols

## Scenarios

##### Push

You can use two ways:

- Use OBS to push a live rtmp stream
- Or use FFmpeg to push a rtmp stream:
  ffmpeg -re -stream_loop -1 -i test.mp4 -c:a copy -c:v copy -f flv -flvflags no_duration_filesize rtmp://127.0.0.1:1935/live/test

##### Play

Use ffplay to play the rtmp/httpflv/hls live stream:

    ffplay -i rtmp://localhost:1935/live/test
    ffplay -i http://localhost:8081/live/test.flv
    ffplay -i http://localhost:8080/live/test/test.m3u8

##### Relay - Static push

The configuration files are as follows:

The configuration file of Service 1 named config.toml:

    [rtmp]
    enabled = true
    port = 1935
    [[rtmp.push]]
    enabled = true
    address = "localhost"
    port = 1936

The configuration file of Service 2 named config_push.toml:

    [rtmp]
    enabled = true
    port = 1936

Run the 2 services:

    ./gms config.toml
    ./gms config_push.toml

Use the above methods to push rtmp live stream to service 1, then the stream can be pushed to service 2 automatically, you can play the same live stream from both the two services:

    ffplay -i rtmp://localhost:1935/live/test
    ffplay -i rtmp://localhost:1936/live/test

##### Relay - Static pull

The configuration file are as follows:

The configuration file of Service 1 named config.toml:

    [rtmp]
    enabled = true
    port = 1935

The configuration file of Service 2 named config_pull.toml:

    [rtmp]
    enabled = true
    port = 1936
    [rtmp.pull]
    enabled = false
    address = "localhost"
    port = 1935

Run the 2 services:

    ./gms config.toml
    ./gms config_pull.toml

Use the above methods to push live stream to service 1, when you play the stream from serivce 2, it will pull the stream from service 1:

    ffplay -i rtmp://localhost:1935/live/test
    ffplay -i rtmp://localhost:1936/live/test

## Thanks

- [media_server](https://github.com/ireader/media-server.git)
- [xiu](https://github.com/harlanc/xiu) - Codebase this was forked from.
- [Phineas](https://github.com/phineas/xiu) - LLHLS implementation.

## Others

Open issues if you have any problems. Star and pull requests are welcomed. Your stars can make this project go faster and further.
