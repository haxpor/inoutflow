use clap::Parser;

/// Commandline's arguments type
#[derive(Debug, Parser)]
#[clap(author="Wasin Thonkaew (wasin@wasin.io)")]
#[clap(name="inoutflow")]
#[clap(about="Commandline program to compute and print in/out flow of native tokens of EVM-based chains (BSC, Ethereum, or Polygon) of the wallet/contract address")]
pub struct CommandlineArgs {
    /// Contract address to process its in/out flow of native token
    #[clap(index=1, required=true)]
    pub address: String,

    /// Which chain to work with.
    #[clap(long="chain", short='c', required=true, multiple_values=false, possible_values=["bsc", "ethereum", "polygon"], ignore_case=true)]
    pub chain: String,
}
