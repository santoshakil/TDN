use chamomile::prelude::SendMessage;
use tdn_types::{
    group::GroupId,
    message::{ReceiveMessage, RecvType, SendType},
    primitive::{DeliveryType, Peer, PeerId, Result, StreamType},
};
use tokio::sync::mpsc::{error::SendError, Sender};

#[inline]
pub(crate) async fn layer_handle_send(
    fgid: GroupId,
    tgid: GroupId,
    p2p_send: &Sender<SendMessage>,
    msg: SendType,
) -> std::result::Result<(), SendError<SendMessage>> {
    // fgid, tgid serialize data to msg data.
    let mut bytes = fgid.0.to_vec();
    bytes.extend(&tgid.0);
    match msg {
        SendType::Connect(tid, peer, data) => {
            bytes.extend(data);
            p2p_send
                .send(SendMessage::StableConnect(tid, peer.into(), bytes))
                .await
        }
        SendType::Disconnect(peer_id) => {
            p2p_send.send(SendMessage::StableDisconnect(peer_id)).await
        }
        SendType::Result(tid, peer, is_ok, is_force, data) => {
            bytes.extend(data);
            p2p_send
                .send(SendMessage::StableResult(
                    tid,
                    peer.into(),
                    is_ok,
                    is_force,
                    bytes,
                ))
                .await
        }
        SendType::Event(tid, peer_id, data) => {
            bytes.extend(data);
            p2p_send.send(SendMessage::Data(tid, peer_id, bytes)).await
        }
        SendType::Stream(id, stream, data) => {
            bytes.extend(data);
            p2p_send.send(SendMessage::Stream(id, stream, bytes)).await
        }
    }
}

#[inline]
pub(crate) async fn layer_handle_connect(
    fgid: GroupId,
    _tgid: GroupId,
    out_send: &Sender<ReceiveMessage>,
    peer: Peer,
    data: Vec<u8>,
) -> Result<()> {
    let gmsg = RecvType::Connect(peer, data);

    #[cfg(any(feature = "single", feature = "std"))]
    let msg = ReceiveMessage::Layer(fgid, gmsg);
    #[cfg(any(feature = "multiple", feature = "full"))]
    let msg = ReceiveMessage::Layer(fgid, _tgid, gmsg);

    out_send
        .send(msg)
        .await
        .map_err(|e| error!("Outside channel: {:?}", e))
        .expect("Outside channel closed");

    Ok(())
}

#[inline]
pub(crate) async fn layer_handle_result_connect(
    fgid: GroupId,
    _tgid: GroupId,
    out_send: &Sender<ReceiveMessage>,
    peer: Peer,
    data: Vec<u8>,
) -> Result<()> {
    let gmsg = RecvType::ResultConnect(peer, data);

    #[cfg(any(feature = "single", feature = "std"))]
    let msg = ReceiveMessage::Layer(fgid, gmsg);
    #[cfg(any(feature = "multiple", feature = "full"))]
    let msg = ReceiveMessage::Layer(fgid, _tgid, gmsg);

    out_send
        .send(msg)
        .await
        .map_err(|e| error!("Outside channel: {:?}", e))
        .expect("Outside channel closed");

    Ok(())
}

#[inline]
pub(crate) async fn layer_handle_result(
    fgid: GroupId,
    _tgid: GroupId,
    out_send: &Sender<ReceiveMessage>,
    peer: Peer,
    is_ok: bool,
    data: Vec<u8>,
) -> Result<()> {
    let gmsg = RecvType::Result(peer, is_ok, data);

    #[cfg(any(feature = "single", feature = "std"))]
    let msg = ReceiveMessage::Layer(fgid, gmsg);
    #[cfg(any(feature = "multiple", feature = "full"))]
    let msg = ReceiveMessage::Layer(fgid, _tgid, gmsg);

    out_send
        .send(msg)
        .await
        .map_err(|e| error!("Outside channel: {:?}", e))
        .expect("Outside channel closed");

    Ok(())
}

#[inline]
pub(crate) async fn layer_handle_leave(
    fgid: GroupId,
    out_send: &Sender<ReceiveMessage>,
    peer_id: PeerId,
) -> Result<()> {
    let gmsg = RecvType::Leave(peer_id);

    #[cfg(any(feature = "single", feature = "std"))]
    let msg = ReceiveMessage::Layer(fgid, gmsg);
    #[cfg(any(feature = "multiple", feature = "full"))]
    let msg = ReceiveMessage::Layer(fgid, fgid, gmsg);

    out_send
        .send(msg)
        .await
        .map_err(|e| error!("Outside channel: {:?}", e))
        .expect("Outside channel closed");

    Ok(())
}

#[inline]
pub(crate) async fn layer_handle_data(
    fgid: GroupId,
    _tgid: GroupId,
    out_send: &Sender<ReceiveMessage>,
    peer_id: PeerId,
    data: Vec<u8>,
) -> Result<()> {
    let gmsg = RecvType::Event(peer_id, data);

    #[cfg(any(feature = "single", feature = "std"))]
    let msg = ReceiveMessage::Layer(fgid, gmsg);
    #[cfg(any(feature = "multiple", feature = "full"))]
    let msg = ReceiveMessage::Layer(fgid, _tgid, gmsg);

    out_send
        .send(msg)
        .await
        .map_err(|e| error!("Outside channel: {:?}", e))
        .expect("Outside channel closed");

    Ok(())
}

#[inline]
pub(crate) async fn layer_handle_stream(
    fgid: GroupId,
    _tgid: GroupId,
    out_send: &Sender<ReceiveMessage>,
    uid: u32,
    stream_type: StreamType,
    data: Vec<u8>,
) -> Result<()> {
    let gmsg = RecvType::Stream(uid, stream_type, data);

    #[cfg(any(feature = "single", feature = "std"))]
    let msg = ReceiveMessage::Layer(fgid, gmsg);
    #[cfg(any(feature = "multiple", feature = "full"))]
    let msg = ReceiveMessage::Layer(fgid, _tgid, gmsg);

    out_send
        .send(msg)
        .await
        .map_err(|e| error!("Outside channel: {:?}", e))
        .expect("Outside channel closed");

    Ok(())
}

#[inline]
pub(crate) async fn layer_handle_delivery(
    fgid: GroupId,
    _tgid: GroupId,
    out_send: &Sender<ReceiveMessage>,
    delivery_type: DeliveryType,
    tid: u64,
    is_sended: bool,
) -> Result<()> {
    let gmsg = RecvType::Delivery(delivery_type, tid, is_sended);

    #[cfg(any(feature = "single", feature = "std"))]
    let msg = ReceiveMessage::Layer(fgid, gmsg);
    #[cfg(any(feature = "multiple", feature = "full"))]
    let msg = ReceiveMessage::Layer(fgid, _tgid, gmsg);

    out_send
        .send(msg)
        .await
        .map_err(|e| error!("Outside channel: {:?}", e))
        .expect("Outside channel closed");

    Ok(())
}
