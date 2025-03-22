//! ConsumerGroupHeartbeatRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ConsumerGroupHeartbeatRequest.json).
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

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsumerGroupHeartbeatRequest {
    /// The group identifier.
    ///
    /// Supported API versions: 0-1
    pub group_id: super::GroupId,

    /// The member id generated by the consumer. The member id must be kept during the entire lifetime of the consumer process.
    ///
    /// Supported API versions: 0-1
    pub member_id: StrBytes,

    /// The current member epoch; 0 to join the group; -1 to leave the group; -2 to indicate that the static member will rejoin.
    ///
    /// Supported API versions: 0-1
    pub member_epoch: i32,

    /// null if not provided or if it didn't change since the last heartbeat; the instance Id otherwise.
    ///
    /// Supported API versions: 0-1
    pub instance_id: Option<StrBytes>,

    /// null if not provided or if it didn't change since the last heartbeat; the rack ID of consumer otherwise.
    ///
    /// Supported API versions: 0-1
    pub rack_id: Option<StrBytes>,

    /// -1 if it didn't change since the last heartbeat; the maximum time in milliseconds that the coordinator will wait on the member to revoke its partitions otherwise.
    ///
    /// Supported API versions: 0-1
    pub rebalance_timeout_ms: i32,

    /// null if it didn't change since the last heartbeat; the subscribed topic names otherwise.
    ///
    /// Supported API versions: 0-1
    pub subscribed_topic_names: Option<Vec<super::TopicName>>,

    /// null if it didn't change since the last heartbeat; the subscribed topic regex otherwise.
    ///
    /// Supported API versions: 1
    pub subscribed_topic_regex: Option<StrBytes>,

    /// null if not used or if it didn't change since the last heartbeat; the server side assignor to use otherwise.
    ///
    /// Supported API versions: 0-1
    pub server_assignor: Option<StrBytes>,

    /// null if it didn't change since the last heartbeat; the partitions owned by the member.
    ///
    /// Supported API versions: 0-1
    pub topic_partitions: Option<Vec<TopicPartitions>>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl ConsumerGroupHeartbeatRequest {
    /// Sets `group_id` to the passed value.
    ///
    /// The group identifier.
    ///
    /// Supported API versions: 0-1
    pub fn with_group_id(mut self, value: super::GroupId) -> Self {
        self.group_id = value;
        self
    }
    /// Sets `member_id` to the passed value.
    ///
    /// The member id generated by the consumer. The member id must be kept during the entire lifetime of the consumer process.
    ///
    /// Supported API versions: 0-1
    pub fn with_member_id(mut self, value: StrBytes) -> Self {
        self.member_id = value;
        self
    }
    /// Sets `member_epoch` to the passed value.
    ///
    /// The current member epoch; 0 to join the group; -1 to leave the group; -2 to indicate that the static member will rejoin.
    ///
    /// Supported API versions: 0-1
    pub fn with_member_epoch(mut self, value: i32) -> Self {
        self.member_epoch = value;
        self
    }
    /// Sets `instance_id` to the passed value.
    ///
    /// null if not provided or if it didn't change since the last heartbeat; the instance Id otherwise.
    ///
    /// Supported API versions: 0-1
    pub fn with_instance_id(mut self, value: Option<StrBytes>) -> Self {
        self.instance_id = value;
        self
    }
    /// Sets `rack_id` to the passed value.
    ///
    /// null if not provided or if it didn't change since the last heartbeat; the rack ID of consumer otherwise.
    ///
    /// Supported API versions: 0-1
    pub fn with_rack_id(mut self, value: Option<StrBytes>) -> Self {
        self.rack_id = value;
        self
    }
    /// Sets `rebalance_timeout_ms` to the passed value.
    ///
    /// -1 if it didn't change since the last heartbeat; the maximum time in milliseconds that the coordinator will wait on the member to revoke its partitions otherwise.
    ///
    /// Supported API versions: 0-1
    pub fn with_rebalance_timeout_ms(mut self, value: i32) -> Self {
        self.rebalance_timeout_ms = value;
        self
    }
    /// Sets `subscribed_topic_names` to the passed value.
    ///
    /// null if it didn't change since the last heartbeat; the subscribed topic names otherwise.
    ///
    /// Supported API versions: 0-1
    pub fn with_subscribed_topic_names(mut self, value: Option<Vec<super::TopicName>>) -> Self {
        self.subscribed_topic_names = value;
        self
    }
    /// Sets `subscribed_topic_regex` to the passed value.
    ///
    /// null if it didn't change since the last heartbeat; the subscribed topic regex otherwise.
    ///
    /// Supported API versions: 1
    pub fn with_subscribed_topic_regex(mut self, value: Option<StrBytes>) -> Self {
        self.subscribed_topic_regex = value;
        self
    }
    /// Sets `server_assignor` to the passed value.
    ///
    /// null if not used or if it didn't change since the last heartbeat; the server side assignor to use otherwise.
    ///
    /// Supported API versions: 0-1
    pub fn with_server_assignor(mut self, value: Option<StrBytes>) -> Self {
        self.server_assignor = value;
        self
    }
    /// Sets `topic_partitions` to the passed value.
    ///
    /// null if it didn't change since the last heartbeat; the partitions owned by the member.
    ///
    /// Supported API versions: 0-1
    pub fn with_topic_partitions(mut self, value: Option<Vec<TopicPartitions>>) -> Self {
        self.topic_partitions = value;
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
impl Encodable for ConsumerGroupHeartbeatRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::CompactString.encode(buf, &self.group_id)?;
        types::CompactString.encode(buf, &self.member_id)?;
        types::Int32.encode(buf, &self.member_epoch)?;
        types::CompactString.encode(buf, &self.instance_id)?;
        types::CompactString.encode(buf, &self.rack_id)?;
        types::Int32.encode(buf, &self.rebalance_timeout_ms)?;
        types::CompactArray(types::CompactString).encode(buf, &self.subscribed_topic_names)?;
        if version >= 1 {
            types::CompactString.encode(buf, &self.subscribed_topic_regex)?;
        } else {
            if !self.subscribed_topic_regex.is_none() {
                bail!("A field is set that is not available on the selected protocol version");
            }
        }
        types::CompactString.encode(buf, &self.server_assignor)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.topic_partitions)?;
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
        total_size += types::CompactString.compute_size(&self.instance_id)?;
        total_size += types::CompactString.compute_size(&self.rack_id)?;
        total_size += types::Int32.compute_size(&self.rebalance_timeout_ms)?;
        total_size +=
            types::CompactArray(types::CompactString).compute_size(&self.subscribed_topic_names)?;
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.subscribed_topic_regex)?;
        } else {
            if !self.subscribed_topic_regex.is_none() {
                bail!("A field is set that is not available on the selected protocol version");
            }
        }
        total_size += types::CompactString.compute_size(&self.server_assignor)?;
        total_size +=
            types::CompactArray(types::Struct { version }).compute_size(&self.topic_partitions)?;
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
impl Decodable for ConsumerGroupHeartbeatRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let group_id = types::CompactString.decode(buf)?;
        let member_id = types::CompactString.decode(buf)?;
        let member_epoch = types::Int32.decode(buf)?;
        let instance_id = types::CompactString.decode(buf)?;
        let rack_id = types::CompactString.decode(buf)?;
        let rebalance_timeout_ms = types::Int32.decode(buf)?;
        let subscribed_topic_names = types::CompactArray(types::CompactString).decode(buf)?;
        let subscribed_topic_regex = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            None
        };
        let server_assignor = types::CompactString.decode(buf)?;
        let topic_partitions = types::CompactArray(types::Struct { version }).decode(buf)?;
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
            instance_id,
            rack_id,
            rebalance_timeout_ms,
            subscribed_topic_names,
            subscribed_topic_regex,
            server_assignor,
            topic_partitions,
            unknown_tagged_fields,
        })
    }
}

impl Default for ConsumerGroupHeartbeatRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            member_id: Default::default(),
            member_epoch: 0,
            instance_id: None,
            rack_id: None,
            rebalance_timeout_ms: -1,
            subscribed_topic_names: None,
            subscribed_topic_regex: None,
            server_assignor: None,
            topic_partitions: None,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ConsumerGroupHeartbeatRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct TopicPartitions {
    /// The topic ID.
    ///
    /// Supported API versions: 0-1
    pub topic_id: Uuid,

    /// The partitions.
    ///
    /// Supported API versions: 0-1
    pub partitions: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl TopicPartitions {
    /// Sets `topic_id` to the passed value.
    ///
    /// The topic ID.
    ///
    /// Supported API versions: 0-1
    pub fn with_topic_id(mut self, value: Uuid) -> Self {
        self.topic_id = value;
        self
    }
    /// Sets `partitions` to the passed value.
    ///
    /// The partitions.
    ///
    /// Supported API versions: 0-1
    pub fn with_partitions(mut self, value: Vec<i32>) -> Self {
        self.partitions = value;
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
impl Encodable for TopicPartitions {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Uuid.encode(buf, &self.topic_id)?;
        types::CompactArray(types::Int32).encode(buf, &self.partitions)?;
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
        total_size += types::Uuid.compute_size(&self.topic_id)?;
        total_size += types::CompactArray(types::Int32).compute_size(&self.partitions)?;
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
impl Decodable for TopicPartitions {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let topic_id = types::Uuid.decode(buf)?;
        let partitions = types::CompactArray(types::Int32).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            topic_id,
            partitions,
            unknown_tagged_fields,
        })
    }
}

impl Default for TopicPartitions {
    fn default() -> Self {
        Self {
            topic_id: Uuid::nil(),
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TopicPartitions {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for ConsumerGroupHeartbeatRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}
