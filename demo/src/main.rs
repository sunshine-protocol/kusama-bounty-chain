use kb_client::{
    bounty::BountyClient,
    mock::{
        test_node,
        AccountKeyring,
        Client,
    },
    BountyBody,
};
use sunshine_cli_utils::Result;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::try_init().ok();
    let (node, _node_tmp) = test_node();
    let alice_client = Client::mock(&node, AccountKeyring::Alice).await;
    let bob_client = Client::mock(&node, AccountKeyring::Bob).await;
    let bounty = BountyBody {
        repo_owner: "sunshine-protocol".to_string(),
        repo_name: "sunshine-bounty".to_string(),
        issue_number: 160,
    };
    // (1) Alice posts a bounty!
    let _ = alice_client.post_bounty(bounty, 2000).await.unwrap();
    let submission = BountyBody {
        repo_owner: "sunshine-protocol".to_string(),
        repo_name: "sunshine-bounty".to_string(),
        issue_number: 161,
    };
    // (2) Bob submits for Alice's bounty!
    let _ = bob_client
        .submit_for_bounty(1, submission, 1000)
        .await
        .unwrap();
    // (3) Alice approves Bob's submission
    let _ = alice_client.approve_bounty_submission(1).await.unwrap();
    Ok(())
}
