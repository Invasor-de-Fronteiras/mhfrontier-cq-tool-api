use std::fs::remove_file;
use std::{path::Path, fs::File};

use better_cursor::{BetterRead, BetterSeek, BetterWrite};
use crate::crypto::ecd::{self, EcdHeader};
use crate::pack::jpk;
use crate::utils::custom_error::CustomResult;

pub struct ProcessLog {
    pub header_detected: String,
    pub meta: String
}

pub fn process_file(input: &str, output: &str) -> CustomResult<Option<ProcessLog>> {
    let input_path = Path::new(input);
    let output_path = Path::new(output);

    let mut input_file = File::open(input_path)?;

    let header = input_file.read_current_u32()?;

    if ecd::is_ecd_header(header) {
        println!("ECD Header detected.");
        let mut reader = input_file.to_buffer_cursor()?;
        reader.read_current_u32()?;

        let decript_result = ecd::decrypt_ecd(&mut reader)?;
        println!("{:?}", decript_result.header);

        if output_path.exists() {
            remove_file(output_path)?;
        }

        let mut output_file = File::create(output_path)?;
        output_file.write_buffer(&decript_result.buffer)?;

        let output_meta_path = &output_path.with_extension("meta");
        if output_meta_path.exists() {
            remove_file(output_meta_path)?;
        }
        let mut meta_file = File::create(output_meta_path)?;

        meta_file.write_struct(&decript_result.header)?;

        return Ok(Some(ProcessLog {
            header_detected: String::from("ecd"),
            meta: serde_json::to_string_pretty(&decript_result.header)?
        }));
    }

    if jpk::is_jpk_header(header) {
        println!("JKR Header detected.");
        let mut reader = input_file.to_buffer_cursor()?;
        reader.read_current_u32()?;

        let original_len = reader.len()?;
        let unpacked_buffer = jpk::decode_jpk(&mut reader)?;
        if output_path.exists() {
            remove_file(output_path)?;
        }

        let mut output_file = File::create(output_path)?;
        output_file.write_buffer(&unpacked_buffer)?;
        let final_len = output_file.len()?;

        return Ok(Some(ProcessLog {
            header_detected: String::from("jpk"),
            meta: format!("original_len: {original_len}, final_len: {final_len}")
        }));
    }

    Ok(None)
}

pub fn process_file_deep(input: &str, output: &str) -> CustomResult<Vec<ProcessLog>> {
    let mut process_logs: Vec<ProcessLog> = vec![];
    let mut current_input = input;

    loop {
        if let Some(process_log) = process_file(current_input, output)? {
            if process_logs.len() == 0 {
                current_input = output;
            }

            process_logs.push(process_log);
            continue;
        }

        break;
    }

    Ok(process_logs)
}

pub fn load_ecd_header(input: &str) -> CustomResult<EcdHeader> {
    let input_path = Path::new(input);

    let mut input_file = File::open(input_path)?;
    let mut reader = input_file.to_buffer_cursor()?;

    let header = reader.read_struct::<EcdHeader>()?;

    Ok(header)
}

pub fn encrypt_ecd_file(input: &str, output: &str, header: EcdHeader) -> CustomResult<()> {
    let input_path = Path::new(input);
    let output_path = Path::new(output);

    let mut input_file = File::open(input_path)?;
    let mut reader = input_file.to_buffer_cursor()?;

    let buffer = ecd::encrypt_ecd(&mut reader, header)?;

    if output_path.exists() {
        remove_file(output_path)?;
    }

    let mut output_file = File::create(output_path)?;
    output_file.write_buffer(&buffer)?;

    Ok(())
}

pub fn encode_jpk_file(input: &str, output: &str, jpk_type: u16, level: u32) -> CustomResult<()> {
    let input_path = Path::new(input);
    let output_path = Path::new(output);

    let mut input_file = File::open(input_path)?;
    let mut reader = input_file.to_buffer_cursor()?;

    let buffer = jpk::encode_jpk(&mut reader, jpk_type, level)?;

    if output_path.exists() {
        remove_file(output_path)?;
    }

    let mut output_file = File::create(output_path)?;
    output_file.write_buffer(&buffer)?;
    
    Ok(())
}