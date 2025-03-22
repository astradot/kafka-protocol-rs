//! ShareGroupHeartbeatRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ShareGroupHeartbeatRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::{bail, Result};
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Decodable, Decoder,
    Encodable, Encoder, HeaderVersion, Message, StrBytes, VersionRange,
};

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct ShareGroupHeartbeatRequest {
    /// The group identifier.
    ///
    /// Supported API versions: 0
    pub group_id: super::GroupId,

    /// The member id.
    ///
    /// Supported API versions: 0
    pub member_id: StrBytes,

    /// The current member epoch; 0 to join the group; -1 to leave the group.
    ///
    /// Supported API versions: 0
    pub member_epoch: i32,

    /// null if not provided or if it didn't change since the last heartbeat; the rack ID of consumer otherwise.
    ///
    /// Supported API versions: 0
    pub rack_id: Option<StrBytes>,

    /// null if it didn't change since the last heartbeat; the subscribed topic names otherwise.
    ///
    /// Supported API versions: 0
    pub subscribed_topic_names: Option<Vec<super::TopicName>>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl ShareGroupHeartbeatRequest {
    /// Sets `group_id` to the passed value.
    ///
    /// The group identifier.
    ///
    /// Supported API versions: 0
    pub fn with_group_id(mut self, value: super::GroupId) -> Self {
        self.group_id = value;
        self
    }
    /// Sets `member_id` to the passed value.
    ///
    /// The member id.
    ///
    /// Supported API versions: 0
    pub fn with_member_id(mut self, value: StrBytes) -> Self {
        self.member_id = value;
        self
    }
    /// Sets `member_epoch` to the passed value.
    ///
    /// The current member epoch; 0 to join the group; -1 to leave the group.
    ///
    /// Supported API versions: 0
    pub fn with_member_epoch(mut self, value: i32) -> Self {
        self.member_epoch = value;
        self
    }
    /// Sets `rack_id` to the passed value.
    ///
    /// null if not provided or if it didn't change since the last heartbeat; the rack ID of consumer otherwise.
    ///
    /// Supported API versions: 0
    pub fn with_rack_id(mut self, value: Option<StrBytes>) -> Self {
        self.rack_id = value;
        self
    }
    /// Sets `subscribed_topic_names` to the passed value.
    ///
    /// null if it didn't change since the last heartbeat; the subscribed topic names otherwise.
    ///
    /// Supported API versions: 0
    pub fn with_subscribed_topic_names(mut self, value: Option<Vec<super::TopicName>>) -> Self {
        self.subscribed_topic_names = value;
        self
    }
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

#[cfg(feature = "client")]
impl Encodable for ShareGroupHeartbeatRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::CompactString.encode(buf, &self.group_id)?;
        types::CompactString.encode(buf, &self.member_id)?;
        types::Int32.encode(buf, &self.member_epoch)?;
        types::CompactString.encode(buf, &self.rack_id)?;
        types::CompactArray(types::CompactString).encode(buf, &self.subscribed_topic_names)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(&self.group_id)?;
        total_size += types::CompactString.compute_size(&self.member_id)?;
        total_size += types::Int32.compute_size(&self.member_epoch)?;
        total_size += types::CompactString.compute_size(&self.rack_id)?;
        total_size +=
            types::CompactArray(types::CompactString).compute_size(&self.subscribed_topic_names)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

#[cfg(feature = "broker")]
impl Decodable for ShareGroupHeartbeatRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let group_id = types::CompactString.decode(buf)?;
        let member_id = types::CompactString.decode(buf)?;
        let member_epoch = types::Int32.decode(buf)?;
        let rack_id = types::CompactString.decode(buf)?;
        let subscribed_topic_names = types::CompactArray(types::CompactString).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            group_id,
            member_id,
            member_epoch,
            rack_id,
            subscribed_topic_names,
            unknown_tagged_fields,
        })
    }
}

impl Default for ShareGroupHeartbeatRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            member_id: Default::default(),
            member_epoch: 0,
            rack_id: None,
            subscribed_topic_names: None,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ShareGroupHeartbeatRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for ShareGroupHeartbeatRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}
