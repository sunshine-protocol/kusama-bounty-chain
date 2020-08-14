# Kusama Bounty Chain + Bot
> *description word count*: 119

This substrate chain implementation supports posting and crowdfunding bounties. 

`Post` bounty user flow:
1. Open an issue on github with the bounty's requirements
2. Use the mobile interface to post a new bounty. 

`Submit` entry user flow:
1. Open a separate issue to track the submission's review.
2. Use the mobile interface to post a new bounty submission. 

Upon transaction inclusion, a bot watching the chain posts in the github issues with success messages containing relevant on-chain data (i.e. amount backing the bounty, amount requested by the submission, etc). 

Submission approval and outside contributions are handled in the mobile interface, and the bot emits relevant messages in the Bounty/Submission github issues to report any changes to on-chain state.

### Did this code exist before the submission period?

The code was completed during the submission period. Neither the bot code nor the bounty module worked with the Rust client until this PR https://github.com/sunshine-protocol/sunshine-bounty/pull/144 was merged August 9th. Since then, a few more PRs were required to set up the testnet, achieve compatibility with the interface, and debug the github bot. The project was completed the final day of submissions.

## Demo Instructions

### Local Testnet

There is a live testnet launched from [sunshine](https://github.com/sunshine-protocol/sunshine). To demo the bounty interface with the live testnet and live bot, follow the instructions on the README to build the [mobile application](https://github.com/sunshine-protocol/sunshine-bounty-ui). **[@shekohex](https://github.com/shekohex)** posted a video demo [here](https://www.youtube.com/watch?v=bQSYjOT1R04&feature=youtu.be).

### Building Code Locally

**Run the bot** in a terminal window.
```sh
source config.sh # initialize the GITHUB_TOKEN env var for the bot
cd bot
cargo run # Ctrl + C to kill
```

**Run the node** in dev mode.
```sh
cd node
cargo build --release
cd ..
./target/release/kb-test-node --dev
```

`Command + C` to kill the node.

To purge the chain from the root of this directory
```sh
./target/release/kb-test-node purge-chain --dev
```

To demo, ensure the node and bot are running. Then, run the tests in `/demo`
```sh
todo!()
```


