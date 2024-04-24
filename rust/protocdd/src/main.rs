pub mod disperser {
    tonic::include_proto!("disperser");
}

use crate::disperser::{BlobStatus, BlobStatusRequest};
use anyhow::{anyhow, Result};
use disperser::disperser_client::DisperserClient;
use disperser::{BlobStatusReply, DisperseBlobRequest, RetrieveBlobRequest, SecurityParams};
use futures::future::join_all;
use std::vec::Vec;

// retrieve_chunk
//
//       Retrieves a single chunk of data from the data availability provider
//
// params:
// batch_header_hash : The message that the operators will sign their signatures
// on.
// blob_index: index of blob in the batch
async fn retrieve_chunk(batch_header_hash: Vec<u8>, blob_index: u32) -> Result<Vec<u8>> {
    let mut client = DisperserClient::connect("https://disperser-goerli.eigenda.xyz:443").await?;
    let request = tonic::Request::new(RetrieveBlobRequest {
        blob_index,
        batch_header_hash,
    });

    let resp = client.retrieve_blob(request).await?;
    Ok(resp.into_inner().data)
}

// recontruct_blob
//
//       Retrieves a series of chunk and concatenates in the order requested.
//
// params:
//
// Vec<BlobStatusReply> : Ordered list of chunks that are confirmed from the
// data availibility layer.
async fn reconstruct_blob(blob_status: Vec<BlobStatusReply>) -> Result<Vec<u8>> {
    // Following code block simply extracts (blob_index, batch_header_hash).
    // The code complexity is due to most of Prost generated types of EigenDA
    // are unnecessarily wrapped as options types.
    let v = blob_status
        .into_iter()
        .map(|reply| {
            let proof = reply
                .info
                .ok_or(anyhow!("None() for BlobInfo"))?
                .blob_verification_proof
                .ok_or(anyhow!("None() for Verification Proof"))?;
            let blob_index = proof.blob_index;
            let batch_header_hash = proof
                .batch_metadata
                .ok_or(anyhow!("None() for BatchMetadata"))?
                .batch_header_hash;
            Ok::<_, anyhow::Error>((blob_index, batch_header_hash))
        })
        .collect::<Vec<_>>();

    // Collect and reconstruct all chunks
    let retrievals = v
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|(blob_index, batch_header_hash)| retrieve_chunk(batch_header_hash, blob_index))
        .collect::<Vec<_>>();
    let res = join_all(retrievals)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    Ok(res.into_iter().flatten().collect())
}

// disperse_blob_request
//
//       Helper function to generate default security parameters for dispersed
//       chunks
//
// params:
//
//       data: single unit of data to be dispersed on the data availability
//       layer.
fn disperse_blob_request(data: &[u8]) -> DisperseBlobRequest {
    disperser::DisperseBlobRequest {
        data: data.to_vec(),
        security_params: vec![SecurityParams {
            quorum_id: 0,
            adversary_threshold: 25,
            quorum_threshold: 50,
        }],
    }
}

// disperse_chunk
//
//       Disperses a single chunk of data to data availability provider
//
// params:
// chunk_id : logical sequence of the chunk within a blob
// data: data for the chunk
async fn disperse_chunk(chunk_idx: usize, data: &[u8]) -> Result<BlobStatusReply> {
    let mut client = DisperserClient::connect("https://disperser-goerli.eigenda.xyz:443").await?;
    let response = loop {
        let request = disperse_blob_request(&data);
        let request = tonic::Request::new(request);
        println!("Sending request to gRPC Server... {chunk_idx}");

        match client.disperse_blob(request).await {
            Ok(resp) => {
                break resp;
            }
            Err(_resp) => {
                println!("{_resp:?}");
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    };

    let request_id = response.into_inner().request_id.clone();
    let response = loop {
        let response = client
            .get_blob_status(BlobStatusRequest {
                request_id: request_id.clone(),
            })
            .await;
        let r = response.unwrap().into_inner();
        println!("{chunk_idx} Response {r:?}");
        let blob_status = BlobStatus::try_from(r.status).ok();
        if let Some(BlobStatus::Confirmed) = blob_status {
            break r;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await
    };
    Ok(response)
}

async fn disperse_blob(chunk_size: usize, data: &[u8]) -> Result<Vec<BlobStatusReply>> {
    let v = data
        .chunks(chunk_size)
        .into_iter()
        .enumerate()
        .map(|(chunk_idx, data)| disperse_chunk(chunk_idx, data))
        .collect::<Vec<_>>();
    join_all(v).await.into_iter().collect::<Result<Vec<_>, _>>()
}

const KB: usize = 1024;
const BLOB_SIZE: usize = 120;
const CHUNK_SIZE: usize = 40;

#[tokio::main]
async fn main() -> Result<()> {
    let mut data = Vec::<u8>::with_capacity(BLOB_SIZE);
    for i in 0..BLOB_SIZE {
        data.push(i as u8)
    }

    let responses = disperse_blob(CHUNK_SIZE, &data)
        .await
        .expect("availability proofs");
    let data = reconstruct_blob(responses).await.expect("retrieved data");
    for i in 0..BLOB_SIZE {
        assert_eq!(data[i], i as u8);
    }
    Ok(())
}
