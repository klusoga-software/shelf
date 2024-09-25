use crate::api::cargo::models::{CrateIndex, Metadata};
use crate::error::Error;
use actix_web::web::{Buf, Bytes};
use actix_web::{put, HttpResponse, Responder};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs::{read_to_string, DirBuilder, File, OpenOptions};
use std::io::Read;
use std::io::{self, Write};

#[put("/api/v1/crates/new")]
pub async fn upload(body: Bytes) -> impl Responder {
    match parse_crate(body.reader()) {
        Ok(_) => {}
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    HttpResponse::Ok().json(json!({}))
}

fn parse_crate<R: Read>(mut reader: R) -> Result<(), Error> {
    fn read_u32_le<R: Read>(reader: &mut R) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    let json_len = read_u32_le(&mut reader)? as usize;

    let mut json_buffer = vec![0u8; json_len];

    reader.read_exact(&mut json_buffer)?;

    let metadata: Metadata = match serde_json::from_slice(&json_buffer) {
        Ok(meta) => meta,
        Err(err) => {
            return Err(Error::CrateParse(format!(
                "Failed to parse metadata: {}",
                err
            )))
        }
    };

    let dir_name = get_dir_name(&metadata.name);

    DirBuilder::new()
        .recursive(true)
        .create(format!("assets/{}", &dir_name))?;

    let mut metadata_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(format!("assets/{}/{}", &dir_name, metadata.name))?;

    for line in read_to_string(format!("assets/{}/{}", &dir_name, metadata.name))?.lines() {
        let crate_metadata: CrateIndex = serde_json::from_str(line)?;
        if crate_metadata.vers == metadata.vers {
            return Err(Error::VersionExists(format!(
                "The version {} already exists",
                metadata.vers
            )));
        }
    }

    let crate_len = read_u32_le(&mut reader)? as usize;

    let mut crate_buffer = vec![0u8; crate_len];

    reader.read_exact(&mut crate_buffer)?;

    let mut hasher = Sha256::new();
    hasher.write_all(&crate_buffer)?;
    let checksum: String = format!("{:x}", hasher.finalize());

    let crate_index = CrateIndex::new_from_metadata(&metadata, checksum);

    writeln!(metadata_file, "{}", serde_json::to_string(&crate_index)?)?;

    let mut crate_file = File::create(format!(
        "crates/{}_{}.crate",
        &metadata.name, &metadata.vers
    ))?;

    crate_file.write_all(crate_buffer.as_slice())?;

    Ok(())
}

fn get_dir_name(crate_name: &str) -> String {
    let first = &crate_name[0..2];
    let second = &crate_name[2..4];

    format!("{}/{}", first, second)
}
