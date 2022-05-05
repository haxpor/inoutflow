# inoutflow
A command line program to compute and print out in/out flow of native tokens of EVM-based chains (BSC, Ethereum, and Polygon) of the wallet/contract address

# How it works

It will involve utilizing 3 related APIs of provided by upstream API platforms (bscscan.com, etherscan.io, or polygonscan.com)

1. Get list of normal transactions
2. Get list of internal transactions
3. Get address's balance

in order to have an understanding of native token (BNB, ETH, or MATIC) and balance respectively for
each type of transaction as well as final balance for entire address.

# Technical Tips

We can manually compute balance of target address ourselves without requesting
to balance API by sum in/out flow of native tokens from normal, and internal transactions,
and subtract by fees from normal transactions.

We have no need to try to get fees from internal transactions because internal
transaction is part of normal transaction, and its `gasPrice` and `gasUsed` fields
are not available as part of API returned. So only fees from normal transactions
are enough.

In short,

```
Address balance = in/out of native-token flow of normal txs + in/out of native-token flow of internal txs + fees of normal txs
```

**NOTE**: We have no need to involve ERC-20/BEP-20, and ERC-721/BEP-721 transactions here because
they are also as part of normal transaction as they shared the same transaction hash.
It just happens that for this type of transaction, it sends `0` native tokens but other tokens.
So normal transactions' fees cover everything we need.

# Setup

1. Register an account on [bscscan.com](https://bscscan.com), [etherscan.io](https://etherscan.io/), and [polygonscan.com](https://polygonscan.com/).
2. Create a new API key for all of API platforms in step 1.
3. Define the following environment variables
    * `INOUTFLOW_BSCSCAN_APIKEY` - for working with Binance Smart Chain (BSC)
    * `INOUTFLOW_ETHERSCAN_APIKEY` - for working with Ethereum
    * `INOUTFLOW_POLYGONSCAN_APIKEY` - for working with Polygon
4. You might need to source it e.g. `source ~/.bash_aliases`.

# Usage

```
inoutflow --chain <CHAIN> <ADDRESS>
```

where `--chain` (or `-c`)'s possible values are `bsc`, `ethereum`, or `polygon`.
So the input address will be based on such specified chain.

Sample output

1. Wallet address on BSC

```bash
$ inoutflow 0x5a52e96bacdabb82fd05763e25335261b270efcb --chain bsc
Found 194 txs!
- BNB outflow: 500000.0001 BNBs
- BNB inflow: 507000.07693536405 BNBs
- Net in/out balance: 7000.076835364045 BNBs

Found 25 internal txs!
- BNB outflow: 0 BNBs
- BNB inflow: 0.000000339984886578 BNBs
- Net in/out balance: 0.000000339984886578 BNBs

Total balance: 7000.052775939041 BNBs
```

2. Wallet address on Ethereum

```bash
$ inoutflow 0x49a2dcc237a65cc1f412ed47e0594602f6141936 --chain ethereum
Found 3930 txs!
- ETH outflow: 253655.5178577849 ETHs
- ETH inflow: 51001.49522086296 ETHs
- Net in/out balance: -202654.02263692193 ETHs

Found 482 internal txs!
- ETH outflow: 0 ETHs
- ETH inflow: 203349.21092720772 ETHs
- Net in/out balance: 203349.21092720772 ETHs

Total balance: 643.5772304060517 ETHs
```

3. Wallet address on Polygon

```bash
$ inoutflow 0xf89d7b9c864f589bbf53a82105107622b35eaa40 --chain polygon
Found 161 txs!
- MATIC outflow: 3258607.025878074 MATICs
- MATIC inflow: 0.001 MATICs
- Net in/out balance: -3258607.024878074 MATICs

Found 38 internal txs!
- MATIC outflow: 0 MATICs
- MATIC inflow: 4004106.7107583876 MATICs
- Net in/out balance: 4004106.7107583876 MATICs

Total balance: 745498.6568908329 MATICs
```

# License
MIT, Wasin Thonkaew
