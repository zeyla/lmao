use crate::{
    gateway::presence::{Presence, UserOrId},
    guild::member::{Member, MemberIntermediary},
    id::{GuildId, UserId},
};
use serde::Serialize;
use serde::{
    de::{Deserializer, Error as DeError, MapAccess, Visitor},
    Deserialize,
};
use serde_value::Value;
use std::{
    collections::HashMap,
    fmt::{Formatter, Result as FmtResult},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MemberChunk {
    pub guild_id: GuildId,
    #[serde(with = "serde_mappable_seq")]
    pub members: HashMap<UserId, Member>,
    #[serde(with = "serde_mappable_seq", default)]
    pub presences: HashMap<UserId, Presence>,
    pub chunk_index: u32,
    pub chunk_count: u32,
    #[serde(default)]
    pub not_found: Vec<UserId>,
    pub nonce: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field {
    ChunkCount,
    ChunkIndex,
    GuildId,
    Members,
    Nonce,
    NotFound,
    Presences,
}

struct MemberChunkVisitor;

impl<'de> Visitor<'de> for MemberChunkVisitor {
    type Value = MemberChunk;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct MemberChunk")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut chunk_count = None;
        let mut chunk_index = None;
        let mut guild_id = None;
        let mut members = None::<Value>;
        let mut nonce = None;
        let mut not_found = None;
        let mut presences = None::<Value>;

        loop {
            let key = match map.next_key() {
                Ok(Some(key)) => key,
                Ok(None) => break,
                Err(_) => {
                    // Encountered when we run into an unknown key.
                    continue;
                }
            };

            match key {
                Field::ChunkCount => {
                    if chunk_count.is_some() {
                        return Err(DeError::duplicate_field("chunk_count"));
                    }

                    chunk_count = Some(map.next_value()?);
                }
                Field::ChunkIndex => {
                    if chunk_index.is_some() {
                        return Err(DeError::duplicate_field("chunk_index"));
                    }

                    chunk_index = Some(map.next_value()?);
                }
                Field::GuildId => {
                    if guild_id.is_some() {
                        return Err(DeError::duplicate_field("guild_id"));
                    }

                    guild_id = Some(map.next_value()?);
                }
                Field::Members => {
                    if members.is_some() {
                        return Err(DeError::duplicate_field("members"));
                    }

                    members = Some(map.next_value()?);
                }
                Field::Nonce => {
                    if nonce.is_some() {
                        return Err(DeError::duplicate_field("nonce"));
                    }

                    nonce = Some(map.next_value()?);
                }
                Field::NotFound => {
                    if not_found.is_some() {
                        return Err(DeError::duplicate_field("not_found"));
                    }

                    not_found = Some(map.next_value()?);
                }
                Field::Presences => {
                    if presences.is_some() {
                        return Err(DeError::duplicate_field("presences"));
                    }

                    presences = Some(map.next_value()?);
                }
            }
        }

        let chunk_count = chunk_count.ok_or_else(|| DeError::missing_field("chunk_count"))?;
        let chunk_index = chunk_index.ok_or_else(|| DeError::missing_field("chunk_index"))?;
        let guild_id = guild_id.ok_or_else(|| DeError::missing_field("guild_id"))?;
        let members = members.ok_or_else(|| DeError::missing_field("members"))?;
        let not_found = not_found.unwrap_or_default();

        let members = members
            .deserialize_into::<Vec<MemberIntermediary>>()
            .map_err(DeError::custom)?
            .into_iter()
            .map(|member| {
                (
                    member.user.id,
                    Member {
                        deaf: member.deaf,
                        guild_id,
                        hoisted_role: member.hoisted_role,
                        joined_at: member.joined_at,
                        mute: member.mute,
                        nick: member.nick,
                        premium_since: member.premium_since,
                        roles: member.roles,
                        user: member.user,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        let presences = match presences {
            Some(presences) => presences
                .deserialize_into::<Vec<Presence>>()
                .map_err(DeError::custom)?
                .into_iter()
                .map(|presence| {
                    let user_id = match presence.user {
                        UserOrId::User(ref u) => u.id,
                        UserOrId::UserId { id } => id,
                    };

                    (user_id, presence)
                })
                .collect::<HashMap<_, _>>(),
            None => HashMap::new(),
        };

        Ok(MemberChunk {
            chunk_count,
            chunk_index,
            guild_id,
            members,
            nonce,
            not_found,
            presences,
        })
    }
}

impl<'de> Deserialize<'de> for MemberChunk {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        const FIELDS: &[&str] = &[
            "chunk_count",
            "chunk_index",
            "guild_id",
            "members",
            "nonce",
            "not_found",
            "presences",
        ];

        deserializer.deserialize_struct("MemberChunk", FIELDS, MemberChunkVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::super::MemberChunk;
    use crate::{
        gateway::presence::{ClientStatus, Presence, Status, UserOrId},
        guild::Member,
        id::{GuildId, RoleId, UserId},
        user::{User, UserFlags},
    };
    use std::collections::HashMap;

    #[test]
    fn test_simple_member_chunk() {
        let input = serde_json::json!({
            "chunk_count": 1,
            "chunk_index": 0,
            "guild_id": "1",
            "members": [{
                "deaf": false,
                "hoisted_role": "6",
                "joined_at": "2020-04-04T04:04:04.000000+00:00",
                "mute": false,
                "nick": "chunk",
                "roles": ["6"],
                "user": {
                    "avatar": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "discriminator": "0001",
                    "id": "5",
                    "public_flags": 131072,
                    "username": "test",
                },
            }, {
                "deaf": false,
                "hoisted_role": "6",
                "joined_at": "2020-04-04T04:04:04.000000+00:00",
                "mute": false,
                "nick": "chunk",
                "roles": ["6"],
                "user": {
                    "avatar": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                    "discriminator": "0001",
                    "id": "6",
                    "username": "test",
                },
            }, {
                "deaf": false,
                "hoisted_role": "6",
                "joined_at": "2020-04-04T04:04:04.000000+00:00",
                "mute": false,
                "nick": "chunk",
                "roles": ["6"],
                "user": {
                    "avatar": "cccccccccccccccccccccccccccccccc",
                    "bot": true,
                    "discriminator": "0001",
                    "id": "3",
                    "username": "test",
                },
            }, {
                "deaf": false,
                "hoisted_role": "6",
                "joined_at": "2020-04-04T04:04:04.000000+00:00",
                "mute": false,
                "nick": "chunk",
                "roles": [
                    "6",
                    "7",
                ],
                "user": {
                    "avatar": "dddddddddddddddddddddddddddddddd",
                    "bot": true,
                    "discriminator": "0001",
                    "id": "2",
                    "username": "test",
                },
            }],
            "presences": [{
                "activities": [],
                "client_status": {
                    "web": "online",
                },
                "game": null,
                "status": "online",
                "user": {
                    "id": "2",
                },
            }, {
                "activities": [],
                "client_status": {
                    "web": "online",
                },
                "game": null,
                "status": "online",
                "user": {
                    "id": "3",
                },
            }, {
                "activities": [],
                "client_status": {
                    "desktop": "dnd",
                },
                "game": null,
                "status": "dnd",
                "user": {
                    "id": "5",
                },
            }],
        });

        let expected = MemberChunk {
            chunk_count: 1,
            chunk_index: 0,
            guild_id: GuildId(1),
            members: {
                let mut members = HashMap::new();
                members.insert(
                    UserId(2),
                    Member {
                        deaf: false,
                        guild_id: GuildId(1),
                        hoisted_role: Some(RoleId(6)),
                        joined_at: Some("2020-04-04T04:04:04.000000+00:00".to_owned()),
                        mute: false,
                        nick: Some("chunk".to_owned()),
                        premium_since: None,
                        roles: vec![RoleId(6), RoleId(7)],
                        user: User {
                            id: UserId(2),
                            avatar: Some("dddddddddddddddddddddddddddddddd".to_owned()),
                            bot: true,
                            discriminator: "0001".to_owned(),
                            name: "test".to_owned(),
                            mfa_enabled: None,
                            locale: None,
                            verified: None,
                            email: None,
                            flags: None,
                            premium_type: None,
                            system: None,
                            public_flags: None,
                        },
                    },
                );
                members.insert(
                    UserId(3),
                    Member {
                        deaf: false,
                        guild_id: GuildId(1),
                        hoisted_role: Some(RoleId(6)),
                        joined_at: Some("2020-04-04T04:04:04.000000+00:00".to_owned()),
                        mute: false,
                        nick: Some("chunk".to_owned()),
                        premium_since: None,
                        roles: vec![RoleId(6)],
                        user: User {
                            id: UserId(3),
                            avatar: Some("cccccccccccccccccccccccccccccccc".to_owned()),
                            bot: true,
                            discriminator: "0001".to_owned(),
                            name: "test".to_owned(),
                            mfa_enabled: None,
                            locale: None,
                            verified: None,
                            email: None,
                            flags: None,
                            premium_type: None,
                            system: None,
                            public_flags: None,
                        },
                    },
                );
                members.insert(
                    UserId(5),
                    Member {
                        deaf: false,
                        guild_id: GuildId(1),
                        hoisted_role: Some(RoleId(6)),
                        joined_at: Some("2020-04-04T04:04:04.000000+00:00".to_owned()),
                        mute: false,
                        nick: Some("chunk".to_owned()),
                        premium_since: None,
                        roles: vec![RoleId(6)],
                        user: User {
                            id: UserId(5),
                            avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                            bot: false,
                            discriminator: "0001".to_owned(),
                            name: "test".to_owned(),
                            mfa_enabled: None,
                            locale: None,
                            verified: None,
                            email: None,
                            flags: None,
                            premium_type: None,
                            system: None,
                            public_flags: Some(UserFlags::VERIFIED_BOT_DEVELOPER),
                        },
                    },
                );
                members.insert(
                    UserId(6),
                    Member {
                        deaf: false,
                        guild_id: GuildId(1),
                        hoisted_role: Some(RoleId(6)),
                        joined_at: Some("2020-04-04T04:04:04.000000+00:00".to_owned()),
                        mute: false,
                        nick: Some("chunk".to_owned()),
                        premium_since: None,
                        roles: vec![RoleId(6)],
                        user: User {
                            id: UserId(6),
                            avatar: Some("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_owned()),
                            bot: false,
                            discriminator: "0001".to_owned(),
                            name: "test".to_owned(),
                            mfa_enabled: None,
                            locale: None,
                            verified: None,
                            email: None,
                            flags: None,
                            premium_type: None,
                            system: None,
                            public_flags: None,
                        },
                    },
                );

                members
            },
            nonce: None,
            not_found: Vec::new(),
            presences: {
                let mut presences = HashMap::new();
                presences.insert(
                    UserId(2),
                    Presence {
                        activities: Vec::new(),
                        client_status: ClientStatus {
                            desktop: None,
                            mobile: None,
                            web: Some(Status::Online),
                        },
                        game: None,
                        guild_id: None,
                        nick: None,
                        status: Status::Online,
                        user: UserOrId::UserId { id: UserId(2) },
                    },
                );
                presences.insert(
                    UserId(3),
                    Presence {
                        activities: Vec::new(),
                        client_status: ClientStatus {
                            desktop: None,
                            mobile: None,
                            web: Some(Status::Online),
                        },
                        game: None,
                        guild_id: None,
                        nick: None,
                        status: Status::Online,
                        user: UserOrId::UserId { id: UserId(3) },
                    },
                );
                presences.insert(
                    UserId(5),
                    Presence {
                        activities: Vec::new(),
                        client_status: ClientStatus {
                            desktop: Some(Status::DoNotDisturb),
                            mobile: None,
                            web: None,
                        },
                        game: None,
                        guild_id: None,
                        nick: None,
                        status: Status::DoNotDisturb,
                        user: UserOrId::UserId { id: UserId(5) },
                    },
                );

                presences
            },
        };

        assert_eq!(
            expected,
            serde_json::from_value::<MemberChunk>(input).unwrap()
        );
    }
}
