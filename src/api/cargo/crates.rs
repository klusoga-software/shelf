use crate::api::cargo::models::Metadata;
use crate::error::Error;
use actix_web::web::{Buf, Bytes};
use actix_web::{put, HttpResponse, Responder};
use log::info;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};

#[put("/api/v1/crates/new")]
pub async fn upload(body: Bytes) -> impl Responder {
    match parse_crate(body.reader()) {
        Ok(_) => {}
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    HttpResponse::Ok().into()
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
            return Err(Error::CrateParseError(format!(
                "Failed to parse metadata: {}",
                err
            )))
        }
    };

    info!("{:?}", metadata);

    let crate_len = read_u32_le(&mut reader)? as usize;

    info!("Crate size: {}", crate_len);

    let mut crate_buffer = vec![0u8; crate_len];

    reader.read_exact(&mut crate_buffer)?;

    let mut crate_file = File::create("crate.crate")?;

    crate_file.write_all(crate_buffer.as_slice())?;

    Ok(())
}
