use std::convert::Infallible;

use amplify::hex::FromHex;
use bp::dbc::Method;
use rgb_schemata::NonInflatableAsset;
use rgbstd::{interface::{FilterIncludeAll, FungibleAllocation, IfaceClass, IssuerClass, Rgb20}, invoice::Precision, persistence::{Inventory, Stock}, resolvers::ResolveHeight, validation::{ResolveWitness, WitnessResolverError}, Outpoint, Txid, WitnessAnchor, WitnessId, XAnchor, XPubWitness};
use strict_types::StrictDumb;

#[ic_cdk::query]
fn greet(name: String) -> String {
    let beneficiary_txid =
        Txid::from_hex("d6afd1233f2c3a7228ae2f07d64b2091db0d66f2e8ef169cf01217617f51b8fb").unwrap();
    let beneficiary = Outpoint::new(beneficiary_txid, 1);

    let contract = NonInflatableAsset::testnet("TEST", "Test asset", None, Precision::CentiMicro)
        .expect("invalid contract data")
        .allocate(Method::TapretFirst, beneficiary, 100_000_000_000_u64.into())
        .expect("invalid allocations")
        .issue_contract()
        .expect("invalid contract data");

    let contract_id = contract.contract_id();

    // Let's create some stock - an in-memory stash and inventory around it:
    let mut stock = Stock::default();   
    stock.import_iface(Rgb20::iface()).unwrap();
    stock.import_schema(NonInflatableAsset::schema()).unwrap();
    stock.import_iface_impl(NonInflatableAsset::issue_impl()).unwrap();     // this code will get IC5022 ERROR when running IC Canister

    // stock.import_contract(contract, &mut DumbResolver).unwrap();

    // // Reading contract state through the interface from the stock:
    // let contract = stock.contract_iface_id(contract_id, Rgb20::iface().iface_id()).unwrap();
    // let contract = Rgb20::from(contract);
  

    ic_cdk::api::print(format!("Hello from IC debugger, {}, {}", name, beneficiary));
    println!("Hello from WASI: {}, {}", name, beneficiary);

    format!("Hello, {}, {}!", name, beneficiary)
    // format!("Hello, {}, {}, Total supply: {:?}", name, beneficiary, contract.total_supply())
}

#[ic_cdk::init]
fn init() {
    unsafe {
        ic_wasi_polyfill::init(&[0u8; 32], &[]);
    };
}

struct DumbResolver;

impl ResolveWitness for DumbResolver {
    fn resolve_pub_witness(&self, _: WitnessId) -> Result<XPubWitness, WitnessResolverError> {
        Ok(XPubWitness::strict_dumb())
    }
}

impl ResolveHeight for DumbResolver {
    type Error = Infallible;
    fn resolve_anchor(&mut self, _: &XAnchor) -> Result<WitnessAnchor, Self::Error> {
        Ok(WitnessAnchor::strict_dumb())
    }
}