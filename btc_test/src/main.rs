use bitcoincore_rpc::bitcoin::{Address, Amount};
use bitcoincore_rpc::{json, Auth, Client, RpcApi};
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let rpc = Client::new(
        "http://localhost:18443".to_string(),
        Auth::UserPass("bitcoin".to_string(), "password".to_string()),
    )
    .unwrap();

    let mining_info = rpc.get_mining_info().unwrap();
    println!("{:#?}", mining_info);

    //let res = rpc.stop().unwrap();
    //println!("{:?}", res);

    // Look up address - not working on newer bitcond versions
    let addr = Address::from_str("bcrt1qanga5jxx845q82h9qgjfuedps92lktqv073qct").unwrap();
    let addr_info = rpc.get_address_info(&addr).unwrap();
    println!("{:?}", addr_info);

    // Look up funds
    let balance = rpc.get_balance(None, None).unwrap();
    println!("Balance: {:?} BTC", balance.as_btc());

    // Generate a new address
    let myaddress = rpc
        .get_new_address(Option::Some("bbb"), Option::Some(json::AddressType::Bech32))
        .unwrap();
    println!("address: {:?}", myaddress);

    // Lets list unspent transaction with at least 20 BTC:
    let unspent = rpc
        .list_unspent(
            None,
            None,
            None,
            None,
            Option::Some(json::ListUnspentQueryOptions {
                minimum_amount: Option::Some(Amount::from_btc(3.0).unwrap()),
                maximum_amount: None,
                maximum_count: None,
                minimum_sum_amount: None,
            }),
        )
        .unwrap();

    let selected_tx = &unspent[0];
    println!("selected unspent transaction: {:#?}", selected_tx);

    let unspent_amount = selected_tx.amount;

    let selected_utxos = json::CreateRawTransactionInput {
        txid: selected_tx.txid,
        vout: selected_tx.vout,
        sequence: None,
    };

    let recipient = Address::from_str("bcrt1q6rhpng9evdsfnn833a4f4vej0asu6dk5srld6x").unwrap();
    println!("recipient: {:?}", recipient);

    // send all bitcoin in the UTXO except a minor value which will be paid to miners
    let amount = unspent_amount - Amount::from_btc(0.00001).unwrap();

    let mut output = HashMap::new();
    output.insert(
        "bcrt1q6rhpng9evdsfnn833a4f4vej0asu6dk5srld6x".to_string(),
        amount,
    );

    let unsigned_tx = rpc
        .create_raw_transaction(&[selected_utxos], &output, None, None)
        .unwrap();

    println!("unsigned tx {:#?}", unsigned_tx);

    // sign transaction
    let signed_tx = rpc
        .sign_raw_transaction_with_wallet(&unsigned_tx, None, None)
        .unwrap();

    println!("singed tx {:?}", signed_tx.transaction().unwrap());

    // broadcast transaction
    let txid_sent = rpc
        .send_raw_transaction(&signed_tx.transaction().unwrap())
        .unwrap();

    println!("{:?}", txid_sent);
}
