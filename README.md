# inoutflow-bsc
A command line program to compute and print out in/out flow of BNB of the target wallet/contract address

# How it works

It will involve utilizing 2 related APIs of [bscscan.com](https://bscscan.com) as follows

1. Get list of normal transactions - [link](https://docs.bscscan.com/api-endpoints/accounts#get-a-list-of-normal-transactions-by-address)
2. Get list of internal transactions - [link](https://docs.bscscan.com/api-endpoints/accounts#get-a-list-of-internal-transactions-by-address)

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
Found 436 transactions!
- BNB outflow: 0 BNBs
- BNB inflow: 10.03534257 BNBs
- BNB balance: 10.03534257 BNBs

Found 2 internal transactions!
- BNB outflow: 10.035 BNBs
- BNB inflow: 0 BNBs
- BNB balance: -10.035 BNBs

Total balance: 0.0003425699999990428 BNBs
```

# License
MIT, Wasin Thonkaew
