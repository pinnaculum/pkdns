use std::{fs::read_to_string, path::{Path, PathBuf}};

use chrono::Duration;
use clap::ArgMatches;
use pkarr::{Keypair, SignedPacket};

use crate::{pkarr_publisher::PkarrPublisher, simple_zone::SimpleZone};


const SECRET_KEY_LENGTH: usize = 32;


pub fn read_zone_file(unexpanded_path: &String, pubkey: &str) -> SimpleZone {
    let csv_path_str: String = shellexpand::full(unexpanded_path)
        .expect("Valid shell path.")
        .into();
    let path = Path::new(&csv_path_str);
    let path = PathBuf::from(path);

    let zone = read_to_string(path);
    if let Err(e) = zone {
        eprintln!("Failed to read zone at {csv_path_str}. {e}");
        std::process::exit(1);
    };
    let zone = zone.unwrap();

    let zone = SimpleZone::read(zone, pubkey);
    if let Err(e) = zone {
        eprintln!("Failed to parse zone file. {e}");
        std::process::exit(1);
    };
    zone.unwrap()
}

pub fn read_seed_file(unexpanded_path: &String) -> Keypair {
    let expanded_path: String = shellexpand::full(unexpanded_path)
        .expect("Valid shell path.")
        .into();
    let path = Path::new(&expanded_path);
    let path = PathBuf::from(path);

    let seed = read_to_string(path);
    if let Err(e) = seed {
        eprintln!("Failed to read seed at {expanded_path}. {e}");
        std::process::exit(1);
    };
    let seed = seed.unwrap();
    parse_seed(&seed)
}

fn parse_seed(seed: &str) -> Keypair {
    let seed = seed.trim();
    let decode_result = zbase32::decode_full_bytes_str(&seed);
    if let Err(e) = decode_result {
        eprintln!("Failed to parse the seed file. {e} {seed}");
        std::process::exit(1);
    };

    let plain_secret = decode_result.unwrap();

    let slice: &[u8; SECRET_KEY_LENGTH] = &plain_secret[0..SECRET_KEY_LENGTH].try_into().unwrap();
    let keypair = Keypair::from_secret_key(slice);
    keypair
}
