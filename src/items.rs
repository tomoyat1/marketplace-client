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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct ListItemsRequest {
    // message fields
    pub page_number: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListItemsRequest {}

impl ListItemsRequest {
    pub fn new() -> ListItemsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListItemsRequest {
        static mut instance: ::protobuf::lazy::Lazy<ListItemsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListItemsRequest,
        };
        unsafe {
            instance.get(ListItemsRequest::new)
        }
    }

    // int32 page_number = 1;

    pub fn clear_page_number(&mut self) {
        self.page_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_page_number(&mut self, v: i32) {
        self.page_number = v;
    }

    pub fn get_page_number(&self) -> i32 {
        self.page_number
    }

    fn get_page_number_for_reflect(&self) -> &i32 {
        &self.page_number
    }

    fn mut_page_number_for_reflect(&mut self) -> &mut i32 {
        &mut self.page_number
    }
}

impl ::protobuf::Message for ListItemsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.page_number = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.page_number != 0 {
            my_size += ::protobuf::rt::value_size(1, self.page_number, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.page_number != 0 {
            os.write_int32(1, self.page_number)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListItemsRequest {
    fn new() -> ListItemsRequest {
        ListItemsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListItemsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "page_number",
                    ListItemsRequest::get_page_number_for_reflect,
                    ListItemsRequest::mut_page_number_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListItemsRequest>(
                    "ListItemsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListItemsRequest {
    fn clear(&mut self) {
        self.clear_page_number();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListItemsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListItemsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListItemsReply {
    // message fields
    pub items: ::protobuf::RepeatedField<Item>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListItemsReply {}

impl ListItemsReply {
    pub fn new() -> ListItemsReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListItemsReply {
        static mut instance: ::protobuf::lazy::Lazy<ListItemsReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListItemsReply,
        };
        unsafe {
            instance.get(ListItemsReply::new)
        }
    }

    // repeated .marketplace.Item items = 1;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<Item>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<Item> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<Item> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[Item] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<Item> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Item> {
        &mut self.items
    }
}

impl ::protobuf::Message for ListItemsReply {
    fn is_initialized(&self) -> bool {
        for v in &self.items {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListItemsReply {
    fn new() -> ListItemsReply {
        ListItemsReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListItemsReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Item>>(
                    "items",
                    ListItemsReply::get_items_for_reflect,
                    ListItemsReply::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListItemsReply>(
                    "ListItemsReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListItemsReply {
    fn clear(&mut self) {
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListItemsReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListItemsReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Item {
    // message fields
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub price: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Item {}

impl Item {
    pub fn new() -> Item {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Item {
        static mut instance: ::protobuf::lazy::Lazy<Item> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Item,
        };
        unsafe {
            instance.get(Item::new)
        }
    }

    // string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // int32 price = 3;

    pub fn clear_price(&mut self) {
        self.price = 0;
    }

    // Param is passed by value, moved
    pub fn set_price(&mut self, v: i32) {
        self.price = v;
    }

    pub fn get_price(&self) -> i32 {
        self.price
    }

    fn get_price_for_reflect(&self) -> &i32 {
        &self.price
    }

    fn mut_price_for_reflect(&mut self) -> &mut i32 {
        &mut self.price
    }
}

impl ::protobuf::Message for Item {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.price = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if self.price != 0 {
            my_size += ::protobuf::rt::value_size(3, self.price, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.price != 0 {
            os.write_int32(3, self.price)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Item {
    fn new() -> Item {
        Item::new()
    }

    fn descriptor_static(_: ::std::option::Option<Item>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Item::get_id_for_reflect,
                    Item::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Item::get_name_for_reflect,
                    Item::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "price",
                    Item::get_price_for_reflect,
                    Item::mut_price_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Item>(
                    "Item",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Item {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_price();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Item {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Item {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bitems.proto\x12\x0bmarketplace\"3\n\x10ListItemsRequest\x12\x1f\n\
    \x0bpage_number\x18\x01\x20\x01(\x05R\npageNumber\"9\n\x0eListItemsReply\
    \x12'\n\x05items\x18\x01\x20\x03(\x0b2\x11.marketplace.ItemR\x05items\"@\
    \n\x04Item\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x12\n\x04name\
    \x18\x02\x20\x01(\tR\x04name\x12\x14\n\x05price\x18\x03\x20\x01(\x05R\
    \x05price2R\n\x05Items\x12I\n\tListItems\x12\x1d.marketplace.ListItemsRe\
    quest\x1a\x1b.marketplace.ListItemsReply\"\0J\x90\x04\n\x06\x12\x04\0\0\
    \x14\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\
    \x13\n\n\n\x02\x06\0\x12\x04\x04\0\x06\x01\n\n\n\x03\x06\0\x01\x12\x03\
    \x04\x08\r\n\x0b\n\x04\x06\0\x02\0\x12\x03\x05\x02>\n\x0c\n\x05\x06\0\
    \x02\0\x01\x12\x03\x05\x06\x0f\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x05\
    \x11!\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x05,:\n\n\n\x02\x04\0\x12\x04\
    \x08\0\n\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x18\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\t\x02\x18\n\r\n\x05\x04\0\x02\0\x04\x12\x04\t\x02\x08\x1a\
    \n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\t\x02\x07\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\t\x08\x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\t\x16\x17\n\n\
    \n\x02\x04\x01\x12\x04\x0c\0\x0e\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0c\
    \x08\x16\n\x0b\n\x04\x04\x01\x02\0\x12\x03\r\x02\x1a\n\x0c\n\x05\x04\x01\
    \x02\0\x04\x12\x03\r\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\r\x0b\
    \x0f\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\r\x10\x15\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03\r\x18\x19\n\n\n\x02\x04\x02\x12\x04\x10\0\x14\x01\n\n\
    \n\x03\x04\x02\x01\x12\x03\x10\x08\x0c\n\x0b\n\x04\x04\x02\x02\0\x12\x03\
    \x11\x02\x10\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x11\x02\x10\x0e\n\x0c\n\
    \x05\x04\x02\x02\0\x05\x12\x03\x11\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03\x11\t\x0b\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x11\x0e\x0f\n\
    \x0b\n\x04\x04\x02\x02\x01\x12\x03\x12\x02\x12\n\r\n\x05\x04\x02\x02\x01\
    \x04\x12\x04\x12\x02\x11\x10\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x12\
    \x02\x08\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x12\t\r\n\x0c\n\x05\x04\
    \x02\x02\x01\x03\x12\x03\x12\x10\x11\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\
    \x13\x02\x12\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x13\x02\x12\x12\n\x0c\
    \n\x05\x04\x02\x02\x02\x05\x12\x03\x13\x02\x07\n\x0c\n\x05\x04\x02\x02\
    \x02\x01\x12\x03\x13\x08\r\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x13\
    \x10\x11b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
