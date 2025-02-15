// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `SendMsgCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SendMsgCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SendMsgCsReq {
    // message fields
    // @@protoc_insertion_point(field:SendMsgCsReq.to_uid_list)
    pub to_uid_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SendMsgCsReq.emote)
    pub emote: u32,
    // @@protoc_insertion_point(field:SendMsgCsReq.msg_type)
    pub msg_type: ::protobuf::EnumOrUnknown<super::MsgType::MsgType>,
    // @@protoc_insertion_point(field:SendMsgCsReq.text)
    pub text: ::std::string::String,
    // @@protoc_insertion_point(field:SendMsgCsReq.chat_type)
    pub chat_type: ::protobuf::EnumOrUnknown<super::ChatType::ChatType>,
    // special fields
    // @@protoc_insertion_point(special_field:SendMsgCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SendMsgCsReq {
    fn default() -> &'a SendMsgCsReq {
        <SendMsgCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SendMsgCsReq {
    pub fn new() -> SendMsgCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "to_uid_list",
            |m: &SendMsgCsReq| { &m.to_uid_list },
            |m: &mut SendMsgCsReq| { &mut m.to_uid_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "emote",
            |m: &SendMsgCsReq| { &m.emote },
            |m: &mut SendMsgCsReq| { &mut m.emote },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "msg_type",
            |m: &SendMsgCsReq| { &m.msg_type },
            |m: &mut SendMsgCsReq| { &mut m.msg_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "text",
            |m: &SendMsgCsReq| { &m.text },
            |m: &mut SendMsgCsReq| { &mut m.text },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "chat_type",
            |m: &SendMsgCsReq| { &m.chat_type },
            |m: &mut SendMsgCsReq| { &mut m.chat_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SendMsgCsReq>(
            "SendMsgCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SendMsgCsReq {
    const NAME: &'static str = "SendMsgCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.to_uid_list)?;
                },
                80 => {
                    self.to_uid_list.push(is.read_uint32()?);
                },
                96 => {
                    self.emote = is.read_uint32()?;
                },
                88 => {
                    self.msg_type = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.text = is.read_string()?;
                },
                120 => {
                    self.chat_type = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.to_uid_list {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.emote != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.emote);
        }
        if self.msg_type != ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(11, self.msg_type.value());
        }
        if !self.text.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.text);
        }
        if self.chat_type != ::protobuf::EnumOrUnknown::new(super::ChatType::ChatType::CHAT_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.chat_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.to_uid_list {
            os.write_uint32(10, *v)?;
        };
        if self.emote != 0 {
            os.write_uint32(12, self.emote)?;
        }
        if self.msg_type != ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.msg_type))?;
        }
        if !self.text.is_empty() {
            os.write_string(2, &self.text)?;
        }
        if self.chat_type != ::protobuf::EnumOrUnknown::new(super::ChatType::ChatType::CHAT_TYPE_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.chat_type))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SendMsgCsReq {
        SendMsgCsReq::new()
    }

    fn clear(&mut self) {
        self.to_uid_list.clear();
        self.emote = 0;
        self.msg_type = ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE);
        self.text.clear();
        self.chat_type = ::protobuf::EnumOrUnknown::new(super::ChatType::ChatType::CHAT_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SendMsgCsReq {
        static instance: SendMsgCsReq = SendMsgCsReq {
            to_uid_list: ::std::vec::Vec::new(),
            emote: 0,
            msg_type: ::protobuf::EnumOrUnknown::from_i32(0),
            text: ::std::string::String::new(),
            chat_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SendMsgCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SendMsgCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SendMsgCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SendMsgCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12SendMsgCsReq.proto\x1a\x0eChatType.proto\x1a\rMsgType.proto\"\xa5\
    \x01\n\x0cSendMsgCsReq\x12\x1e\n\x0bto_uid_list\x18\n\x20\x03(\rR\ttoUid\
    List\x12\x14\n\x05emote\x18\x0c\x20\x01(\rR\x05emote\x12#\n\x08msg_type\
    \x18\x0b\x20\x01(\x0e2\x08.MsgTypeR\x07msgType\x12\x12\n\x04text\x18\x02\
    \x20\x01(\tR\x04text\x12&\n\tchat_type\x18\x0f\x20\x01(\x0e2\t.ChatTypeR\
    \x08chatTypeB\x15\n\x13emu.lunarcore.protob\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::ChatType::file_descriptor().clone());
            deps.push(super::MsgType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SendMsgCsReq::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
