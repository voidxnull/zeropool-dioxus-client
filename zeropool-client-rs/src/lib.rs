use libzeropool_rs::client::TxType;
use libzeropool_rs::libzeropool::fawkes_crypto::backend::bellman_groth16::engines::Bn256;
use libzeropool_rs::libzeropool::fawkes_crypto::backend::bellman_groth16::prover::Proof;
use libzeropool_rs::libzeropool::native::params::{PoolBN256, PoolParams as PoolParamsTrait};
use libzeropool_rs::libzeropool::fawkes_crypto::ff_uint::Num;
use reqwest::Url;
use serde::Deserialize;

pub type PoolParams = PoolBN256;
pub type Fr = <PoolParams as PoolParamsTrait>::Fr;
pub type Fs = <PoolParams as PoolParamsTrait>::Fs;
pub type Engine = Bn256;
pub type SnarkProof = Proof<Engine>;

pub fn test_prover() {
    use std::time::Instant;

    use libzeropool_rs::libzeropool::fawkes_crypto::backend::bellman_groth16::engines::Bn256;
    use libzeropool_rs::libzeropool::fawkes_crypto::backend::bellman_groth16::Parameters;
    use libzeropool_rs::libzeropool::fawkes_crypto::ff_uint::Num;
    use libzeropool_rs::libzeropool::native::boundednum::BoundedNum;
    use libzeropool_rs::libzeropool::POOL_PARAMS;
    use libzeropool_rs::client::state::State;
    use libzeropool_rs::client::{TxType, UserAccount};
    use libzeropool_rs::proof::prove_tx;

    println!("Start");
    let state = State::init_test(POOL_PARAMS.clone());
    let acc = UserAccount::from_seed(&[0], state, POOL_PARAMS.clone());

    let tx = acc
        .create_tx(
            TxType::Deposit(
                BoundedNum::new(Num::from(0)),
                vec![],
                BoundedNum::new(Num::from(1)),
                vec![],
            ),
            None,
        )
        .unwrap();

    let data = std::fs::read("./transfer_params.bin").unwrap();
    let params = Parameters::<Bn256>::read(&mut data.as_slice(), true, true).unwrap();

    println!("Prove");
    for _ in 0..10 {
        let t = Instant::now();
        prove_tx(&params, &*POOL_PARAMS, tx.public.clone(), tx.secret.clone());
        println!("{:?}", t.elapsed());
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayerInfo {
    pub root: Num<Fr>,
    pub delta_index: u64,
}

pub async fn relayer_info(url: &str) -> RelayerInfo {
    let url = Url::parse(url).unwrap().join("/info").unwrap();
    let res = reqwest::get(url).await.unwrap();
    res.json().await.unwrap()
}

pub async fn fetch_transactions(url: &str, offset: u64, limit: u64) -> Vec<String> {
    let mut url = Url::parse(url).unwrap().join("/transactions").unwrap();
    url.query_pairs_mut().extend_pairs([
        ("offset", offset.to_string().as_str()),
        ("limit", limit.to_string().as_str()),
    ].iter().cloned());
    let res = reqwest::get(url).await.unwrap();
    res.json().await.unwrap()
}

// pub async fn send_transaction(url: &str, proof: SnarkProof, memo: &[u8], tx_type: TxType<Fr>, deposit_signature: Option<String>) -> u64 {
//     let mut url = Url::parse(url).unwrap().join("/transactions").unwrap();
// }

/*
async function fetchTransactions(relayerUrl: string, offset: BigInt, limit: number = 100): Promise<string[]> {
  const url = new URL(`/transactions`, relayerUrl);
  url.searchParams.set('limit', limit.toString());
  url.searchParams.set('offset', offset.toString());
  const res = await (await fetch(url.toString())).json();

  return res;
}

// returns transaction job ID
async function sendTransaction(relayerUrl: string, proof: Proof, memo: string, txType: TxType, depositSignature?: string): Promise<string> {
  const url = new URL('/transaction', relayerUrl);
  const res = await fetch(url.toString(), { method: 'POST', body: JSON.stringify({ proof, memo, txType, depositSignature }) });

  if (!res.ok) {
    const body = await res.json();
    throw new Error(`Error ${res.status}: ${JSON.stringify(body)}`)
  }

  const json = await res.json();
  return json.jobId;
}

async function getJob(relayerUrl: string, id: string): Promise<{ state: string, txHash: string } | null> {
  const url = new URL(`/job/${id}`, relayerUrl);
  const res = await (await fetch(url.toString())).json();

  if (typeof res === 'string') {
    return null;
  } else {
    return res;
  }
}

async function info(relayerUrl: string): Promise<RelayerInfo> {
  const url = new URL('/info', relayerUrl);
  const res = await fetch(url.toString());

  return await res.json();
}
 */