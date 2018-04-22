// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Items {
    fn list_items(&self, o: ::grpc::RequestOptions, p: super::items::ListItemsRequest) -> ::grpc::SingleResponse<super::items::ListItemsReply>;
}

// client

pub struct ItemsClient {
    grpc_client: ::grpc::Client,
    method_ListItems: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::items::ListItemsRequest, super::items::ListItemsReply>>,
}

impl ItemsClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ItemsClient {
            grpc_client: grpc_client,
            method_ListItems: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/marketplace.Items/ListItems".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ItemsClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ItemsClient::with_client(c)
        })
    }
}

impl Items for ItemsClient {
    fn list_items(&self, o: ::grpc::RequestOptions, p: super::items::ListItemsRequest) -> ::grpc::SingleResponse<super::items::ListItemsReply> {
        self.grpc_client.call_unary(o, p, self.method_ListItems.clone())
    }
}

// server

pub struct ItemsServer;


impl ItemsServer {
    pub fn new_service_def<H : Items + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/marketplace.Items",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/marketplace.Items/ListItems".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_items(o, p))
                    },
                ),
            ],
        )
    }
}
