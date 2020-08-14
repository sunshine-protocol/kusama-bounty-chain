use ipld_block_builder::{
    derive_cache,
    Codec,
    IpldCache,
};
use libipld::store::Store;
use std::sync::Arc;
use substrate_subxt::{
    balances::{
        AccountData,
        Balances,
    },
    extrinsic,
    sp_core,
    sp_runtime,
    sp_runtime::traits::{
        IdentifyAccount,
        Verify,
    },
    system::System,
};
use sunshine_bounty_client::{
    bank::Bank,
    bounty::Bounty,
    donate::Donate,
    org::Org,
    vote::Vote,
};
use sunshine_client_utils::{
    cid::CidBytes,
    client::{
        GenericClient,
        KeystoreImpl,
        OffchainStoreImpl,
    },
    crypto::{
        keychain::KeyType,
        sr25519,
    },
    node::{
        ChainSpecError,
        Configuration,
        NodeConfig,
        RpcHandlers,
        ScServiceError,
        TaskManager,
    },
};

pub use sunshine_bounty_client::*;
pub use sunshine_bounty_utils as utils;
pub use sunshine_client_utils as client;

pub type AccountId = <<sp_runtime::MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Uid = u32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Runtime;

impl System for Runtime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Address = AccountId;
    type Header = sp_runtime::generic::Header<Self::BlockNumber, Self::Hashing>;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
    type AccountData = AccountData<u128>;
}

impl Balances for Runtime {
    type Balance = u128;
}

impl Bounty for Runtime {
    type IpfsReference = CidBytes;
    type BountyId = u64;
    type BountyPost = BountyBody;
    type SubmissionId = u64;
    type BountySubmission = BountyBody;
}

impl sunshine_identity_client::Identity for Runtime {
    type Uid = u8;
    type Cid = CidBytes;
    type Mask = u8;
    type Gen = u16;
    type IdAccountData = ();
}

impl sunshine_faucet_client::Faucet for Runtime {}

impl substrate_subxt::Runtime for Runtime {
    type Signature = sp_runtime::MultiSignature;
    type Extra = extrinsic::DefaultExtra<Self>;
}

pub struct OffchainClient<S> {
    bounties: IpldCache<S, Codec, BountyBody>,
    constitutions: IpldCache<S, Codec, TextBlock>,
}

impl<S: Store> OffchainClient<S> {
    pub fn new(store: S) -> Self {
        Self {
            bounties: IpldCache::new(store.clone(), Codec::new(), 64),
            constitutions: IpldCache::new(store, Codec::new(), 64),
        }
    }
}

derive_cache!(OffchainClient, bounties, Codec, BountyBody);
derive_cache!(OffchainClient, constitutions, Codec, TextBlock);

impl<S: Store> From<S> for OffchainClient<S> {
    fn from(store: S) -> Self {
        Self::new(store)
    }
}

pub struct Node;

impl NodeConfig for Node {
    type ChainSpec = kb_test_node::chain_spec::ChainSpec;
    type Runtime = Runtime;

    fn impl_name() -> &'static str {
        kb_test_node::IMPL_NAME
    }

    fn impl_version() -> &'static str {
        kb_test_node::IMPL_VERSION
    }

    fn author() -> &'static str {
        kb_test_node::AUTHOR
    }

    fn copyright_start_year() -> i32 {
        kb_test_node::COPYRIGHT_START_YEAR
    }

    fn chain_spec_dev() -> Self::ChainSpec {
        kb_test_node::chain_spec::development_config()
    }

    fn chain_spec_from_json_bytes(
        json: Vec<u8>,
    ) -> Result<Self::ChainSpec, ChainSpecError> {
        Self::ChainSpec::from_json_bytes(json).map_err(ChainSpecError)
    }

    fn new_light(
        config: Configuration,
    ) -> Result<(TaskManager, Arc<RpcHandlers>), ScServiceError> {
        Ok(kb_test_node::service::new_light(config)?)
    }

    fn new_full(
        config: Configuration,
    ) -> Result<(TaskManager, Arc<RpcHandlers>), ScServiceError> {
        Ok(kb_test_node::service::new_full(config)?)
    }
}

pub struct UserDevice;

impl KeyType for UserDevice {
    const KEY_TYPE: u8 = 0;
    type Pair = sr25519::Pair;
}

pub type Client = GenericClient<
    Node,
    UserDevice,
    KeystoreImpl<UserDevice>,
    OffchainClient<OffchainStoreImpl>,
>;

#[cfg(feature = "mock")]
pub mod mock {
    use super::*;
    use sunshine_client_utils::mock::{
        self,
        build_kb_test_node,
        OffchainStoreImpl,
    };
    pub use sunshine_client_utils::mock::{
        AccountKeyring,
        TempDir,
        TestNode,
    };

    pub type Client = GenericClient<
        Node,
        UserDevice,
        mock::KeystoreImpl<UserDevice>,
        OffchainClient<OffchainStoreImpl>,
    >;

    pub type ClientWithKeystore = GenericClient<
        Node,
        UserDevice,
        KeystoreImpl<UserDevice>,
        OffchainClient<OffchainStoreImpl>,
    >;

    pub fn kb_test_node() -> (TestNode, TempDir) {
        build_kb_test_node::<Node>()
    }
}
