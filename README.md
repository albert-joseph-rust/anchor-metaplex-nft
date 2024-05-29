### Solana NFT Mint with Metaplex

This project is based on [Mint Reference](https://hackernoon.com/how-to-mint-solana-nft-using-anchor-and-metaplex), but I updated the dependencies to the latest version.
Here are the modules I used.

```
anchor-lang = { version = "0.30.0", features = ["init-if-needed"] }
anchor-spl = { version = "0.30.0", features = ["metadata"] }
mpl-token-metadata = "4.1.2"
```

The project is working with latest version and you can find the deploy results here in devnet.
[TX](https://explorer.solana.com/tx/4RNbr3GWt3gx5hTd3uACnejiaVC3zNkXowLjQoqtTKAwaLjQiKeGXjBSJTRLeX632QBhmgvDpfyoikzQ3heMRtb3?cluster=devnet)
[Mint Account](https://explorer.solana.com/address/CwUYGhu2brUo69scPaCiL4MDV8tPSCaaBm1pbHG2H4XD?cluster=devnet)
[Mint](https://explorer.solana.com/address/DTtVGR7C4ErbhsXcMGix9HE4pyjTjwYuWcut3HCfX61r?cluster=devnet)