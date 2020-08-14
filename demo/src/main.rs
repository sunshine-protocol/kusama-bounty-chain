use rand::{
    rngs::OsRng,
    RngCore,
};
use kb_client::{
    bounty::{
        BountyClient,
        BountyPostedEvent,
    },
    client::Client as _,
    mock::{
        test_node,
        AccountKeyring,
        Client,
    },
    utils::bounty::BountyInformation,
    BountyBody,
};

#[async_std::main]
async fn main() -> Result<()> {
    let (node, _node_tmp) = test_node();
    let client = Client::mock(&node, AccountKeyring::Alice).await;
    let alice_account_id = AccountKeyring::Alice.to_account_id();
    let bounty = BountyBody {
        repo_owner: "sunshine-protocol".to_string(),
        repo_name: "sunshine-bounty".to_string(),
        issue_number: 124,
    };
    let event = client.post_bounty(bounty, 10u128).await.unwrap();
    let submission = BountyBody {
        repo_owner: "sunshine-protocol".to_string(),
        repo_name: "sunshine-bounty".to_string(),
        issue_number: 124,
    };
}
