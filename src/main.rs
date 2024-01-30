use bdk::{Wallet, SyncOptions};
use bdk::database::MemoryDatabase;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::bitcoin::Network;
use bdk::wallet::AddressIndex;
use std::str::FromStr;
use bdk::bitcoin::Address;
use bdk::SignOptions;
use bdk::blockchain::Blockchain;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);
    let wallet = Wallet::new(
        "wpkh([373ce4ed/84h/1h/0h]tprv8ZgxMBicQKsPek81SH2DXa6cwyrHV9Q8cFeMPEk1jaYLPC3WN8S5UXdeXgdXWks6h6VGZR89H6oowPMWeD6S5X8FJ6FMdj8MLFqz1L3aWoc/0/*)",
        Some("wpkh([373ce4ed/84h/1h/0h]tprv8ZgxMBicQKsPek81SH2DXa6cwyrHV9Q8cFeMPEk1jaYLPC3WN8S5UXdeXgdXWks6h6VGZR89H6oowPMWeD6S5X8FJ6FMdj8MLFqz1L3aWoc/1/*)"),
        Network::Testnet,
        MemoryDatabase::default(),
    )?;

    wallet.sync(&blockchain, SyncOptions::default())?;

    println!("Descriptor balance: {} SAT", wallet.get_balance()?);

//tb1qa8ss7tlygqx72h9k3zr0c2tes9pnwu30umtqjw  => address
    println!("Address: {}", wallet.get_address(AddressIndex::New)?);
    let faucet_address = Address::from_str("mwZPfTMiVQEzXTEhwLmi5dcmq7EvcDqLGr")?;

//transaction builder
    let mut tx_builder = wallet.build_tx();
        tx_builder
        .drain_wallet()
        .drain_to(faucet_address.payload.script_pubkey());

    let (mut psbt, tx_details) = tx_builder.finish()?;

    println!("Transaction details: {:#?}", tx_details);
    let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
    assert!(finalized, "Tx has not been finalized");
    println!("Transaction Signed: {}", finalized);
    
    
    let raw_transaction = psbt.extract_tx();
    let txid = raw_transaction.txid();
    blockchain.broadcast(&raw_transaction)?;
    println!("Transaction sent! TXID: {txid}.\nExplorer URL: https://blockstream.info/testnet/tx/{txid}", txid = txid);

    Ok(())


    
}