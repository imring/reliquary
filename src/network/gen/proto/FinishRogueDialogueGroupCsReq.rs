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

//! Generated file from `FinishRogueDialogueGroupCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FinishRogueDialogueGroupCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FinishRogueDialogueGroupCsReq {
    // message fields
    // @@protoc_insertion_point(field:FinishRogueDialogueGroupCsReq.entity_id)
    pub entity_id: u32,
    // @@protoc_insertion_point(field:FinishRogueDialogueGroupCsReq.dialogue_group_id)
    pub dialogue_group_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FinishRogueDialogueGroupCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FinishRogueDialogueGroupCsReq {
    fn default() -> &'a FinishRogueDialogueGroupCsReq {
        <FinishRogueDialogueGroupCsReq as ::protobuf::Message>::default_instance()
    }
}

impl FinishRogueDialogueGroupCsReq {
    pub fn new() -> FinishRogueDialogueGroupCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &FinishRogueDialogueGroupCsReq| { &m.entity_id },
            |m: &mut FinishRogueDialogueGroupCsReq| { &mut m.entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dialogue_group_id",
            |m: &FinishRogueDialogueGroupCsReq| { &m.dialogue_group_id },
            |m: &mut FinishRogueDialogueGroupCsReq| { &mut m.dialogue_group_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FinishRogueDialogueGroupCsReq>(
            "FinishRogueDialogueGroupCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FinishRogueDialogueGroupCsReq {
    const NAME: &'static str = "FinishRogueDialogueGroupCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.entity_id = is.read_uint32()?;
                },
                8 => {
                    self.dialogue_group_id = is.read_uint32()?;
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
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.entity_id);
        }
        if self.dialogue_group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.dialogue_group_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.entity_id != 0 {
            os.write_uint32(15, self.entity_id)?;
        }
        if self.dialogue_group_id != 0 {
            os.write_uint32(1, self.dialogue_group_id)?;
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

    fn new() -> FinishRogueDialogueGroupCsReq {
        FinishRogueDialogueGroupCsReq::new()
    }

    fn clear(&mut self) {
        self.entity_id = 0;
        self.dialogue_group_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FinishRogueDialogueGroupCsReq {
        static instance: FinishRogueDialogueGroupCsReq = FinishRogueDialogueGroupCsReq {
            entity_id: 0,
            dialogue_group_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FinishRogueDialogueGroupCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FinishRogueDialogueGroupCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FinishRogueDialogueGroupCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FinishRogueDialogueGroupCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#FinishRogueDialogueGroupCsReq.proto\"h\n\x1dFinishRogueDialogueGroupC\
    sReq\x12\x1b\n\tentity_id\x18\x0f\x20\x01(\rR\x08entityId\x12*\n\x11dial\
    ogue_group_id\x18\x01\x20\x01(\rR\x0fdialogueGroupIdB\x15\n\x13emu.lunar\
    core.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FinishRogueDialogueGroupCsReq::generated_message_descriptor_data());
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
