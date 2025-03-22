// impl From<crate::Options> for tungstenite::protocol::WebSocketConfig {
//     fn from(options: crate::Options) -> Self {
//         let crate::Options {
//             max_incoming_frame_size,
//             ..
//         } = options;

//         let max_frame_size = if max_incoming_frame_size == usize::MAX {
//             None
//         } else {
//             Some(max_incoming_frame_size)
//         };

//         let default = tungstenite::protocol::WebSocketConfig::default();

//         Self {
//             max_frame_size,
//             read_buffer_size: default.read_buffer_size,
//             write_buffer_size: default.write_buffer_size,
//             max_write_buffer_size: default.max_write_buffer_size,
//             max_message_size: default.max_message_size,
//             accept_unmasked_frames: default.accept_unmasked_frames,
//         }
//     }
// }

/// transform uri and options into a request builder
pub fn into_requester(
    uri: tungstenite::http::Uri,
    options: crate::Options,
) -> tungstenite::client::ClientRequestBuilder {
    let mut client_request = tungstenite::client::ClientRequestBuilder::new(uri);
    for (key, value) in options.additional_headers {
        client_request = client_request.with_header(key, value);
    }
    for subprotocol in options.subprotocols {
        client_request = client_request.with_sub_protocol(subprotocol);
    }
    client_request
}
