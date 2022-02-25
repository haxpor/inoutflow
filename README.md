# inoutflow-bsc
A command line program to compute and print out in/out flow of BNB of the target wallet/contract address

# How it works

It will involve utilizing 3 related APIs of [bscscan.com](https://bscscan.com) as follows

1. Get list of normal transactions - [link](https://docs.bscscan.com/api-endpoints/accounts#get-a-list-of-normal-transactions-by-address)
2. Get list of internal transactions - [link](https://docs.bscscan.com/api-endpoints/accounts#get-a-list-of-internal-transactions-by-address)
3. Get address's balance - [link](https://docs.bscscan.com/api-endpoints/accounts#get-bnb-balance-for-a-single-address)

in order to have an understanding of BNB in/out flow, and balance respectively for
each type of transaction as well as final balance for entire address.

# How to setup

1. Register an account on [bscscan.com](https://bscscan.com)
2. Create a new API key
3. Build project via `cargo build`.
3. Define environment variable namely `HX_INOUTFLOW_API_KEY` to api-key that you have from step 2.
4. Execute e.g. `cargo r -- <target-address>`

# Usage

```
inoutflow-bsc <target-address>
```

Sample output is similar to following

```
Found 594 txs!
- BNB outflow: 1837.683964544 BNBs
- BNB inflow: 2.006 BNBs
- Net in/out balance: -1835.677964544 BNBs

Found 134 internal txs!
- BNB outflow: 0 BNBs
- BNB inflow: 1836.96689 BNBs
- Net in/out balance: 1836.96689 BNBs

Total 728 txs
Total balance: 0.042507285 BNBs
```

# License
MIT, Wasin Thonkaew
