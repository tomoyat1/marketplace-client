extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
extern crate tls_api;

pub mod items_grpc;
pub mod items;

use items_grpc::*;
use items::*;

fn main() {
    let client_conf = Default::default();
    let client = ItemsClient::new_plain("127.0.0.1", 50051, client_conf).unwrap();

    let mut req = ListItemsRequest::new();
    req.set_page_number(0);

    println!("Calling list_items at localhost:50051");
    let resp = client.list_items(grpc::RequestOptions::new(), req)
        .wait();
    match resp {
        Ok((_, reply, _)) =>  print(&reply.items.into_vec()),
        Err(e) => println!("Error: {}", e),
    }

}

fn print(items: &Vec<Item>) {
    for item in items {
        println!("id: {}", item.id);
        println!("\tname: {}", item.name);
        println!("\tprice: {}", item.price);
        println!("");

    }
}
