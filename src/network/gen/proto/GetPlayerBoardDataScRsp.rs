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

//! Generated file from `GetPlayerBoardDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetPlayerBoardDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetPlayerBoardDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetPlayerBoardDataScRsp.signature)
    pub signature: ::std::string::String,
    // @@protoc_insertion_point(field:GetPlayerBoardDataScRsp.unlocked_head_icon_list)
    pub unlocked_head_icon_list: ::std::vec::Vec<super::HeadIcon::HeadIcon>,
    // @@protoc_insertion_point(field:GetPlayerBoardDataScRsp.display_support_avatar_vec)
    pub display_support_avatar_vec: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetPlayerBoardDataScRsp.current_head_icon_id)
    pub current_head_icon_id: u32,
    // @@protoc_insertion_point(field:GetPlayerBoardDataScRsp.display_avatar_vec)
    pub display_avatar_vec: ::protobuf::MessageField<super::DisplayAvatarVec::DisplayAvatarVec>,
    // @@protoc_insertion_point(field:GetPlayerBoardDataScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetPlayerBoardDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetPlayerBoardDataScRsp {
    fn default() -> &'a GetPlayerBoardDataScRsp {
        <GetPlayerBoardDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetPlayerBoardDataScRsp {
    pub fn new() -> GetPlayerBoardDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "signature",
            |m: &GetPlayerBoardDataScRsp| { &m.signature },
            |m: &mut GetPlayerBoardDataScRsp| { &mut m.signature },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlocked_head_icon_list",
            |m: &GetPlayerBoardDataScRsp| { &m.unlocked_head_icon_list },
            |m: &mut GetPlayerBoardDataScRsp| { &mut m.unlocked_head_icon_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "display_support_avatar_vec",
            |m: &GetPlayerBoardDataScRsp| { &m.display_support_avatar_vec },
            |m: &mut GetPlayerBoardDataScRsp| { &mut m.display_support_avatar_vec },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "current_head_icon_id",
            |m: &GetPlayerBoardDataScRsp| { &m.current_head_icon_id },
            |m: &mut GetPlayerBoardDataScRsp| { &mut m.current_head_icon_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DisplayAvatarVec::DisplayAvatarVec>(
            "display_avatar_vec",
            |m: &GetPlayerBoardDataScRsp| { &m.display_avatar_vec },
            |m: &mut GetPlayerBoardDataScRsp| { &mut m.display_avatar_vec },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetPlayerBoardDataScRsp| { &m.retcode },
            |m: &mut GetPlayerBoardDataScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetPlayerBoardDataScRsp>(
            "GetPlayerBoardDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetPlayerBoardDataScRsp {
    const NAME: &'static str = "GetPlayerBoardDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    self.signature = is.read_string()?;
                },
                26 => {
                    self.unlocked_head_icon_list.push(is.read_message()?);
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.display_support_avatar_vec)?;
                },
                120 => {
                    self.display_support_avatar_vec.push(is.read_uint32()?);
                },
                80 => {
                    self.current_head_icon_id = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.display_avatar_vec)?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
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
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.signature);
        }
        for value in &self.unlocked_head_icon_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.display_support_avatar_vec {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        if self.current_head_icon_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.current_head_icon_id);
        }
        if let Some(v) = self.display_avatar_vec.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.signature.is_empty() {
            os.write_string(9, &self.signature)?;
        }
        for v in &self.unlocked_head_icon_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        for v in &self.display_support_avatar_vec {
            os.write_uint32(15, *v)?;
        };
        if self.current_head_icon_id != 0 {
            os.write_uint32(10, self.current_head_icon_id)?;
        }
        if let Some(v) = self.display_avatar_vec.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
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

    fn new() -> GetPlayerBoardDataScRsp {
        GetPlayerBoardDataScRsp::new()
    }

    fn clear(&mut self) {
        self.signature.clear();
        self.unlocked_head_icon_list.clear();
        self.display_support_avatar_vec.clear();
        self.current_head_icon_id = 0;
        self.display_avatar_vec.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetPlayerBoardDataScRsp {
        static instance: GetPlayerBoardDataScRsp = GetPlayerBoardDataScRsp {
            signature: ::std::string::String::new(),
            unlocked_head_icon_list: ::std::vec::Vec::new(),
            display_support_avatar_vec: ::std::vec::Vec::new(),
            current_head_icon_id: 0,
            display_avatar_vec: ::protobuf::MessageField::none(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetPlayerBoardDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetPlayerBoardDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetPlayerBoardDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetPlayerBoardDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dGetPlayerBoardDataScRsp.proto\x1a\x0eHeadIcon.proto\x1a\x16Display\
    AvatarVec.proto\"\xc2\x02\n\x17GetPlayerBoardDataScRsp\x12\x1c\n\tsignat\
    ure\x18\t\x20\x01(\tR\tsignature\x12@\n\x17unlocked_head_icon_list\x18\
    \x03\x20\x03(\x0b2\t.HeadIconR\x14unlockedHeadIconList\x12;\n\x1adisplay\
    _support_avatar_vec\x18\x0f\x20\x03(\rR\x17displaySupportAvatarVec\x12/\
    \n\x14current_head_icon_id\x18\n\x20\x01(\rR\x11currentHeadIconId\x12?\n\
    \x12display_avatar_vec\x18\x0b\x20\x01(\x0b2\x11.DisplayAvatarVecR\x10di\
    splayAvatarVec\x12\x18\n\x07retcode\x18\x0c\x20\x01(\rR\x07retcodeB\x15\
    \n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::HeadIcon::file_descriptor().clone());
            deps.push(super::DisplayAvatarVec::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetPlayerBoardDataScRsp::generated_message_descriptor_data());
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
