use {
    crate::session::common::SubscriberInfo,
    crate::statistics::StreamStatistics,
    bytes::BytesMut,
    std::fmt,
    tokio::sync::{broadcast, mpsc, oneshot},
};
#[derive(Clone)]
pub enum ChannelData {
    Video { timestamp: u32, data: BytesMut },
    Audio { timestamp: u32, data: BytesMut },
    MetaData { timestamp: u32, data: BytesMut },
}

pub type ChannelDataProducer = mpsc::UnboundedSender<ChannelData>;
pub type ChannelDataConsumer = mpsc::UnboundedReceiver<ChannelData>;

pub type ChannelEventProducer = mpsc::UnboundedSender<ChannelEvent>;
pub type ChannelEventConsumer = mpsc::UnboundedReceiver<ChannelEvent>;

pub type ClientEventProducer = broadcast::Sender<ClientEvent>;
pub type ClientEventConsumer = broadcast::Receiver<ClientEvent>;

pub type TransmitterEventProducer = mpsc::UnboundedSender<TransmitterEvent>;
pub type TransmitterEventConsumer = mpsc::UnboundedReceiver<TransmitterEvent>;

pub type AvStatisticSender = mpsc::UnboundedSender<StreamStatistics>;
pub type AvStatisticReceiver = mpsc::UnboundedReceiver<StreamStatistics>;

pub type StreamStatisticSizeSender = oneshot::Sender<usize>;
pub type StreamStatisticSizeReceiver = oneshot::Sender<usize>;

type ChannelResponder<T> = oneshot::Sender<T>;
#[derive(Debug)]
pub enum ChannelEvent {
    Subscribe {
        app_name: String,
        stream_name: String,
        info: SubscriberInfo,
        responder: ChannelResponder<ChannelDataConsumer>,
    },
    UnSubscribe {
        app_name: String,
        stream_name: String,
        info: SubscriberInfo,
    },
    Publish {
        app_name: String,
        stream_name: String,
        responder: ChannelResponder<ChannelDataProducer>,
    },
    UnPublish {
        app_name: String,
        stream_name: String,
    },
    Api {
        data_sender: AvStatisticSender,
        size_sender: StreamStatisticSizeSender,
    },
}

impl fmt::Display for ChannelEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChannelEvent::Subscribe {
                app_name,
                stream_name,
                info,
                responder: _,
            } => {
                write!(
                    f,
                    "receive event, event_name: Subscribe, app_name: {},stream_name: {}, subscriber id: {}",
                    app_name, stream_name, info.id,
                )
            }
            ChannelEvent::UnSubscribe {
                app_name,
                stream_name,
                info,
            } => {
                write!(
                    f,
                    "receive event, event_name: UnSubscribe, app_name: {},stream_name: {}, subscriber id: {}",
                    app_name, stream_name, info.id,
                )
            }
            ChannelEvent::Publish {
                app_name,
                stream_name,
                responder: _,
            } => {
                write!(
                    f,
                    "receive event, event_name: Publish, app_name: {app_name},stream_name: {stream_name}",
                )
            }
            ChannelEvent::UnPublish {
                app_name,
                stream_name,
            } => {
                write!(
                    f,
                    "receive event, event_name: UnPublish, app_name: {app_name},stream_name: {stream_name}",
                )
            }
            ChannelEvent::Api {
                data_sender: _,
                size_sender: _,
            } => {
                write!(f, "receive event, event_name: Api",)
            }
        }
    }
}

#[derive(Debug)]
pub enum TransmitterEvent {
    Subscribe {
        producer: ChannelDataProducer,
        info: SubscriberInfo,
    },
    UnSubscribe {
        info: SubscriberInfo,
    },
    UnPublish {},

    Api {
        sender: AvStatisticSender,
    },
}

impl fmt::Display for TransmitterEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

#[derive(Debug, Clone)]
pub enum ClientEvent {
    /*Need publish(push) a stream to other rtmp server*/
    Publish {
        app_name: String,
        stream_name: String,
    },
    UnPublish {
        app_name: String,
        stream_name: String,
    },
    /*Need subscribe(pull) a stream from other rtmp server*/
    Subscribe {
        app_name: String,
        stream_name: String,
    },
    UnSubscribe {
        app_name: String,
        stream_name: String,
    },
}
