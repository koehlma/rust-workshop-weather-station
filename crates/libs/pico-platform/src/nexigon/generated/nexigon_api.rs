/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod actor {
    #![doc = "Actor that may be used to perform actions.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nActor that may invoke actions in the system.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum Actor<'a> {
        #[doc = "An anonymous, unauthenticated actor.\n"]
        Anonymous(AnonymousActor),
        #[doc = "The system itself, typically used for internal actions.\n"]
        System(SystemActor),
        #[doc = "A device authenticated via a specific deployment token.\n"]
        Device(DeviceActor<'a>),
        #[doc = "A user typically authenticated via a user session.\n"]
        User(UserActor<'a>),
        #[doc = "A user authenticated via a specific token.\n"]
        UserToken(UserTokenActor<'a>),
        #[doc = "A cluster node.\n"]
        ClusterNode(ClusterNodeActor<'a>),
        #[doc = "An organization API token.\n"]
        OrganizationApiToken(OrganizationApiTokenActor<'a>),
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for Actor<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let name = match reader.tag("actor")? {
                ::core::result::Result::Ok(name) => name,
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match :: sidex_embedded :: json :: case (& name , & ["Anonymous" , "System" , "Device" , "User" , "UserToken" , "ClusterNode" , "OrganizationApiToken"]) { :: core :: option :: Option :: Some (0usize) => :: core :: result :: Result :: Ok (Self :: Anonymous (if < AnonymousActor as :: sidex_embedded :: Decode < 'a >> :: IS_RECORD { < AnonymousActor as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . clone ()) ? } else { < AnonymousActor as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . content ("content") ?) ? })) , :: core :: option :: Option :: Some (1usize) => :: core :: result :: Result :: Ok (Self :: System (if < SystemActor as :: sidex_embedded :: Decode < 'a >> :: IS_RECORD { < SystemActor as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . clone ()) ? } else { < SystemActor as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . content ("content") ?) ? })) , :: core :: option :: Option :: Some (2usize) => :: core :: result :: Result :: Ok (Self :: Device (if < DeviceActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: IS_RECORD { < DeviceActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . clone ()) ? } else { < DeviceActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . content ("content") ?) ? })) , :: core :: option :: Option :: Some (3usize) => :: core :: result :: Result :: Ok (Self :: User (if < UserActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: IS_RECORD { < UserActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . clone ()) ? } else { < UserActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . content ("content") ?) ? })) , :: core :: option :: Option :: Some (4usize) => :: core :: result :: Result :: Ok (Self :: UserToken (if < UserTokenActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: IS_RECORD { < UserTokenActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . clone ()) ? } else { < UserTokenActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . content ("content") ?) ? })) , :: core :: option :: Option :: Some (5usize) => :: core :: result :: Result :: Ok (Self :: ClusterNode (if < ClusterNodeActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: IS_RECORD { < ClusterNodeActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . clone ()) ? } else { < ClusterNodeActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . content ("content") ?) ? })) , :: core :: option :: Option :: Some (6usize) => :: core :: result :: Result :: Ok (Self :: OrganizationApiToken (if < OrganizationApiTokenActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: IS_RECORD { < OrganizationApiTokenActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . clone ()) ? } else { < OrganizationApiTokenActor < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . content ("content") ?) ? })) , _ => :: core :: result :: Result :: Err (value . error (:: sidex_embedded :: DecodeErrorKind :: UnknownCase)) , }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for Actor<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Anonymous(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("Anonymous")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("Anonymous")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::System(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("System")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("System")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::Device(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("Device")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("Device")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::User(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("User")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("User")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::UserToken(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("UserToken")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("UserToken")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::ClusterNode(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("ClusterNode")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("ClusterNode")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::OrganizationApiToken(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("OrganizationApiToken")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("actor")?.str("OrganizationApiToken")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
            }
        }
    }
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nAnonymous actor, typically used for unauthenticated actions.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct AnonymousActor {}
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for AnonymousActor {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[])?;
            ::core::result::Result::Ok(Self {})
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for AnonymousActor {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nSystem actor, typically used for internal actions.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct SystemActor {}
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for SystemActor {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[])?;
            ::core::result::Result::Ok(Self {})
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for SystemActor {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDevice authenticated via a specific deployment token.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceActor<'a> {
        #[doc = "ID of the device.\n"]
        pub device_id: super::devices::DeviceId<'a>,
        #[doc = "ID of the deployment token used for authentication.\n"]
        pub token_id: super::projects::DeploymentTokenId<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceActor<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["deviceId"], &["tokenId"]])?;
            ::core::result::Result::Ok(Self {
                device_id: fields.required(0usize)?,
                token_id: fields.required(1usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceActor<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.device_id, object.field("deviceId")?)?;
            ::sidex_embedded::Encode::encode(&self.token_id, object.field("tokenId")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUser authenticated via a user session.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct UserActor<'a> {
        #[doc = "ID of the user.\n"]
        pub user_id: super::users::UserId<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for UserActor<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["userId"]])?;
            ::core::result::Result::Ok(Self {
                user_id: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for UserActor<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.user_id, object.field("userId")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUser authenticated via a specific token.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct UserTokenActor<'a> {
        #[doc = "ID of the user.\n"]
        pub user_id: super::users::UserId<'a>,
        #[doc = "ID of the user token used for authentication.\n"]
        pub token_id: super::users::UserTokenId<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for UserTokenActor<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["userId"], &["tokenId"]])?;
            ::core::result::Result::Ok(Self {
                user_id: fields.required(0usize)?,
                token_id: fields.required(1usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for UserTokenActor<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.user_id, object.field("userId")?)?;
            ::sidex_embedded::Encode::encode(&self.token_id, object.field("tokenId")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nCluster node.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ClusterNodeActor<'a> {
        #[doc = "ID of the cluster node.\n"]
        pub node_id: super::cluster::ClusterNodeId<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ClusterNodeActor<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["nodeId"]])?;
            ::core::result::Result::Ok(Self {
                node_id: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ClusterNodeActor<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.node_id, object.field("nodeId")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOrganization API token actor.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OrganizationApiTokenActor<'a> {
        #[doc = "ID of the organization.\n"]
        pub organization_id: super::organizations::OrganizationId<'a>,
        #[doc = "ID of the API token.\n"]
        pub token_id: super::organizations::OrganizationApiTokenId<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OrganizationApiTokenActor<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["organizationId"], &["tokenId"]])?;
            ::core::result::Result::Ok(Self {
                organization_id: fields.required(0usize)?,
                token_id: fields.required(1usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OrganizationApiTokenActor<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(
                &self.organization_id,
                object.field("organizationId")?,
            )?;
            ::sidex_embedded::Encode::encode(&self.token_id, object.field("tokenId")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nGet the actor invoking the action.\n\nThe actor identifies the authenticated caller, such as a user, an API token, or a\ndevice. Devices use this action to learn their own device ID.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct GetActorAction {}
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for GetActorAction {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[])?;
            ::core::result::Result::Ok(Self {})
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for GetActorAction {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOutput of getting the actor.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct GetActorOutput<'a> {
        #[doc = "The actor.\n"]
        pub actor: Actor<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for GetActorOutput<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["actor"]])?;
            ::core::result::Result::Ok(Self {
                actor: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for GetActorOutput<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.actor, object.field("actor")?)?;
            ::core::result::Result::Ok(())
        }
    }
}
pub mod cluster {
    #![doc = "Cluster identifier types.\n\nCluster bookkeeping (registration, heartbeats, sweep) is internal and\nlives in the internal API. Only the opaque [`ClusterNodeId`] is part of\nthe public surface, because [`actor::ClusterNodeActor`] and\n[`devices`] connection records reference it.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a cluster node.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ClusterNodeId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ClusterNodeId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ClusterNodeId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for ClusterNodeId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
}
pub mod datetime {
    #![doc = "Time and date types.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nISO 8601 formatted date and time string.\n"]
    pub type Timestamp = ::sidex_embedded::types::Timestamp;
}
pub mod devices {
    #![doc = "Actions and types related to devices.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a device.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for DeviceId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDevice fingerprint.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceFingerprint<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceFingerprint<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceFingerprint<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for DeviceFingerprint<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nID of a device event (unique per device).\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceEventId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceEventId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceEventId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for DeviceEventId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nIssue a token that proves a device's identity to an external system.\n\nThe token is a JWT signed by Nexigon Hub that carries the device ID and the given\nclaims. An external backend validates it with `devices_ValidateDeviceToken`.\n\n**Note: Only use this action for integration with external systems.** The token\ndoes not replace the credentials a device uses to connect to Nexigon Hub.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct IssueDeviceTokenAction<'a> {
        #[doc = "ID of the device.\n"]
        pub device_id: DeviceId<'a>,
        #[doc = "Token validity duration in seconds. Defaults to 600 seconds (10 minutes).\n"]
        pub valid_for_secs: ::core::option::Option<u32>,
        #[doc = "Additional user-defined claims.\n\nNexigon Hub rejects claims larger than the instance's limit, 4 KiB by\ndefault.\n"]
        pub claims: ::core::option::Option<super::json::JsonValue<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for IssueDeviceTokenAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["deviceId"], &["validForSecs"], &["claims"]])?;
            ::core::result::Result::Ok(Self {
                device_id: fields.required(0usize)?,
                valid_for_secs: fields.optional(1usize)?,
                claims: fields.optional(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for IssueDeviceTokenAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.device_id, object.field("deviceId")?)?;
            if let ::core::option::Option::Some(value) = &self.valid_for_secs {
                ::sidex_embedded::Encode::encode(value, object.field("validForSecs")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.claims {
                ::sidex_embedded::Encode::encode(value, object.field("claims")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOutput of issuing a JWT for a device.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct IssueDeviceTokenOutput<'a> {
        #[doc = "Issued JWT.\n"]
        pub jwt: super::jwt::Jwt<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for IssueDeviceTokenOutput<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["jwt"]])?;
            ::core::result::Result::Ok(Self {
                jwt: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for IssueDeviceTokenOutput<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.jwt, object.field("jwt")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nPublish events of a device.\n\nNexigon Hub stores events as a history of device behavior. Each event has an ID\nthat is unique per device. An event published again with the same ID is not\nstored again, so publishing can safely be retried.\n\nNexigon Hub may limit the number of events that a device can store per day and\nper billing period. If storing the new events of a batch would exceed a limit,\nNexigon Hub rejects the whole batch with a `RateLimited` error that states when\nto retry. A batch with more new events than a limit allows fails with an\n`Invalid` error. Events with an already stored ID do not count toward the limits.\n\nNexigon Hub also limits the size of each event, 4 KiB by default, and the number\nof events in a batch, 1000 by default. A batch that exceeds either limit fails with\nan `Invalid` error.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct PublishDeviceEventsAction<'a> {
        #[doc = "ID of the device.\n"]
        pub device_id: DeviceId<'a>,
        #[doc = "Events to publish.\n"]
        pub events: ::sidex_embedded::types::Seq<'a, DeviceEvent<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for PublishDeviceEventsAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["deviceId"], &["events"]])?;
            ::core::result::Result::Ok(Self {
                device_id: fields.required(0usize)?,
                events: fields.required(1usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for PublishDeviceEventsAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.device_id, object.field("deviceId")?)?;
            ::sidex_embedded::Encode::encode(&self.events, object.field("events")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nEvent emitted by a device.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceEvent<'a> {
        #[doc = "ID of the event.\n"]
        pub event_id: DeviceEventId<'a>,
        #[doc = "Category identifying the type of the event. Keep the set of categories small.\n"]
        pub category: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Severity of the event.\n"]
        pub severity: DeviceEventSeverity,
        #[doc = "Event body.\n"]
        pub body: super::json::JsonValue<'a>,
        #[doc = "Event attributes.\n"]
        pub attributes: ::sidex_embedded::types::Map<
            'a,
            ::sidex_embedded::types::Str<'a>,
            super::json::JsonValue<'a>,
        >,
        #[doc = "Timestamp when the event was emitted.\n"]
        pub emitted_at: super::datetime::Timestamp,
        #[doc = "Timestamp when Nexigon Hub received the event. Nexigon Hub sets it and\nignores the value in published events.\n"]
        pub received_at: ::core::option::Option<super::datetime::Timestamp>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceEvent<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["eventId"],
                &["category"],
                &["severity"],
                &["body"],
                &["attributes"],
                &["emittedAt"],
                &["receivedAt"],
            ])?;
            ::core::result::Result::Ok(Self {
                event_id: fields.required(0usize)?,
                category: fields.optional(1usize)?,
                severity: fields.required(2usize)?,
                body: fields.required(3usize)?,
                attributes: fields.required(4usize)?,
                emitted_at: fields.required(5usize)?,
                received_at: fields.optional(6usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceEvent<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.event_id, object.field("eventId")?)?;
            if let ::core::option::Option::Some(value) = &self.category {
                ::sidex_embedded::Encode::encode(value, object.field("category")?)?;
            }
            ::sidex_embedded::Encode::encode(&self.severity, object.field("severity")?)?;
            ::sidex_embedded::Encode::encode(&self.body, object.field("body")?)?;
            ::sidex_embedded::Encode::encode(&self.attributes, object.field("attributes")?)?;
            ::sidex_embedded::Encode::encode(&self.emitted_at, object.field("emittedAt")?)?;
            if let ::core::option::Option::Some(value) = &self.received_at {
                ::sidex_embedded::Encode::encode(value, object.field("receivedAt")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nSeverity of a device event.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum DeviceEventSeverity {
        #[doc = "Tracing event.\n"]
        Trace,
        #[doc = "Debugging event.\n"]
        Debug,
        #[doc = "Informational event.\n"]
        Info,
        #[doc = "Warning event.\n"]
        Warning,
        #[doc = "Error event.\n"]
        Error,
        #[doc = "Critical event.\n"]
        Critical,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceEventSeverity {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match (
                ::sidex_embedded::json::case(
                    &name,
                    &["Trace", "Debug", "Info", "Warning", "Error", "Critical"],
                ),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Trace)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Debug)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Info)
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Warning)
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Error)
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Critical)
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for DeviceEventSeverity {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Trace => writer.str("Trace"),
                Self::Debug => writer.str("Debug"),
                Self::Info => writer.str("Info"),
                Self::Warning => writer.str("Warning"),
                Self::Error => writer.str("Error"),
                Self::Critical => writer.str("Critical"),
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nSet a property of a device.\n\nA property is a named JSON value of a device. Setting a property replaces the\ndevice's own value and increments the property's version. With an expected version,\nthe property is only updated if its version still matches (compare-and-swap).\nDevices cannot change protected properties.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct SetDevicePropertyAction<'a> {
        #[doc = "ID of the device.\n"]
        pub device_id: DeviceId<'a>,
        #[doc = "Name of the property.\n"]
        pub name: ::sidex_embedded::types::Str<'a>,
        #[doc = "Value of the property.\n\nNexigon Hub rejects values larger than the instance's limit, 64 KiB by\ndefault.\n"]
        pub value: super::json::JsonValue<'a>,
        #[doc = "Indicates whether the property should be protected.\n"]
        pub protected: ::core::option::Option<bool>,
        #[doc = "If set, the property is only updated when its current version matches\nthis value (compare-and-swap). Returns `VersionConflict` when the\nversion does not match, or `Deleted` when the property no longer exists.\n"]
        pub expected_version: ::core::option::Option<u64>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for SetDevicePropertyAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["deviceId"],
                &["name"],
                &["value"],
                &["protected"],
                &["expectedVersion"],
            ])?;
            ::core::result::Result::Ok(Self {
                device_id: fields.required(0usize)?,
                name: fields.required(1usize)?,
                value: fields.required(2usize)?,
                protected: fields.optional(3usize)?,
                expected_version: fields.optional(4usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for SetDevicePropertyAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.device_id, object.field("deviceId")?)?;
            ::sidex_embedded::Encode::encode(&self.name, object.field("name")?)?;
            ::sidex_embedded::Encode::encode(&self.value, object.field("value")?)?;
            if let ::core::option::Option::Some(value) = &self.protected {
                ::sidex_embedded::Encode::encode(value, object.field("protected")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.expected_version {
                ::sidex_embedded::Encode::encode(value, object.field("expectedVersion")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOutput of setting a device property.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum SetDevicePropertyOutput {
        #[doc = "The property was set successfully.\n"]
        Set(SetDevicePropertySuccess),
        #[doc = "The property was not updated because its version did not match the\nexpected version (compare-and-swap conflict).\n"]
        VersionConflict(SetDevicePropertyVersionConflict),
        #[doc = "The property was not updated because it has been deleted while an\nexpected version was specified.\n"]
        Deleted,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for SetDevicePropertyOutput {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let name = match reader.tag("result")? {
                ::core::result::Result::Ok(name) => name,
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match :: sidex_embedded :: json :: case (& name , & ["Set" , "VersionConflict" , "Deleted"]) { :: core :: option :: Option :: Some (0usize) => :: core :: result :: Result :: Ok (Self :: Set (if < SetDevicePropertySuccess as :: sidex_embedded :: Decode < 'a >> :: IS_RECORD { < SetDevicePropertySuccess as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . clone ()) ? } else { < SetDevicePropertySuccess as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . content ("content") ?) ? })) , :: core :: option :: Option :: Some (1usize) => :: core :: result :: Result :: Ok (Self :: VersionConflict (if < SetDevicePropertyVersionConflict as :: sidex_embedded :: Decode < 'a >> :: IS_RECORD { < SetDevicePropertyVersionConflict as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . clone ()) ? } else { < SetDevicePropertyVersionConflict as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . content ("content") ?) ? })) , :: core :: option :: Option :: Some (2usize) => :: core :: result :: Result :: Ok (Self :: Deleted) , _ => :: core :: result :: Result :: Err (value . error (:: sidex_embedded :: DecodeErrorKind :: UnknownCase)) , }
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for SetDevicePropertyOutput {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Set(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("result")?.str("Set")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("result")?.str("Set")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::VersionConflict(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("result")?.str("VersionConflict")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("result")?.str("VersionConflict")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::Deleted => {
                    let mut object = writer.object()?;
                    object.field("result")?.str("Deleted")?;
                    object.end()
                }
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDetails of a successful property update.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct SetDevicePropertySuccess {
        #[doc = "The new version of the property.\n\nAbsent in responses of Nexigon Hub 2026.3 and earlier.\n"]
        pub version: ::core::option::Option<u64>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for SetDevicePropertySuccess {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["version"]])?;
            ::core::result::Result::Ok(Self {
                version: fields.optional(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for SetDevicePropertySuccess {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            if let ::core::option::Option::Some(value) = &self.version {
                ::sidex_embedded::Encode::encode(value, object.field("version")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDetails of a version conflict when setting a device property.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct SetDevicePropertyVersionConflict {
        #[doc = "The current version of the property.\n"]
        pub current_version: u64,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for SetDevicePropertyVersionConflict {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["currentVersion"]])?;
            ::core::result::Result::Ok(Self {
                current_version: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for SetDevicePropertyVersionConflict {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(
                &self.current_version,
                object.field("currentVersion")?,
            )?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nGet a property of a device.\n\nThe value combines the values of the device's profiles with the device's own\nvalue.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct GetDevicePropertyAction<'a> {
        #[doc = "ID of the device.\n"]
        pub device_id: DeviceId<'a>,
        #[doc = "Name of the property.\n"]
        pub name: ::sidex_embedded::types::Str<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for GetDevicePropertyAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["deviceId"], &["name"]])?;
            ::core::result::Result::Ok(Self {
                device_id: fields.required(0usize)?,
                name: fields.required(1usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for GetDevicePropertyAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.device_id, object.field("deviceId")?)?;
            ::sidex_embedded::Encode::encode(&self.name, object.field("name")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOutput of getting a property of a device.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum GetDevicePropertyOutput<'a> {
        #[doc = "The property was not found.\n"]
        NotFound,
        #[doc = "The property was found.\n"]
        Found(DeviceProperty<'a>),
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for GetDevicePropertyOutput<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let name = match reader.tag("result")? {
                ::core::result::Result::Ok(name) => name,
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match ::sidex_embedded::json::case(&name, &["NotFound", "Found"]) {
                ::core::option::Option::Some(0usize) => ::core::result::Result::Ok(Self::NotFound),
                ::core::option::Option::Some(1usize) => ::core::result::Result::Ok(Self::Found(
                    if <DeviceProperty<'a> as ::sidex_embedded::Decode<'a>>::IS_RECORD {
                        <DeviceProperty<'a> as ::sidex_embedded::Decode<'a>>::decode(
                            &mut value.clone(),
                        )?
                    } else {
                        <DeviceProperty<'a> as ::sidex_embedded::Decode<'a>>::decode(
                            &mut value.content("content")?,
                        )?
                    },
                )),
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for GetDevicePropertyOutput<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::NotFound => {
                    let mut object = writer.object()?;
                    object.field("result")?.str("NotFound")?;
                    object.end()
                }
                Self::Found(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("result")?.str("Found")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("result")?.str("Found")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDevice property.\n\nThe value results from applying the values of the device's profiles in\norder, followed by the device's own value, each as a JSON merge patch\n(RFC 7396). Without profiles setting the property, it is the device's own\nvalue.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceProperty<'a> {
        #[doc = "Name of the property.\n"]
        pub name: ::sidex_embedded::types::Str<'a>,
        #[doc = "Value of the property.\n"]
        pub value: super::json::JsonValue<'a>,
        #[doc = "Indicates whether the property is protected.\n"]
        pub protected: bool,
        #[doc = "Version number.\n\nIncreases with every write to the device's own value, even with an\nunchanged value, and with every change of the device's profiles that\naffects the property.\n"]
        pub version: u64,
        #[doc = "Last time the property was updated.\n"]
        pub last_updated: super::datetime::Timestamp,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceProperty<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["name"],
                &["value"],
                &["protected"],
                &["version"],
                &["lastUpdated"],
            ])?;
            ::core::result::Result::Ok(Self {
                name: fields.required(0usize)?,
                value: fields.required(1usize)?,
                protected: fields.required(2usize)?,
                version: fields.required(3usize)?,
                last_updated: fields.required(4usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceProperty<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.name, object.field("name")?)?;
            ::sidex_embedded::Encode::encode(&self.value, object.field("value")?)?;
            ::sidex_embedded::Encode::encode(&self.protected, object.field("protected")?)?;
            ::sidex_embedded::Encode::encode(&self.version, object.field("version")?)?;
            ::sidex_embedded::Encode::encode(&self.last_updated, object.field("lastUpdated")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Unstable (feature `device_commands`).**\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nStatus of a device command invocation.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum DeviceCommandStatus {
        #[doc = "Command completed successfully.\n"]
        Ok,
        #[doc = "Command failed.\n"]
        Error,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceCommandStatus {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match (
                ::sidex_embedded::json::case(&name, &["Ok", "Error"]),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Ok)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Error)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for DeviceCommandStatus {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Ok => writer.str("Ok"),
                Self::Error => writer.str("Error"),
            }
        }
    }
    #[doc = "**Unstable (feature `device_commands`).**\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nHub-to-device frame.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum DeviceCommandHubFrame<'a> {
        #[doc = "Invoke a command.\n"]
        Invoke(DeviceCommandInvokeData<'a>),
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceCommandHubFrame<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let name = match reader.tag("type")? {
                ::core::result::Result::Ok(name) => name,
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match ::sidex_embedded::json::case(&name, &["Invoke"]) {
                ::core::option::Option::Some(0usize) => ::core::result::Result::Ok(Self::Invoke(
                    if <DeviceCommandInvokeData<'a> as ::sidex_embedded::Decode<'a>>::IS_RECORD {
                        <DeviceCommandInvokeData<'a> as ::sidex_embedded::Decode<'a>>::decode(
                            &mut value.clone(),
                        )?
                    } else {
                        <DeviceCommandInvokeData<'a> as ::sidex_embedded::Decode<'a>>::decode(
                            &mut value.content("content")?,
                        )?
                    },
                )),
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceCommandHubFrame<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Invoke(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("type")?.str("Invoke")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("type")?.str("Invoke")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
            }
        }
    }
    #[doc = "**Unstable (feature `device_commands`).**\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nCommand invocation parameters.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceCommandInvokeData<'a> {
        #[doc = "Command name.\n"]
        pub command: ::sidex_embedded::types::Str<'a>,
        #[doc = "Command input.\n"]
        pub input: super::json::JsonValue<'a>,
        #[doc = "Stream log output to Nexigon Hub during execution (defaults to false).\n"]
        pub stream_log: ::core::option::Option<bool>,
        #[doc = "Timeout in seconds.\n"]
        pub timeout_secs: ::core::option::Option<u32>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceCommandInvokeData<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields =
                reader.record(&[&["command"], &["input"], &["streamLog"], &["timeoutSecs"]])?;
            ::core::result::Result::Ok(Self {
                command: fields.required(0usize)?,
                input: fields.required(1usize)?,
                stream_log: fields.optional(2usize)?,
                timeout_secs: fields.optional(3usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceCommandInvokeData<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.command, object.field("command")?)?;
            ::sidex_embedded::Encode::encode(&self.input, object.field("input")?)?;
            if let ::core::option::Option::Some(value) = &self.stream_log {
                ::sidex_embedded::Encode::encode(value, object.field("streamLog")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.timeout_secs {
                ::sidex_embedded::Encode::encode(value, object.field("timeoutSecs")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Unstable (feature `device_commands`).**\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDevice-to-hub frame.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum DeviceCommandDeviceFrame<'a> {
        #[doc = "Diagnostic log lines from the command.\n"]
        Log(DeviceCommandLogData<'a>),
        #[doc = "Command finished executing.\n"]
        Done(DeviceCommandDoneData<'a>),
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceCommandDeviceFrame<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let name = match reader.tag("type")? {
                ::core::result::Result::Ok(name) => name,
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match ::sidex_embedded::json::case(&name, &["Log", "Done"]) {
                ::core::option::Option::Some(0usize) => ::core::result::Result::Ok(Self::Log(
                    if <DeviceCommandLogData<'a> as ::sidex_embedded::Decode<'a>>::IS_RECORD {
                        <DeviceCommandLogData<'a> as ::sidex_embedded::Decode<'a>>::decode(
                            &mut value.clone(),
                        )?
                    } else {
                        <DeviceCommandLogData<'a> as ::sidex_embedded::Decode<'a>>::decode(
                            &mut value.content("content")?,
                        )?
                    },
                )),
                ::core::option::Option::Some(1usize) => ::core::result::Result::Ok(Self::Done(
                    if <DeviceCommandDoneData<'a> as ::sidex_embedded::Decode<'a>>::IS_RECORD {
                        <DeviceCommandDoneData<'a> as ::sidex_embedded::Decode<'a>>::decode(
                            &mut value.clone(),
                        )?
                    } else {
                        <DeviceCommandDoneData<'a> as ::sidex_embedded::Decode<'a>>::decode(
                            &mut value.content("content")?,
                        )?
                    },
                )),
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceCommandDeviceFrame<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Log(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("type")?.str("Log")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("type")?.str("Log")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::Done(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("type")?.str("Done")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("type")?.str("Done")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
            }
        }
    }
    #[doc = "**Unstable (feature `device_commands`).**\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nLog data.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceCommandLogData<'a> {
        #[doc = "Lines of diagnostic output from the command.\n"]
        pub lines: ::sidex_embedded::types::Seq<'a, ::sidex_embedded::types::Str<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceCommandLogData<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["lines"]])?;
            ::core::result::Result::Ok(Self {
                lines: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceCommandLogData<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.lines, object.field("lines")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Unstable (feature `device_commands`).**\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nCommand completion.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceCommandDoneData<'a> {
        #[doc = "Completion status.\n"]
        pub status: DeviceCommandStatus,
        #[doc = "Command output.\n"]
        pub output: ::core::option::Option<super::json::JsonValue<'a>>,
        #[doc = "Error message (present on failure).\n"]
        pub error: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Last lines of the command's diagnostic log output.\n"]
        pub log_tail: ::sidex_embedded::types::Seq<'a, ::sidex_embedded::types::Str<'a>>,
        #[doc = "Execution duration in milliseconds.\n"]
        pub duration_ms: u64,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceCommandDoneData<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["status"],
                &["output"],
                &["error"],
                &["logTail"],
                &["durationMs"],
            ])?;
            ::core::result::Result::Ok(Self {
                status: fields.required(0usize)?,
                output: fields.optional(1usize)?,
                error: fields.optional(2usize)?,
                log_tail: fields.required(3usize)?,
                duration_ms: fields.required(4usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceCommandDoneData<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.status, object.field("status")?)?;
            if let ::core::option::Option::Some(value) = &self.output {
                ::sidex_embedded::Encode::encode(value, object.field("output")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.error {
                ::sidex_embedded::Encode::encode(value, object.field("error")?)?;
            }
            ::sidex_embedded::Encode::encode(&self.log_tail, object.field("logTail")?)?;
            ::sidex_embedded::Encode::encode(&self.duration_ms, object.field("durationMs")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a device operation.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceOperationId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceOperationId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceOperationId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for DeviceOperationId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique claim for one lease of a device operation work item.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceOperationWorkClaimId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceOperationWorkClaimId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceOperationWorkClaimId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for DeviceOperationWorkClaimId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nWork item returned to a device when it polls.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceOperationWorkItem<'a> {
        #[doc = "ID of the device operation the work belongs to.\n"]
        pub device_operation_id: DeviceOperationId<'a>,
        #[doc = "Claim that must accompany reports for this delivery of the work.\n\nA later poll after the lease expires replaces this claim, fencing reports from\nthe older worker.\n"]
        pub claim_id: DeviceOperationWorkClaimId<'a>,
        #[doc = "Name of the standalone or fleet operation that defined this work.\n"]
        pub operation_name: ::sidex_embedded::types::Str<'a>,
        #[doc = "Zero-based index of the step within the device operation. Pass it to\n`devices_ReportOperationStep` with the claim.\n"]
        pub step_index: u32,
        #[doc = "Step to run.\n"]
        pub step: DeviceOperationWorkStep<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceOperationWorkItem<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["deviceOperationId"],
                &["claimId"],
                &["operationName"],
                &["stepIndex"],
                &["step"],
            ])?;
            ::core::result::Result::Ok(Self {
                device_operation_id: fields.required(0usize)?,
                claim_id: fields.required(1usize)?,
                operation_name: fields.required(2usize)?,
                step_index: fields.required(3usize)?,
                step: fields.required(4usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceOperationWorkItem<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(
                &self.device_operation_id,
                object.field("deviceOperationId")?,
            )?;
            ::sidex_embedded::Encode::encode(&self.claim_id, object.field("claimId")?)?;
            ::sidex_embedded::Encode::encode(&self.operation_name, object.field("operationName")?)?;
            ::sidex_embedded::Encode::encode(&self.step_index, object.field("stepIndex")?)?;
            ::sidex_embedded::Encode::encode(&self.step, object.field("step")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDevice-executable operation step.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum DeviceOperationWorkStep<'a> {
        #[doc = "Run an on-device command and report its result.\n"]
        DeviceCommand(DeviceOperationCommandWork<'a>),
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceOperationWorkStep<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let name = match reader.tag("kind")? {
                ::core::result::Result::Ok(name) => name,
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match :: sidex_embedded :: json :: case (& name , & ["DeviceCommand"]) { :: core :: option :: Option :: Some (0usize) => :: core :: result :: Result :: Ok (Self :: DeviceCommand (if < DeviceOperationCommandWork < 'a , > as :: sidex_embedded :: Decode < 'a >> :: IS_RECORD { < DeviceOperationCommandWork < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . clone ()) ? } else { < DeviceOperationCommandWork < 'a , > as :: sidex_embedded :: Decode < 'a >> :: decode (& mut value . content ("content") ?) ? })) , _ => :: core :: result :: Result :: Err (value . error (:: sidex_embedded :: DecodeErrorKind :: UnknownCase)) , }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceOperationWorkStep<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::DeviceCommand(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("kind")?.str("DeviceCommand")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("kind")?.str("DeviceCommand")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nCommand that a device runs for a device operation.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceOperationCommandWork<'a> {
        #[doc = "Command name from the device command manifest.\n"]
        pub command: ::sidex_embedded::types::Str<'a>,
        #[doc = "Command input.\n"]
        pub input: super::json::JsonValue<'a>,
        #[doc = "Execution timeout of the command handler, in seconds. Defaults to one\nhour.\n"]
        pub timeout_secs: ::core::option::Option<u32>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceOperationCommandWork<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["command"], &["input"], &["timeoutSecs"]])?;
            ::core::result::Result::Ok(Self {
                command: fields.required(0usize)?,
                input: fields.required(1usize)?,
                timeout_secs: fields.optional(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceOperationCommandWork<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.command, object.field("command")?)?;
            ::sidex_embedded::Encode::encode(&self.input, object.field("input")?)?;
            if let ::core::option::Option::Some(value) = &self.timeout_secs {
                ::sidex_embedded::Encode::encode(value, object.field("timeoutSecs")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nKind of device operation work to return from a poll.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum DeviceOperationWorkKind {
        #[doc = "Command work, which Nexigon Agent runs with its device commands.\n"]
        DeviceCommand,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceOperationWorkKind {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match (
                ::sidex_embedded::json::case(&name, &["DeviceCommand"]),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::DeviceCommand)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for DeviceOperationWorkKind {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::DeviceCommand => writer.str("DeviceCommand"),
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nReport from a device after executing a polled step.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceOperationStepReport<'a> {
        #[doc = "Result status.\n"]
        pub status: DeviceOperationStepReportStatus,
        #[doc = "Command output.\n\nNexigon Hub stores outputs up to the instance's limit, 64 KiB by default,\nand only the size of larger outputs.\n"]
        pub output: ::core::option::Option<super::json::JsonValue<'a>>,
        #[doc = "Error message when the step failed.\n"]
        pub error: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceOperationStepReport<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["status"], &["output"], &["error"]])?;
            ::core::result::Result::Ok(Self {
                status: fields.required(0usize)?,
                output: fields.optional(1usize)?,
                error: fields.optional(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceOperationStepReport<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.status, object.field("status")?)?;
            if let ::core::option::Option::Some(value) = &self.output {
                ::sidex_embedded::Encode::encode(value, object.field("output")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.error {
                ::sidex_embedded::Encode::encode(value, object.field("error")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDevice-reported step result status.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum DeviceOperationStepReportStatus {
        #[doc = "Step succeeded.\n"]
        Succeeded,
        #[doc = "Step failed.\n"]
        Failed,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceOperationStepReportStatus {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match (
                ::sidex_embedded::json::case(&name, &["Succeeded", "Failed"]),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Succeeded)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Failed)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for DeviceOperationStepReportStatus {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Succeeded => writer.str("Succeeded"),
                Self::Failed => writer.str("Failed"),
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nClaim device operation work for a device.\n\nLeases pending work items to the device atomically, so no other worker receives\nthem at the same time. Report the outcome of each item with\n`devices_ReportOperationStep`, passing the item's claim ID. Only the device itself\ncan claim its work.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ClaimDeviceOperationWorkAction<'a> {
        #[doc = "ID of the device claiming work.\n"]
        pub device_id: DeviceId<'a>,
        #[doc = "Maximum number of work items to claim. Defaults to 1.\n"]
        pub limit: ::core::option::Option<u32>,
        #[doc = "Work kinds to return. Defaults to all device-executable work.\n"]
        pub kinds:
            ::core::option::Option<::sidex_embedded::types::Seq<'a, DeviceOperationWorkKind>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ClaimDeviceOperationWorkAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["deviceId"], &["limit"], &["kinds"]])?;
            ::core::result::Result::Ok(Self {
                device_id: fields.required(0usize)?,
                limit: fields.optional(1usize)?,
                kinds: fields.optional(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ClaimDeviceOperationWorkAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.device_id, object.field("deviceId")?)?;
            if let ::core::option::Option::Some(value) = &self.limit {
                ::sidex_embedded::Encode::encode(value, object.field("limit")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.kinds {
                ::sidex_embedded::Encode::encode(value, object.field("kinds")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOutput of claiming operation work for a device.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ClaimDeviceOperationWorkOutput<'a> {
        #[doc = "Work items leased to the device.\n"]
        pub work: ::sidex_embedded::types::Seq<'a, DeviceOperationWorkItem<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ClaimDeviceOperationWorkOutput<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["work"]])?;
            ::core::result::Result::Ok(Self {
                work: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ClaimDeviceOperationWorkOutput<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.work, object.field("work")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nReport the result of a device operation step.\n\nRepeating a report after it was already applied is a successful no-op. A report whose\nclaim has been superseded is also ignored, so an expired worker cannot overwrite the\ncurrent attempt.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ReportDeviceOperationStepAction<'a> {
        #[doc = "ID of the reporting device.\n"]
        pub device_id: DeviceId<'a>,
        #[doc = "ID of the operation.\n"]
        pub device_operation_id: DeviceOperationId<'a>,
        #[doc = "Step index being reported.\n"]
        pub step_index: u32,
        #[doc = "Claim returned by the poll that delivered this work item.\n"]
        pub claim_id: DeviceOperationWorkClaimId<'a>,
        #[doc = "Reported step outcome.\n"]
        pub report: DeviceOperationStepReport<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ReportDeviceOperationStepAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["deviceId"],
                &["deviceOperationId"],
                &["stepIndex"],
                &["claimId"],
                &["report"],
            ])?;
            ::core::result::Result::Ok(Self {
                device_id: fields.required(0usize)?,
                device_operation_id: fields.required(1usize)?,
                step_index: fields.required(2usize)?,
                claim_id: fields.required(3usize)?,
                report: fields.required(4usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ReportDeviceOperationStepAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.device_id, object.field("deviceId")?)?;
            ::sidex_embedded::Encode::encode(
                &self.device_operation_id,
                object.field("deviceOperationId")?,
            )?;
            ::sidex_embedded::Encode::encode(&self.step_index, object.field("stepIndex")?)?;
            ::sidex_embedded::Encode::encode(&self.claim_id, object.field("claimId")?)?;
            ::sidex_embedded::Encode::encode(&self.report, object.field("report")?)?;
            ::core::result::Result::Ok(())
        }
    }
}
pub mod digest {
    #![doc = "Hash digests.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nA hash digest.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct Digest<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for Digest<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for Digest<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for Digest<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
}
pub mod errors {
    #![doc = "Error types.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nKind of an action error.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum ActionErrorKind {
        #[doc = "Invalid action.\n"]
        Invalid,
        #[doc = "Resource not found in the selected organization.\n"]
        NotFound,
        #[doc = "Forbidden action.\n"]
        Forbidden,
        #[doc = "The action requires inline credential re-verification.\n"]
        OperationAuthorizationRequired,
        #[doc = "A scope policy requires authenticator enrollment before access.\n"]
        MfaEnrollmentRequired,
        #[doc = "Internal server error.\n"]
        Internal,
        #[doc = "Rate limit exceeded.\n"]
        RateLimited,
        #[doc = "Service or dependency capacity is temporarily unavailable.\n"]
        Unavailable,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ActionErrorKind {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match (
                ::sidex_embedded::json::case(
                    &name,
                    &[
                        "Invalid",
                        "NotFound",
                        "Forbidden",
                        "OperationAuthorizationRequired",
                        "MfaEnrollmentRequired",
                        "Internal",
                        "RateLimited",
                        "Unavailable",
                    ],
                ),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Invalid)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::NotFound)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Forbidden)
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::OperationAuthorizationRequired)
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::MfaEnrollmentRequired)
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Internal)
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(6usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::RateLimited)
                }
                (::core::option::Option::Some(6usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(7usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Unavailable)
                }
                (::core::option::Option::Some(7usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for ActionErrorKind {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Invalid => writer.str("Invalid"),
                Self::NotFound => writer.str("NotFound"),
                Self::Forbidden => writer.str("Forbidden"),
                Self::OperationAuthorizationRequired => {
                    writer.str("OperationAuthorizationRequired")
                }
                Self::MfaEnrollmentRequired => writer.str("MfaEnrollmentRequired"),
                Self::Internal => writer.str("Internal"),
                Self::RateLimited => writer.str("RateLimited"),
                Self::Unavailable => writer.str("Unavailable"),
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nAction error.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ActionError<'a> {
        #[doc = "Kind of the error.\n"]
        pub kind: ActionErrorKind,
        #[doc = "Message of the error.\n"]
        pub message: ::sidex_embedded::types::Str<'a>,
        #[doc = "Minimum number of seconds to wait before retrying.\n"]
        pub retry_after_seconds: ::core::option::Option<u64>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ActionError<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["kind"], &["message"], &["retryAfterSeconds"]])?;
            ::core::result::Result::Ok(Self {
                kind: fields.required(0usize)?,
                message: fields.required(1usize)?,
                retry_after_seconds: fields.optional(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ActionError<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.kind, object.field("kind")?)?;
            ::sidex_embedded::Encode::encode(&self.message, object.field("message")?)?;
            if let ::core::option::Option::Some(value) = &self.retry_after_seconds {
                ::sidex_embedded::Encode::encode(value, object.field("retryAfterSeconds")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nAction result.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum ActionResult<'a, T: ::sidex_embedded::Decode<'a> + ::core::clone::Clone> {
        Ok(T),
        Error(ActionError<'a>),
    }
    #[automatically_derived]
    impl<'a, T: ::sidex_embedded::Decode<'a> + ::core::clone::Clone> ::sidex_embedded::Decode<'a>
        for ActionResult<'a, T>
    {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let name = match reader.tag("result")? {
                ::core::result::Result::Ok(name) => name,
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match ::sidex_embedded::json::case(&name, &["Ok", "Error"]) {
                ::core::option::Option::Some(0usize) => ::core::result::Result::Ok(Self::Ok(
                    <T as ::sidex_embedded::Decode<'a>>::decode(&mut value.content("value")?)?,
                )),
                ::core::option::Option::Some(1usize) => ::core::result::Result::Ok(Self::Error(
                    <ActionError<'a> as ::sidex_embedded::Decode<'a>>::decode(
                        &mut value.content("error")?,
                    )?,
                )),
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl<'a, T: ::sidex_embedded::Decode<'a> + ::core::clone::Clone + ::sidex_embedded::Encode>
        ::sidex_embedded::Encode for ActionResult<'a, T>
    {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Ok(payload) => {
                    let mut object = writer.object()?;
                    object.field("result")?.str("Ok")?;
                    ::sidex_embedded::Encode::encode(payload, object.field("value")?)?;
                    object.end()
                }
                Self::Error(payload) => {
                    let mut object = writer.object()?;
                    object.field("result")?.str("Error")?;
                    ::sidex_embedded::Encode::encode(payload, object.field("error")?)?;
                    object.end()
                }
            }
        }
    }
}
pub mod json {
    #![doc = "JSON value type.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nJSON value.\n"]
    pub type JsonValue<'a> = ::sidex_embedded::types::Json<'a>;
}
pub mod jwt {
    #![doc = ""]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nJSON Web Token (JWT).\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct Jwt<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for Jwt<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for Jwt<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for Jwt<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
}
pub mod organizations {
    #![doc = "Actions and types related to organizations.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of an organization\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OrganizationId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OrganizationId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OrganizationId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for OrganizationId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of an organization API token.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OrganizationApiTokenId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OrganizationApiTokenId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OrganizationApiTokenId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for OrganizationApiTokenId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
}
pub mod outputs {
    #![doc = "Additional output types for actions.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nEmpty output.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct Empty {}
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for Empty {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[])?;
            ::core::result::Result::Ok(Self {})
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for Empty {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::core::result::Result::Ok(())
        }
    }
}
pub mod projects {
    #![doc = "Actions and types related to projects.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a project.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ProjectId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ProjectId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ProjectId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for ProjectId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a deployment token.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeploymentTokenId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeploymentTokenId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeploymentTokenId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for DeploymentTokenId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDeployment token.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeploymentToken<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeploymentToken<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeploymentToken<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for DeploymentToken<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nRedeem a device pairing key.\n\nCreates a device in the pairing key's project and issues a deployment token bound\nto it. The device uses the token to connect to Nexigon Hub. This action does not\nrequire authentication; the pairing key authorizes it.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct RedeemDevicePairingKeyAction<'a> {
        #[doc = "Pairing key as shown to the user.\n"]
        pub pairing_key: ::sidex_embedded::types::Str<'a>,
        #[doc = "Fingerprint generated by the device.\n"]
        pub fingerprint: super::devices::DeviceFingerprint<'a>,
        #[doc = "Optional display name for the device.\n"]
        pub device_name: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for RedeemDevicePairingKeyAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["pairingKey"], &["fingerprint"], &["deviceName"]])?;
            ::core::result::Result::Ok(Self {
                pairing_key: fields.required(0usize)?,
                fingerprint: fields.required(1usize)?,
                device_name: fields.optional(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for RedeemDevicePairingKeyAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.pairing_key, object.field("pairingKey")?)?;
            ::sidex_embedded::Encode::encode(&self.fingerprint, object.field("fingerprint")?)?;
            if let ::core::option::Option::Some(value) = &self.device_name {
                ::sidex_embedded::Encode::encode(value, object.field("deviceName")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOutput of redeeming a device pairing key.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct RedeemDevicePairingKeyOutput<'a> {
        #[doc = "Project the device was created in.\n"]
        pub project_id: ProjectId<'a>,
        #[doc = "Device created for this pairing key.\n"]
        pub device_id: super::devices::DeviceId<'a>,
        #[doc = "ID of the issued device-bound deployment token.\n"]
        pub token_id: DeploymentTokenId<'a>,
        #[doc = "Device-bound deployment token. This is only returned once.\n"]
        pub token: DeploymentToken<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for RedeemDevicePairingKeyOutput<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields =
                reader.record(&[&["projectId"], &["deviceId"], &["tokenId"], &["token"]])?;
            ::core::result::Result::Ok(Self {
                project_id: fields.required(0usize)?,
                device_id: fields.required(1usize)?,
                token_id: fields.required(2usize)?,
                token: fields.required(3usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for RedeemDevicePairingKeyOutput<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.project_id, object.field("projectId")?)?;
            ::sidex_embedded::Encode::encode(&self.device_id, object.field("deviceId")?)?;
            ::sidex_embedded::Encode::encode(&self.token_id, object.field("tokenId")?)?;
            ::sidex_embedded::Encode::encode(&self.token, object.field("token")?)?;
            ::core::result::Result::Ok(())
        }
    }
}
pub mod properties {
    #![doc = "Types related to device properties.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOTA update configuration (`dev.nexigon.ota.config`).\n\nDesired state of a device. Integrations apply the property to defaults\nbuilt into the image as a JSON merge patch (RFC 7396): objects merge\nrecursively, `null` removes a default, and any other value, including an\narray, replaces the default.\n\n[`OtaUpdateStatus`] describes how the contract evolves.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OtaUpdateConfig<'a> {
        #[doc = "Format version of the property.\n\nDefaults to 1, the format that this version of the contract defines.\n"]
        pub format_version: ::core::option::Option<u32>,
        #[doc = "Version path in the format `<repository>/<package>/<version>`.\n\n`<version>` is a tag or a version ID. A tag tracks the version the tag\npoints to. A version ID pins a single version.\n"]
        pub path: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Filename of the asset that the device installs from a version.\n\nDefaults to `<system>.<extension>`, where `<system>` is the name of the\nrunning system and the update system determines the extension, such as\n`rugixb` for Rugix Ctrl and `raucb` for RAUC.\n"]
        pub asset: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Hold the device at its current version.\n\nWhile set, the device starts no new update. An update in progress\ncompletes, including its activation. Defaults to false.\n"]
        pub hold: ::core::option::Option<bool>,
        #[doc = "Maintenance window restricting when updates are activated.\n\nUpdates can be activated at any time when absent.\n"]
        pub window: ::core::option::Option<OtaUpdateWindow<'a>>,
        #[doc = "Retry policy for failed update attempts.\n"]
        pub retry: ::core::option::Option<OtaUpdateRetryPolicy>,
        #[doc = "Spreading of updates across devices.\n\nDevices update without waiting when absent.\n"]
        pub spread: ::core::option::Option<OtaUpdateSpread>,
        #[doc = "Rollout number.\n\nIntegrations ignore its value. Changing it changes the effective\nconfiguration, which starts a new round of attempts on devices whose\nattempts failed.\n"]
        pub rollout: ::core::option::Option<u32>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateConfig<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["formatVersion"],
                &["path"],
                &["asset"],
                &["hold"],
                &["window"],
                &["retry"],
                &["spread"],
                &["rollout"],
            ])?;
            ::core::result::Result::Ok(Self {
                format_version: fields.optional(0usize)?,
                path: fields.optional(1usize)?,
                asset: fields.optional(2usize)?,
                hold: fields.optional(3usize)?,
                window: fields.optional(4usize)?,
                retry: fields.optional(5usize)?,
                spread: fields.optional(6usize)?,
                rollout: fields.optional(7usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdateConfig<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            if let ::core::option::Option::Some(value) = &self.format_version {
                ::sidex_embedded::Encode::encode(value, object.field("formatVersion")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.path {
                ::sidex_embedded::Encode::encode(value, object.field("path")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.asset {
                ::sidex_embedded::Encode::encode(value, object.field("asset")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.hold {
                ::sidex_embedded::Encode::encode(value, object.field("hold")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.window {
                ::sidex_embedded::Encode::encode(value, object.field("window")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.retry {
                ::sidex_embedded::Encode::encode(value, object.field("retry")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.spread {
                ::sidex_embedded::Encode::encode(value, object.field("spread")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.rollout {
                ::sidex_embedded::Encode::encode(value, object.field("rollout")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nMaintenance window in the device's local time.\n\nThe window restricts activation, the step that interrupts the device's\noperation, such as the reboot into the new version. Integrations may\ndownload and install an update outside the window if doing so does not\ninterrupt operation.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OtaUpdateWindow<'a> {
        #[doc = "Time ranges in which updates can be activated.\n\nAn empty list never allows activation.\n"]
        pub ranges: ::sidex_embedded::types::Seq<'a, OtaUpdateWindowRange<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateWindow<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["ranges"]])?;
            ::core::result::Result::Ok(Self {
                ranges: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdateWindow<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.ranges, object.field("ranges")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nRecurring time range of a maintenance window.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OtaUpdateWindowRange<'a> {
        #[doc = "Days on which the range starts.\n\nDefaults to every day.\n"]
        pub days: ::core::option::Option<::sidex_embedded::types::Seq<'a, Weekday>>,
        #[doc = "Start of the range (inclusive).\n"]
        pub start: TimeOfDay<'a>,
        #[doc = "End of the range (exclusive).\n\nAn end at or before the start lies on the following day. A range whose\nend equals its start therefore spans 24 hours.\n"]
        pub end: TimeOfDay<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateWindowRange<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["days"], &["start"], &["end"]])?;
            ::core::result::Result::Ok(Self {
                days: fields.optional(0usize)?,
                start: fields.required(1usize)?,
                end: fields.required(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdateWindowRange<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            if let ::core::option::Option::Some(value) = &self.days {
                ::sidex_embedded::Encode::encode(value, object.field("days")?)?;
            }
            ::sidex_embedded::Encode::encode(&self.start, object.field("start")?)?;
            ::sidex_embedded::Encode::encode(&self.end, object.field("end")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nTime of day in the format `HH:MM` (24-hour clock).\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct TimeOfDay<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for TimeOfDay<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for TimeOfDay<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for TimeOfDay<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDay of the week.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum Weekday {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for Weekday {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match (
                ::sidex_embedded::json::case(
                    &name,
                    &[
                        "monday",
                        "tuesday",
                        "wednesday",
                        "thursday",
                        "friday",
                        "saturday",
                        "sunday",
                    ],
                ),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Monday)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Tuesday)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Wednesday)
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Thursday)
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Friday)
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Saturday)
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(6usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Sunday)
                }
                (::core::option::Option::Some(6usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for Weekday {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Monday => writer.str("monday"),
                Self::Tuesday => writer.str("tuesday"),
                Self::Wednesday => writer.str("wednesday"),
                Self::Thursday => writer.str("thursday"),
                Self::Friday => writer.str("friday"),
                Self::Saturday => writer.str("saturday"),
                Self::Sunday => writer.str("sunday"),
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nSpreading of updates across devices.\n\nEach device derives its position `u` from the SHA-256 hash of\n`<device-id>/<target-version-id>`: the first 8 bytes, read as a big-endian\nunsigned integer `n`, give `u = floor(n / 2^11) / 2^53`, which a\ndouble-precision number represents exactly. A device may start updating to the\ntarget once both of the following hold:\n\n- With `duration_secs`, its turn has begun at\n`start + floor(duration_secs * sqrt(u))`. The share of eligible devices\nthus grows quadratically from 0 at the start to 1 after `duration_secs`.\n- With `percentage`, its position is below `percentage / 100`.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OtaUpdateSpread {
        #[doc = "Time over which updates are spread, in seconds.\n"]
        pub duration_secs: ::core::option::Option<u32>,
        #[doc = "Start of the spread over time.\n\nDefaults to the time at which the tag in the configured path was assigned\nto its current version. Without an explicit start, updates to versions\nconfigured by version ID are not spread over time.\n"]
        pub start_at: ::core::option::Option<super::datetime::Timestamp>,
        #[doc = "Share of devices that may update, in percent.\n\nRaising the percentage only adds devices. Without `duration_secs`, it\nreleases the update to the given share right away. With `duration_secs`,\nit caps the share of the spread over time.\n"]
        pub percentage: ::core::option::Option<f64>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateSpread {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["durationSecs"], &["startAt"], &["percentage"]])?;
            ::core::result::Result::Ok(Self {
                duration_secs: fields.optional(0usize)?,
                start_at: fields.optional(1usize)?,
                percentage: fields.optional(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for OtaUpdateSpread {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            if let ::core::option::Option::Some(value) = &self.duration_secs {
                ::sidex_embedded::Encode::encode(value, object.field("durationSecs")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.start_at {
                ::sidex_embedded::Encode::encode(value, object.field("startAt")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.percentage {
                ::sidex_embedded::Encode::encode(value, object.field("percentage")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nRetry policy for failed update attempts.\n\nAn attempt starts when the device begins downloading a version. Failures\nbefore that, such as an unreachable Nexigon Hub, do not count as attempts.\nThe delay between attempts starts at `initial_delay_secs` and doubles after\neach failed attempt, up to `max_delay_secs`.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OtaUpdateRetryPolicy {
        #[doc = "Maximum number of failed attempts for a version.\n\nUnlimited when absent.\n"]
        pub max_attempts: ::core::option::Option<u32>,
        #[doc = "Delay before the first retry in seconds.\n"]
        pub initial_delay_secs: ::core::option::Option<u32>,
        #[doc = "Maximum delay between attempts in seconds.\n"]
        pub max_delay_secs: ::core::option::Option<u32>,
        #[doc = "Time in seconds after reaching `max_attempts` before the failure count\nresets and a new round of attempts begins.\n\nWithout a cooldown, the device stops attempting the version until the\neffective configuration changes or the failure count is cleared.\n"]
        pub cooldown_secs: ::core::option::Option<u32>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateRetryPolicy {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["maxAttempts"],
                &["initialDelaySecs"],
                &["maxDelaySecs"],
                &["cooldownSecs"],
            ])?;
            ::core::result::Result::Ok(Self {
                max_attempts: fields.optional(0usize)?,
                initial_delay_secs: fields.optional(1usize)?,
                max_delay_secs: fields.optional(2usize)?,
                cooldown_secs: fields.optional(3usize)?,
            })
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for OtaUpdateRetryPolicy {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            if let ::core::option::Option::Some(value) = &self.max_attempts {
                ::sidex_embedded::Encode::encode(value, object.field("maxAttempts")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.initial_delay_secs {
                ::sidex_embedded::Encode::encode(value, object.field("initialDelaySecs")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.max_delay_secs {
                ::sidex_embedded::Encode::encode(value, object.field("maxDelaySecs")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.cooldown_secs {
                ::sidex_embedded::Encode::encode(value, object.field("cooldownSecs")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOTA update status (`dev.nexigon.ota.status`).\n\nActual state of a device, published by its update integration.\n\nThe integration writes the status with the version it read last\n(compare-and-swap), so that it does not overwrite concurrent changes. Other\nwriters may only clear `failure_count` and `next_attempt_at`, which requests\na new attempt.\n\nWithin a format version, the contract evolves compatibly. Later versions\nonly add optional fields, states, capabilities, and codes. Readers ignore\nfields that they do not know and treat unknown states, capabilities, and\ncodes as unknown. An integration honors a configuration field that a later\nversion adds only if it reports the corresponding capability.\n\nA change that readers of earlier versions must not interpret increments the\nformat version of the property. Readers do not interpret a property with a\nformat version that they do not support. An integration does not apply such\na configuration and reports the blocked code `unsupported-config-format`.\nWriters use a later format version of the configuration only for\nintegrations that report support for it through a capability, which the\nlater version defines.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OtaUpdateStatus<'a> {
        #[doc = "Format version of the property.\n\nDefaults to 1, the format that this version of the contract defines.\n"]
        pub format_version: ::core::option::Option<u32>,
        #[doc = "Optional features of the OTA update contract that the integration\nsupports.\n"]
        pub capabilities:
            ::core::option::Option<::sidex_embedded::types::Seq<'a, OtaUpdateCapability<'a>>>,
        #[doc = "Effective configuration, including the defaults the integration applies.\n"]
        pub config: ::core::option::Option<OtaUpdateConfig<'a>>,
        #[doc = "Version of the `dev.nexigon.ota.config` property that the integration\nlast read.\n\nAbsent when the property does not exist. The version changes with every\nwrite to the device's own value and whenever a profile revision that\nsets the property is applied to the device, even with an unchanged\nvalue.\n"]
        pub config_version: ::core::option::Option<u64>,
        #[doc = "Version string of the running system, as built into the image.\n\nBy convention, a locked tag of the corresponding package version.\n"]
        pub current_version: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Package version of the running system.\n\nResolved by looking up `current_version` as a tag in the configured package.\nAbsent when the tag does not resolve.\n"]
        pub current_version_id: ::core::option::Option<super::repositories::PackageVersionId<'a>>,
        #[doc = "Current state of the update process.\n"]
        pub state: ::core::option::Option<OtaUpdateState<'a>>,
        #[doc = "Why the update is blocked. Present when `state` is `blocked`.\n"]
        pub blocked: ::core::option::Option<OtaUpdateBlocked<'a>>,
        #[doc = "Version string of the target version, as built into its image.\n"]
        pub target_version: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Package version that the configured path resolves to.\n"]
        pub target_version_id: ::core::option::Option<super::repositories::PackageVersionId<'a>>,
        #[doc = "Package version that the device installs next.\n\nDiffers from `target_version_id` when the device must install a\ncheckpoint before the target.\n"]
        pub next_version_id: ::core::option::Option<super::repositories::PackageVersionId<'a>>,
        #[doc = "Number of consecutive failed attempts for the next version.\n\nResets to zero when the next version or the effective configuration in\n`config` changes, when the retry cooldown ends, when an update\nsucceeds, or when another writer clears it.\n"]
        pub failure_count: ::core::option::Option<u32>,
        #[doc = "Machine-readable reason of the last failed attempt.\n\nPresent while `last_error` describes a failed attempt.\n"]
        pub failure_code: ::core::option::Option<OtaUpdateFailureCode<'a>>,
        #[doc = "Error message of the last failure, if any.\n"]
        pub last_error: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Earliest time of the next attempt.\n\nSet while the device waits for a spread, a maintenance window, a retry\ndelay, or a retry cooldown. Absent when the time of the next attempt is\nunknown or no further attempt is scheduled.\n"]
        pub next_attempt_at: ::core::option::Option<super::datetime::Timestamp>,
        #[doc = "Update system that installs versions on the device.\n"]
        pub updater: ::core::option::Option<OtaUpdater<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateStatus<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["formatVersion"],
                &["capabilities"],
                &["config"],
                &["configVersion"],
                &["currentVersion"],
                &["currentVersionId"],
                &["state"],
                &["blocked"],
                &["targetVersion"],
                &["targetVersionId"],
                &["nextVersionId"],
                &["failureCount"],
                &["failureCode"],
                &["lastError"],
                &["nextAttemptAt"],
                &["updater"],
            ])?;
            ::core::result::Result::Ok(Self {
                format_version: fields.optional(0usize)?,
                capabilities: fields.optional(1usize)?,
                config: fields.optional(2usize)?,
                config_version: fields.optional(3usize)?,
                current_version: fields.optional(4usize)?,
                current_version_id: fields.optional(5usize)?,
                state: fields.optional(6usize)?,
                blocked: fields.optional(7usize)?,
                target_version: fields.optional(8usize)?,
                target_version_id: fields.optional(9usize)?,
                next_version_id: fields.optional(10usize)?,
                failure_count: fields.optional(11usize)?,
                failure_code: fields.optional(12usize)?,
                last_error: fields.optional(13usize)?,
                next_attempt_at: fields.optional(14usize)?,
                updater: fields.optional(15usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdateStatus<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            if let ::core::option::Option::Some(value) = &self.format_version {
                ::sidex_embedded::Encode::encode(value, object.field("formatVersion")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.capabilities {
                ::sidex_embedded::Encode::encode(value, object.field("capabilities")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.config {
                ::sidex_embedded::Encode::encode(value, object.field("config")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.config_version {
                ::sidex_embedded::Encode::encode(value, object.field("configVersion")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.current_version {
                ::sidex_embedded::Encode::encode(value, object.field("currentVersion")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.current_version_id {
                ::sidex_embedded::Encode::encode(value, object.field("currentVersionId")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.state {
                ::sidex_embedded::Encode::encode(value, object.field("state")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.blocked {
                ::sidex_embedded::Encode::encode(value, object.field("blocked")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.target_version {
                ::sidex_embedded::Encode::encode(value, object.field("targetVersion")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.target_version_id {
                ::sidex_embedded::Encode::encode(value, object.field("targetVersionId")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.next_version_id {
                ::sidex_embedded::Encode::encode(value, object.field("nextVersionId")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.failure_count {
                ::sidex_embedded::Encode::encode(value, object.field("failureCount")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.failure_code {
                ::sidex_embedded::Encode::encode(value, object.field("failureCode")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.last_error {
                ::sidex_embedded::Encode::encode(value, object.field("lastError")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.next_attempt_at {
                ::sidex_embedded::Encode::encode(value, object.field("nextAttemptAt")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.updater {
                ::sidex_embedded::Encode::encode(value, object.field("updater")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOptional feature of the OTA update contract.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum OtaUpdateCapability<'a> {
        #[doc = "The integration honors `config.hold`.\n"]
        Hold,
        #[doc = "The integration honors `config.asset`.\n"]
        Asset,
        #[doc = "The integration honors `config.window`.\n"]
        Window,
        #[doc = "The integration honors `config.retry`.\n"]
        Retry,
        #[doc = "The integration honors `config.spread`.\n"]
        Spread,
        #[doc = "The integration installs checkpoints according to base versions.\n"]
        Checkpoints,
        #[doc = "Capability that a later version defines.\n"]
        Other(::sidex_embedded::types::Str<'a>),
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateCapability<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(_) => {
                    return ::core::result::Result::Ok(Self::Other(<::sidex_embedded::types::Str<
                        'a,
                    > as ::sidex_embedded::Decode<'a>>::decode(
                        &mut value.clone()
                    )?));
                }
            };
            match (
                ::sidex_embedded::json::case(
                    &name,
                    &["hold", "asset", "window", "retry", "spread", "checkpoints"],
                ),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Hold)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Asset)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Window)
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Retry)
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Spread)
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Checkpoints)
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Ok(Self::Other(
                    <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(
                        &mut value.clone(),
                    )?,
                )),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdateCapability<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Hold => writer.str("hold"),
                Self::Asset => writer.str("asset"),
                Self::Window => writer.str("window"),
                Self::Retry => writer.str("retry"),
                Self::Spread => writer.str("spread"),
                Self::Checkpoints => writer.str("checkpoints"),
                Self::Other(payload) => ::sidex_embedded::Encode::encode(payload, writer),
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nReason of a failed update attempt.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum OtaUpdateFailureCode<'a> {
        #[doc = "Downloading or installing the version failed.\n"]
        InstallFailed,
        #[doc = "The installation was interrupted, for example because the device shut\ndown.\n"]
        Interrupted,
        #[doc = "Rebooting into the installed version failed.\n"]
        RebootFailed,
        #[doc = "After the reboot, the device runs another version than the installed one.\n"]
        RolledBack,
        #[doc = "Code in kebab case that a later version or the integration defines.\n"]
        Other(::sidex_embedded::types::Str<'a>),
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateFailureCode<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(_) => {
                    return ::core::result::Result::Ok(Self::Other(<::sidex_embedded::types::Str<
                        'a,
                    > as ::sidex_embedded::Decode<'a>>::decode(
                        &mut value.clone()
                    )?));
                }
            };
            match (
                ::sidex_embedded::json::case(
                    &name,
                    &[
                        "install-failed",
                        "interrupted",
                        "reboot-failed",
                        "rolled-back",
                    ],
                ),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::InstallFailed)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Interrupted)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::RebootFailed)
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::RolledBack)
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Ok(Self::Other(
                    <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(
                        &mut value.clone(),
                    )?,
                )),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdateFailureCode<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::InstallFailed => writer.str("install-failed"),
                Self::Interrupted => writer.str("interrupted"),
                Self::RebootFailed => writer.str("reboot-failed"),
                Self::RolledBack => writer.str("rolled-back"),
                Self::Other(payload) => ::sidex_embedded::Encode::encode(payload, writer),
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUpdate system that installs versions on a device, such as Rugix Ctrl or RAUC.\n\nSlot names, states, and health values are those of the update system.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OtaUpdater<'a> {
        #[doc = "Name of the update system, such as `rugix` or `rauc`.\n"]
        pub name: ::sidex_embedded::types::Str<'a>,
        #[doc = "Version of the update system.\n"]
        pub version: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Slot that the running system booted from.\n"]
        pub booted_slot: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Slot that the device boots from next.\n\nDiffers from `booted_slot` when the device runs a version on trial or\nhas installed a version that it has not booted yet.\n"]
        pub primary_slot: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Whether the running system awaits its commit.\n"]
        pub awaiting_commit: ::core::option::Option<bool>,
        #[doc = "Slots that the update system installs versions into.\n"]
        pub slots: ::core::option::Option<::sidex_embedded::types::Seq<'a, OtaUpdaterSlot<'a>>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdater<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["name"],
                &["version"],
                &["bootedSlot"],
                &["primarySlot"],
                &["awaitingCommit"],
                &["slots"],
            ])?;
            ::core::result::Result::Ok(Self {
                name: fields.required(0usize)?,
                version: fields.optional(1usize)?,
                booted_slot: fields.optional(2usize)?,
                primary_slot: fields.optional(3usize)?,
                awaiting_commit: fields.optional(4usize)?,
                slots: fields.optional(5usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdater<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.name, object.field("name")?)?;
            if let ::core::option::Option::Some(value) = &self.version {
                ::sidex_embedded::Encode::encode(value, object.field("version")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.booted_slot {
                ::sidex_embedded::Encode::encode(value, object.field("bootedSlot")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.primary_slot {
                ::sidex_embedded::Encode::encode(value, object.field("primarySlot")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.awaiting_commit {
                ::sidex_embedded::Encode::encode(value, object.field("awaitingCommit")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.slots {
                ::sidex_embedded::Encode::encode(value, object.field("slots")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nSlot of an update system.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OtaUpdaterSlot<'a> {
        #[doc = "Name of the slot.\n"]
        pub name: ::sidex_embedded::types::Str<'a>,
        #[doc = "State of the slot, such as `booted` or `inactive`.\n"]
        pub state: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Health of the slot, such as `good` or `bad`.\n"]
        pub health: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Version installed in the slot, if the update system records it.\n"]
        pub version: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdaterSlot<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["name"], &["state"], &["health"], &["version"]])?;
            ::core::result::Result::Ok(Self {
                name: fields.required(0usize)?,
                state: fields.optional(1usize)?,
                health: fields.optional(2usize)?,
                version: fields.optional(3usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdaterSlot<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.name, object.field("name")?)?;
            if let ::core::option::Option::Some(value) = &self.state {
                ::sidex_embedded::Encode::encode(value, object.field("state")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.health {
                ::sidex_embedded::Encode::encode(value, object.field("health")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.version {
                ::sidex_embedded::Encode::encode(value, object.field("version")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOTA update state.\n\nThe resting states are `idle`, `completed`, `failed`, and `blocked`.\nThe integration remains in one of these until the next check begins or\nthe blocking condition clears.\n\nA rollback ends the attempt as failed with the failure code `rolled-back`.\nAn integration that rolls back deliberately, for example because the new\nversion fails a health check, reports `rollback` while it does so. It\nleaves `failure_count` unchanged and may describe the reason in\n`last_error`; the next check records the failure. A rollback that the\ndevice only detects after the reboot, for example because the boot loader\nfell back to the previous version, has no `rollback` state.\n\n`installing`, `rebooting`, `committing`, and `rollback` belong to an\nattempt in progress. If the previous status reports one of them, the next\ncheck determines the outcome of the attempt from the running version. If it\nreports a state that the integration does not know, the integration keeps\nthe failure count and the time of the next attempt, like for `failed`.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum OtaUpdateState<'a> {
        #[doc = "No path is configured, or no update has been attempted yet.\n"]
        Idle,
        #[doc = "An update is being installed.\n"]
        Installing,
        #[doc = "The device is rebooting into the new version.\n"]
        Rebooting,
        #[doc = "Committing the new version after a successful reboot.\n"]
        Committing,
        #[doc = "The last update completed successfully, or the device already runs the\ntarget.\n"]
        Completed,
        #[doc = "The last update failed.\n"]
        Failed,
        #[doc = "The update cannot proceed yet. See `blocked` for the reason.\n"]
        Blocked,
        #[doc = "The device is rolling back to the previous version, for example because\nthe new version failed a health check.\n"]
        Rollback,
        #[doc = "State that a later version defines.\n"]
        Other(::sidex_embedded::types::Str<'a>),
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateState<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(_) => {
                    return ::core::result::Result::Ok(Self::Other(<::sidex_embedded::types::Str<
                        'a,
                    > as ::sidex_embedded::Decode<'a>>::decode(
                        &mut value.clone()
                    )?));
                }
            };
            match (
                ::sidex_embedded::json::case(
                    &name,
                    &[
                        "idle",
                        "installing",
                        "rebooting",
                        "committing",
                        "completed",
                        "failed",
                        "blocked",
                        "rollback",
                    ],
                ),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Idle)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Installing)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Rebooting)
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Committing)
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Completed)
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Failed)
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(6usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Blocked)
                }
                (::core::option::Option::Some(6usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(7usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Rollback)
                }
                (::core::option::Option::Some(7usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Ok(Self::Other(
                    <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(
                        &mut value.clone(),
                    )?,
                )),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdateState<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Idle => writer.str("idle"),
                Self::Installing => writer.str("installing"),
                Self::Rebooting => writer.str("rebooting"),
                Self::Committing => writer.str("committing"),
                Self::Completed => writer.str("completed"),
                Self::Failed => writer.str("failed"),
                Self::Blocked => writer.str("blocked"),
                Self::Rollback => writer.str("rollback"),
                Self::Other(payload) => ::sidex_embedded::Encode::encode(payload, writer),
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nInformation about a blocked update.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct OtaUpdateBlocked<'a> {
        #[doc = "Machine-readable reason.\n"]
        pub code: OtaUpdateBlockedCode<'a>,
        #[doc = "Human-readable explanation.\n"]
        pub message: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateBlocked<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["code"], &["message"]])?;
            ::core::result::Result::Ok(Self {
                code: fields.required(0usize)?,
                message: fields.optional(1usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdateBlocked<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.code, object.field("code")?)?;
            if let ::core::option::Option::Some(value) = &self.message {
                ::sidex_embedded::Encode::encode(value, object.field("message")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nReason of a blocked update.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum OtaUpdateBlockedCode<'a> {
        #[doc = "The configuration holds the device at its current version.\n"]
        Hold,
        #[doc = "The device waits for its turn in a spread or is outside the released\npercentage.\n"]
        Spread,
        #[doc = "The device waits for its maintenance window.\n"]
        Window,
        #[doc = "The target is unreachable without going back across a checkpoint.\n"]
        NoUpdatePath,
        #[doc = "The target has a base version, but `current_version` does not resolve\nto a package version.\n"]
        UnknownCurrentVersion,
        #[doc = "The next version has no available asset with the filename in\n`config.asset`.\n"]
        NoAsset,
        #[doc = "The configuration has a format version that the integration does not\nsupport.\n"]
        UnsupportedConfigFormat,
        #[doc = "Code in kebab case that a later version or the integration defines, for\nconditions specific to the device, such as `awaiting-approval`.\n"]
        Other(::sidex_embedded::types::Str<'a>),
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for OtaUpdateBlockedCode<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(_) => {
                    return ::core::result::Result::Ok(Self::Other(<::sidex_embedded::types::Str<
                        'a,
                    > as ::sidex_embedded::Decode<'a>>::decode(
                        &mut value.clone()
                    )?));
                }
            };
            match (
                ::sidex_embedded::json::case(
                    &name,
                    &[
                        "hold",
                        "spread",
                        "window",
                        "no-update-path",
                        "unknown-current-version",
                        "no-asset",
                        "unsupported-config-format",
                    ],
                ),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Hold)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Spread)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Window)
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::NoUpdatePath)
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::UnknownCurrentVersion)
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::NoAsset)
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(6usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::UnsupportedConfigFormat)
                }
                (::core::option::Option::Some(6usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Ok(Self::Other(
                    <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(
                        &mut value.clone(),
                    )?,
                )),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for OtaUpdateBlockedCode<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Hold => writer.str("hold"),
                Self::Spread => writer.str("spread"),
                Self::Window => writer.str("window"),
                Self::NoUpdatePath => writer.str("no-update-path"),
                Self::UnknownCurrentVersion => writer.str("unknown-current-version"),
                Self::NoAsset => writer.str("no-asset"),
                Self::UnsupportedConfigFormat => writer.str("unsupported-config-format"),
                Self::Other(payload) => ::sidex_embedded::Encode::encode(payload, writer),
            }
        }
    }
    #[doc = "**Unstable (feature `device_commands`).**\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nCommand manifest (`dev.nexigon.commands` property).\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceCommandManifest<'a> {
        #[doc = "Available commands.\n"]
        pub commands: ::sidex_embedded::types::Seq<'a, DeviceCommandDescriptor<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceCommandManifest<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["commands"]])?;
            ::core::result::Result::Ok(Self {
                commands: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceCommandManifest<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.commands, object.field("commands")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Unstable (feature `device_commands`).**\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nDescriptor for an on-demand command.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceCommandDescriptor<'a> {
        #[doc = "Command name.\n"]
        pub name: ::sidex_embedded::types::Str<'a>,
        #[doc = "Description of what the command does.\n"]
        pub description: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Category for grouping in the UI.\n"]
        pub category: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "JSON Schema for the command input.\n"]
        pub input: ::core::option::Option<super::json::JsonValue<'a>>,
        #[doc = "JSON Schema for the command output.\n"]
        pub output: ::core::option::Option<super::json::JsonValue<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for DeviceCommandDescriptor<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["name"],
                &["description"],
                &["category"],
                &["input"],
                &["output"],
            ])?;
            ::core::result::Result::Ok(Self {
                name: fields.required(0usize)?,
                description: fields.optional(1usize)?,
                category: fields.optional(2usize)?,
                input: fields.optional(3usize)?,
                output: fields.optional(4usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for DeviceCommandDescriptor<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.name, object.field("name")?)?;
            if let ::core::option::Option::Some(value) = &self.description {
                ::sidex_embedded::Encode::encode(value, object.field("description")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.category {
                ::sidex_embedded::Encode::encode(value, object.field("category")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.input {
                ::sidex_embedded::Encode::encode(value, object.field("input")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.output {
                ::sidex_embedded::Encode::encode(value, object.field("output")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
}
pub mod repositories {
    #![doc = "Actions and types related to repositories.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a repository.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct RepositoryId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for RepositoryId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for RepositoryId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for RepositoryId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a repository asset.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct RepositoryAssetId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for RepositoryAssetId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for RepositoryAssetId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for RepositoryAssetId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a package.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct PackageId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for PackageId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for PackageId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for PackageId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a package version.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct PackageVersionId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for PackageVersionId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for PackageVersionId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for PackageVersionId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nAsset status.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum RepositoryAssetStatus {
        #[doc = "The asset has been created but not yet uploaded.\n"]
        Dangling,
        #[doc = "The asset has been uploaded and is available.\n"]
        Available,
        #[doc = "The asset is in the process of being deleted.\n"]
        Deleting,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for RepositoryAssetStatus {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match (
                ::sidex_embedded::json::case(&name, &["Dangling", "Available", "Deleting"]),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Dangling)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Available)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Deleting)
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for RepositoryAssetStatus {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Dangling => writer.str("Dangling"),
                Self::Available => writer.str("Available"),
                Self::Deleting => writer.str("Deleting"),
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nResult of the latest non-destructive object-storage check.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum RepositoryAssetStorageStatus {
        #[doc = "The asset has not been checked yet.\n"]
        Unknown,
        #[doc = "A legacy object exists with the expected size, but has no verifiable checksum metadata.\n"]
        Present,
        #[doc = "The object carries matching size, checksum, digest, and ownership metadata.\n"]
        Verified,
        #[doc = "No object exists at the expected key.\n"]
        Missing,
        #[doc = "The object exists but its size or verification metadata does not match.\n"]
        Mismatched,
        #[doc = "The storage provider could not be checked.\n"]
        Error,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for RepositoryAssetStorageStatus {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let (name, payload) = match reader.external()? {
                ::core::result::Result::Ok(::sidex_embedded::json::External::Name(name)) => {
                    (name, ::core::option::Option::None)
                }
                ::core::result::Result::Ok(::sidex_embedded::json::External::Entry(
                    name,
                    payload,
                )) => (name, ::core::option::Option::Some(payload)),
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match (
                ::sidex_embedded::json::case(
                    &name,
                    &[
                        "Unknown",
                        "Present",
                        "Verified",
                        "Missing",
                        "Mismatched",
                        "Error",
                    ],
                ),
                payload,
            ) {
                (::core::option::Option::Some(0usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Unknown)
                }
                (::core::option::Option::Some(0usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Present)
                }
                (::core::option::Option::Some(1usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Verified)
                }
                (::core::option::Option::Some(2usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Missing)
                }
                (::core::option::Option::Some(3usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Mismatched)
                }
                (::core::option::Option::Some(4usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::Error)
                }
                (::core::option::Option::Some(5usize), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(
                        ::sidex_embedded::DecodeErrorKind::Unexpected(
                            "a string naming a case without payload",
                        ),
                    ))
                }
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::sidex_embedded::Encode for RepositoryAssetStorageStatus {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Unknown => writer.str("Unknown"),
                Self::Present => writer.str("Present"),
                Self::Verified => writer.str("Verified"),
                Self::Missing => writer.str("Missing"),
                Self::Mismatched => writer.str("Mismatched"),
                Self::Error => writer.str("Error"),
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nLatest object-storage check for an asset.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct RepositoryAssetStorage<'a> {
        pub status: RepositoryAssetStorageStatus,
        pub checked_at: ::core::option::Option<super::datetime::Timestamp>,
        pub detail: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for RepositoryAssetStorage<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["status"], &["checkedAt"], &["detail"]])?;
            ::core::result::Result::Ok(Self {
                status: fields.required(0usize)?,
                checked_at: fields.optional(1usize)?,
                detail: fields.optional(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for RepositoryAssetStorage<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.status, object.field("status")?)?;
            if let ::core::option::Option::Some(value) = &self.checked_at {
                ::sidex_embedded::Encode::encode(value, object.field("checkedAt")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.detail {
                ::sidex_embedded::Encode::encode(value, object.field("detail")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nPackage version tag.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct PackageVersionTag<'a> {
        #[doc = "Tag of the package version.\n"]
        pub tag: ::sidex_embedded::types::Str<'a>,
        #[doc = "Indicates whether the tag is locked.\n"]
        pub locked: bool,
        #[doc = "Time at which the tag was assigned to the version. Absent on older Hub versions.\n"]
        pub assigned_at: ::core::option::Option<super::datetime::Timestamp>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for PackageVersionTag<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["tag"], &["locked"], &["assignedAt"]])?;
            ::core::result::Result::Ok(Self {
                tag: fields.required(0usize)?,
                locked: fields.required(1usize)?,
                assigned_at: fields.optional(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for PackageVersionTag<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.tag, object.field("tag")?)?;
            ::sidex_embedded::Encode::encode(&self.locked, object.field("locked")?)?;
            if let ::core::option::Option::Some(value) = &self.assigned_at {
                ::sidex_embedded::Encode::encode(value, object.field("assignedAt")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nPackage version asset.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct PackageVersionAsset<'a> {
        #[doc = "Filename of the asset.\n"]
        pub filename: ::sidex_embedded::types::Str<'a>,
        #[doc = "ID of the asset.\n"]
        pub asset_id: RepositoryAssetId<'a>,
        #[doc = "Size of the asset.\n"]
        pub size: u64,
        #[doc = "Digest of the asset.\n"]
        pub digest: super::digest::Digest<'a>,
        #[doc = "Status of the asset.\n"]
        pub status: RepositoryAssetStatus,
        #[doc = "Latest storage check. Absent when connected to an older Hub.\n"]
        pub storage: ::core::option::Option<RepositoryAssetStorage<'a>>,
        #[doc = "Arbitrary metadata associated with this asset in the version.\n"]
        pub metadata: ::sidex_embedded::types::Map<
            'a,
            ::sidex_embedded::types::Str<'a>,
            super::json::JsonValue<'a>,
        >,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for PackageVersionAsset<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["filename"],
                &["assetId"],
                &["size"],
                &["digest"],
                &["status"],
                &["storage"],
                &["metadata"],
            ])?;
            ::core::result::Result::Ok(Self {
                filename: fields.required(0usize)?,
                asset_id: fields.required(1usize)?,
                size: fields.required(2usize)?,
                digest: fields.required(3usize)?,
                status: fields.required(4usize)?,
                storage: fields.optional(5usize)?,
                metadata: fields.required(6usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for PackageVersionAsset<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.filename, object.field("filename")?)?;
            ::sidex_embedded::Encode::encode(&self.asset_id, object.field("assetId")?)?;
            ::sidex_embedded::Encode::encode(&self.size, object.field("size")?)?;
            ::sidex_embedded::Encode::encode(&self.digest, object.field("digest")?)?;
            ::sidex_embedded::Encode::encode(&self.status, object.field("status")?)?;
            if let ::core::option::Option::Some(value) = &self.storage {
                ::sidex_embedded::Encode::encode(value, object.field("storage")?)?;
            }
            ::sidex_embedded::Encode::encode(&self.metadata, object.field("metadata")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nResolve a package version by its path.\n\nA path consists of a repository, a package, and a version tag. Devices can resolve\nversions in repositories linked to their project.\n\nWithin an organization, the repository is given by its path name. The\norganization defaults to that of the request, which is set by an organization\nAPI token, the `X-Nexigon-Organization` header, or the project of a device.\nWithout an organization, the repository is given by its slug. A repository ID\nresolves in both cases.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ResolvePackageVersionByPathAction<'a> {
        #[doc = "Organization to resolve path names in.\n\nDefaults to the organization of the request. See the action for details.\n"]
        pub organization_id: ::core::option::Option<super::organizations::OrganizationId<'a>>,
        #[doc = "Path name, slug, or ID of the repository.\n"]
        pub repository: ::sidex_embedded::types::Str<'a>,
        #[doc = "Name or ID of the package.\n"]
        pub package: ::sidex_embedded::types::Str<'a>,
        #[doc = "Tag or ID of the version.\n"]
        pub tag: ::sidex_embedded::types::Str<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ResolvePackageVersionByPathAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields =
                reader.record(&[&["organizationId"], &["repository"], &["package"], &["tag"]])?;
            ::core::result::Result::Ok(Self {
                organization_id: fields.optional(0usize)?,
                repository: fields.required(1usize)?,
                package: fields.required(2usize)?,
                tag: fields.required(3usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ResolvePackageVersionByPathAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            if let ::core::option::Option::Some(value) = &self.organization_id {
                ::sidex_embedded::Encode::encode(value, object.field("organizationId")?)?;
            }
            ::sidex_embedded::Encode::encode(&self.repository, object.field("repository")?)?;
            ::sidex_embedded::Encode::encode(&self.package, object.field("package")?)?;
            ::sidex_embedded::Encode::encode(&self.tag, object.field("tag")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOutput of resolving a package version by its path.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum ResolvePackageVersionByPathOutput<'a> {
        #[doc = "The package version has been found.\n"]
        Found(ResolvePackageVersionByPathSuccess<'a>),
        #[doc = "The package version has not been found.\n"]
        NotFound,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ResolvePackageVersionByPathOutput<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let name = match reader.tag("result")? {
                ::core::result::Result::Ok(name) => name,
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match ::sidex_embedded::json::case(&name, &["Found", "NotFound"]) {
                ::core::option::Option::Some(0usize) => {
                    ::core::result::Result::Ok(Self::Found(
                        if <ResolvePackageVersionByPathSuccess<'a> as ::sidex_embedded::Decode<
                            'a,
                        >>::IS_RECORD
                        {
                            <ResolvePackageVersionByPathSuccess<'a> as ::sidex_embedded::Decode<
                                'a,
                            >>::decode(&mut value.clone())?
                        } else {
                            <ResolvePackageVersionByPathSuccess<'a> as ::sidex_embedded::Decode<
                                'a,
                            >>::decode(&mut value.content("content")?)?
                        },
                    ))
                }
                ::core::option::Option::Some(1usize) => ::core::result::Result::Ok(Self::NotFound),
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ResolvePackageVersionByPathOutput<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Found(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("result")?.str("Found")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("result")?.str("Found")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::NotFound => {
                    let mut object = writer.object()?;
                    object.field("result")?.str("NotFound")?;
                    object.end()
                }
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nSuccessfully resolved package version by path.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ResolvePackageVersionByPathSuccess<'a> {
        #[doc = "ID of the repository.\n"]
        pub repository_id: RepositoryId<'a>,
        #[doc = "ID of the package.\n"]
        pub package_id: PackageId<'a>,
        #[doc = "ID of the package version.\n"]
        pub version_id: PackageVersionId<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ResolvePackageVersionByPathSuccess<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["repositoryId"], &["packageId"], &["versionId"]])?;
            ::core::result::Result::Ok(Self {
                repository_id: fields.required(0usize)?,
                package_id: fields.required(1usize)?,
                version_id: fields.required(2usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ResolvePackageVersionByPathSuccess<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.repository_id, object.field("repositoryId")?)?;
            ::sidex_embedded::Encode::encode(&self.package_id, object.field("packageId")?)?;
            ::sidex_embedded::Encode::encode(&self.version_id, object.field("versionId")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nGet information about a package version.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct GetPackageVersionDetailsAction<'a> {
        #[doc = "ID of the package version.\n"]
        pub version_id: PackageVersionId<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for GetPackageVersionDetailsAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["versionId"]])?;
            ::core::result::Result::Ok(Self {
                version_id: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for GetPackageVersionDetailsAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.version_id, object.field("versionId")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOutput of getting information about a package version.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct GetPackageVersionDetailsOutput<'a> {
        #[doc = "ID of the package version.\n"]
        pub version_id: PackageVersionId<'a>,
        #[doc = "ID of the repository.\n"]
        pub repository_id: RepositoryId<'a>,
        #[doc = "ID of the package.\n"]
        pub package_id: PackageId<'a>,
        #[doc = "Optional name of the version.\n"]
        pub name: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
        #[doc = "Assets of the version.\n"]
        pub assets: ::sidex_embedded::types::Seq<'a, PackageVersionAsset<'a>>,
        #[doc = "Tags of the version.\n"]
        pub tags: ::sidex_embedded::types::Seq<'a, PackageVersionTag<'a>>,
        #[doc = "Version-level metadata.\n"]
        pub metadata: ::sidex_embedded::types::Map<
            'a,
            ::sidex_embedded::types::Str<'a>,
            super::json::JsonValue<'a>,
        >,
        #[doc = "Explicit deletion deadline. Package retention can delete the version earlier.\n"]
        pub delete_at: ::core::option::Option<super::datetime::Timestamp>,
        #[doc = "Sealed versions reject content changes and deletion, except by deleting their\nrepository. Absent on older Hub versions.\n"]
        pub sealed: ::core::option::Option<bool>,
        #[doc = "Releases are retained indefinitely. Independent of sealing. Absent on older Hub versions.\n"]
        pub release: ::core::option::Option<bool>,
        #[doc = "Version that a device must run before installing this version.\n"]
        pub base_version_id: ::core::option::Option<PackageVersionId<'a>>,
        #[doc = "Timestamp when the package version was created. Absent on older Hub versions.\n"]
        pub created_at: ::core::option::Option<super::datetime::Timestamp>,
        #[doc = "Optional name of the base version.\n"]
        pub base_version_name: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for GetPackageVersionDetailsOutput<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["versionId"],
                &["repositoryId"],
                &["packageId"],
                &["name"],
                &["assets"],
                &["tags"],
                &["metadata"],
                &["deleteAt"],
                &["sealed"],
                &["release"],
                &["baseVersionId"],
                &["createdAt"],
                &["baseVersionName"],
            ])?;
            ::core::result::Result::Ok(Self {
                version_id: fields.required(0usize)?,
                repository_id: fields.required(1usize)?,
                package_id: fields.required(2usize)?,
                name: fields.optional(3usize)?,
                assets: fields.required(4usize)?,
                tags: fields.required(5usize)?,
                metadata: fields.required(6usize)?,
                delete_at: fields.optional(7usize)?,
                sealed: fields.optional(8usize)?,
                release: fields.optional(9usize)?,
                base_version_id: fields.optional(10usize)?,
                created_at: fields.optional(11usize)?,
                base_version_name: fields.optional(12usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for GetPackageVersionDetailsOutput<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.version_id, object.field("versionId")?)?;
            ::sidex_embedded::Encode::encode(&self.repository_id, object.field("repositoryId")?)?;
            ::sidex_embedded::Encode::encode(&self.package_id, object.field("packageId")?)?;
            if let ::core::option::Option::Some(value) = &self.name {
                ::sidex_embedded::Encode::encode(value, object.field("name")?)?;
            }
            ::sidex_embedded::Encode::encode(&self.assets, object.field("assets")?)?;
            ::sidex_embedded::Encode::encode(&self.tags, object.field("tags")?)?;
            ::sidex_embedded::Encode::encode(&self.metadata, object.field("metadata")?)?;
            if let ::core::option::Option::Some(value) = &self.delete_at {
                ::sidex_embedded::Encode::encode(value, object.field("deleteAt")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.sealed {
                ::sidex_embedded::Encode::encode(value, object.field("sealed")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.release {
                ::sidex_embedded::Encode::encode(value, object.field("release")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.base_version_id {
                ::sidex_embedded::Encode::encode(value, object.field("baseVersionId")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.created_at {
                ::sidex_embedded::Encode::encode(value, object.field("createdAt")?)?;
            }
            if let ::core::option::Option::Some(value) = &self.base_version_name {
                ::sidex_embedded::Encode::encode(value, object.field("baseVersionName")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nResolve a version asset by its path.\n\nA path consists of a repository, a package, a version tag, and the asset's\nfilename within the version.\n\nWithin an organization, the repository is given by its path name. The\norganization defaults to that of the request, which is set by an organization\nAPI token, the `X-Nexigon-Organization` header, or the project of a device.\nWithout an organization, the repository is given by its slug. A repository ID\nresolves in both cases.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ResolvePackageVersionAssetByPathAction<'a> {
        #[doc = "Organization to resolve path names in.\n\nDefaults to the organization of the request. See the action for details.\n"]
        pub organization_id: ::core::option::Option<super::organizations::OrganizationId<'a>>,
        #[doc = "Path name, slug, or ID of the repository.\n"]
        pub repository: ::sidex_embedded::types::Str<'a>,
        #[doc = "Name or ID of the package.\n"]
        pub package: ::sidex_embedded::types::Str<'a>,
        #[doc = "Tag or ID of the version.\n"]
        pub tag: ::sidex_embedded::types::Str<'a>,
        #[doc = "Filename or ID of the asset.\n"]
        pub filename: ::sidex_embedded::types::Str<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ResolvePackageVersionAssetByPathAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["organizationId"],
                &["repository"],
                &["package"],
                &["tag"],
                &["filename"],
            ])?;
            ::core::result::Result::Ok(Self {
                organization_id: fields.optional(0usize)?,
                repository: fields.required(1usize)?,
                package: fields.required(2usize)?,
                tag: fields.required(3usize)?,
                filename: fields.required(4usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ResolvePackageVersionAssetByPathAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            if let ::core::option::Option::Some(value) = &self.organization_id {
                ::sidex_embedded::Encode::encode(value, object.field("organizationId")?)?;
            }
            ::sidex_embedded::Encode::encode(&self.repository, object.field("repository")?)?;
            ::sidex_embedded::Encode::encode(&self.package, object.field("package")?)?;
            ::sidex_embedded::Encode::encode(&self.tag, object.field("tag")?)?;
            ::sidex_embedded::Encode::encode(&self.filename, object.field("filename")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOutput of resolving a version asset by its path.\n"]
    #[derive(Clone, Copy, Debug)]
    pub enum ResolvePackageVersionAssetByPathOutput<'a> {
        #[doc = "The asset has been found.\n"]
        Found(ResolvePackageVersionAssetByPathSuccess<'a>),
        #[doc = "The asset has not been found.\n"]
        NotFound,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ResolvePackageVersionAssetByPathOutput<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let value = reader.clone();
            let name = match reader.tag("result")? {
                ::core::result::Result::Ok(name) => name,
                ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
            };
            match ::sidex_embedded::json::case(&name, &["Found", "NotFound"]) {
                ::core::option::Option::Some(0usize) => ::core::result::Result::Ok(Self::Found(
                    if <ResolvePackageVersionAssetByPathSuccess<'a> as ::sidex_embedded::Decode<
                        'a,
                    >>::IS_RECORD
                    {
                        <ResolvePackageVersionAssetByPathSuccess<'a> as ::sidex_embedded::Decode<
                            'a,
                        >>::decode(&mut value.clone())?
                    } else {
                        <ResolvePackageVersionAssetByPathSuccess<'a> as ::sidex_embedded::Decode<
                            'a,
                        >>::decode(&mut value.content("content")?)?
                    },
                )),
                ::core::option::Option::Some(1usize) => ::core::result::Result::Ok(Self::NotFound),
                _ => ::core::result::Result::Err(
                    value.error(::sidex_embedded::DecodeErrorKind::UnknownCase),
                ),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ResolvePackageVersionAssetByPathOutput<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            match self {
                Self::Found(payload) => {
                    if ::sidex_embedded::Encode::is_record(payload) {
                        let mut object = writer.object()?;
                        object.field("result")?.str("Found")?;
                        ::sidex_embedded::Encode::encode_fields(payload, &mut object)?;
                        object.end()
                    } else {
                        let mut object = writer.object()?;
                        object.field("result")?.str("Found")?;
                        ::sidex_embedded::Encode::encode(payload, object.field("content")?)?;
                        object.end()
                    }
                }
                Self::NotFound => {
                    let mut object = writer.object()?;
                    object.field("result")?.str("NotFound")?;
                    object.end()
                }
            }
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nSuccessfully resolved version asset by path.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct ResolvePackageVersionAssetByPathSuccess<'a> {
        #[doc = "ID of the repository.\n"]
        pub repository_id: RepositoryId<'a>,
        #[doc = "ID of the package.\n"]
        pub package_id: PackageId<'a>,
        #[doc = "ID of the package version.\n"]
        pub version_id: PackageVersionId<'a>,
        #[doc = "ID of the asset.\n"]
        pub asset_id: RepositoryAssetId<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for ResolvePackageVersionAssetByPathSuccess<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[
                &["repositoryId"],
                &["packageId"],
                &["versionId"],
                &["assetId"],
            ])?;
            ::core::result::Result::Ok(Self {
                repository_id: fields.required(0usize)?,
                package_id: fields.required(1usize)?,
                version_id: fields.required(2usize)?,
                asset_id: fields.required(3usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for ResolvePackageVersionAssetByPathSuccess<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.repository_id, object.field("repositoryId")?)?;
            ::sidex_embedded::Encode::encode(&self.package_id, object.field("packageId")?)?;
            ::sidex_embedded::Encode::encode(&self.version_id, object.field("versionId")?)?;
            ::sidex_embedded::Encode::encode(&self.asset_id, object.field("assetId")?)?;
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nIssue a URL for downloading an asset.\n\nThe URL is valid for two hours and requires no further authentication.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct IssueAssetDownloadUrlAction<'a> {
        #[doc = "ID of the asset to issue the URL for.\n"]
        pub asset_id: RepositoryAssetId<'a>,
        #[doc = "Filename under which browsers save the download, set through the\n`Content-Disposition` header of the download response.\n"]
        pub filename: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for IssueAssetDownloadUrlAction<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["assetId"], &["filename"]])?;
            ::core::result::Result::Ok(Self {
                asset_id: fields.required(0usize)?,
                filename: fields.optional(1usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for IssueAssetDownloadUrlAction<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.asset_id, object.field("assetId")?)?;
            if let ::core::option::Option::Some(value) = &self.filename {
                ::sidex_embedded::Encode::encode(value, object.field("filename")?)?;
            }
            ::core::result::Result::Ok(())
        }
    }
    #[doc = "**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nOutput of issuing a URL for downloading an asset.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct IssueAssetDownloadUrlOutput<'a> {
        #[doc = "URL for downloading the asset.\n"]
        pub url: ::sidex_embedded::types::Str<'a>,
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for IssueAssetDownloadUrlOutput<'a> {
        const IS_RECORD: bool = true;
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            let fields = reader.record(&[&["url"]])?;
            ::core::result::Result::Ok(Self {
                url: fields.required(0usize)?,
            })
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for IssueAssetDownloadUrlOutput<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            let mut object = writer.object()?;
            ::sidex_embedded::Encode::encode_fields(self, &mut object)?;
            object.end()
        }
        fn is_record(&self) -> bool {
            true
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.url, object.field("url")?)?;
            ::core::result::Result::Ok(())
        }
    }
}
pub mod users {
    #![doc = "Actions and types related to users.\n"]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, deprecated)]
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a user.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct UserId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for UserId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for UserId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for UserId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
    #[doc = "**Group `admin`:** Part of the Admin API, which requires instance administrator permissions. Backward-incompatible changes are more frequent than for other actions.\n\n**Group `device`:** Part of the Device API, which Nexigon Agent and other software on devices use. Changes remain backward compatible for at least five years.\n\nUnique ID of a user token.\n"]
    #[derive(Clone, Copy, Debug)]
    pub struct UserTokenId<'a>(pub ::sidex_embedded::types::Str<'a>);
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Decode<'a> for UserTokenId<'a> {
        fn decode(
            reader: &mut ::sidex_embedded::json::Reader<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeError> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::Decode<'a>>::decode(reader)
                .map(Self)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::Encode for UserTokenId<'a> {
        fn encode(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode(&self.0, writer)
        }
        fn encode_fields(
            &self,
            object: &mut ::sidex_embedded::json::ObjectWriter<'_, '_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::Encode::encode_fields(&self.0, object)
        }
    }
    #[automatically_derived]
    impl<'a> ::sidex_embedded::MapKey<'a> for UserTokenId<'a> {
        fn decode_key(
            key: ::sidex_embedded::types::Str<'a>,
        ) -> ::core::result::Result<Self, ::sidex_embedded::DecodeErrorKind> {
            <::sidex_embedded::types::Str<'a> as ::sidex_embedded::MapKey<'a>>::decode_key(key)
                .map(Self)
        }
        fn encode_key(
            &self,
            writer: &mut ::sidex_embedded::json::Writer<'_>,
        ) -> ::core::result::Result<(), ::sidex_embedded::EncodeError> {
            ::sidex_embedded::MapKey::encode_key(&self.0, writer)
        }
    }
}
