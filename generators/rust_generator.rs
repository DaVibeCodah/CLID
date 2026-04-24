use std::time::{SystemTime, UNIX_EPOCH};

const BASE62: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn base62_encode(mut num: u64) -> String {
    if num == 0 {
        return String::from(BASE62[0] as char);
    }
    let mut result = Vec::new();
    while num > 0 {
        result.push(BASE62[(num % 62) as usize]);
        num /= 62;
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

fn random_string(length: usize) -> String {
    let mut result = String::with_capacity(length);
    for _ in 0..length {
        let idx = rand::random::<usize>() % 62;
        result.push(BASE62[idx] as char);
    }
    result
}

fn timestamp_part() -> String {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    base62_encode(since_epoch.as_secs())
}

fn generate_clid(format_type: &str, with_timestamp: bool) -> String {
    match format_type {
        "CLID-1" => {
            let mut parts: Vec<String> = (0..6).map(|_| random_string(4)).collect();
            if with_timestamp {
                parts[0] = timestamp_part();
            }
            parts.join("-")
        }
        "CLID-2" => {
            if with_timestamp {
                format!(
                    "{}{}{}{}{}{}",
                    timestamp_part(),
                    random_string(4),
                    random_string(4),
                    random_string(4),
                    random_string(4),
                    random_string(4)
                )
            } else {
                format!(
                    "{}{}{}{}{}{}",
                    random_string(4),
                    random_string(4),
                    random_string(4),
                    random_string(4),
                    random_string(4),
                    random_string(4)
                )
            }
        }
        "CLID-3" => {
            if with_timestamp {
                format!("{}{}", timestamp_part(), random_string(15))
            } else {
                random_string(21)
            }
        }
        "CLID-4" => {
            if with_timestamp {
                format!("{}{}", timestamp_part(), random_string(14))
            } else {
                random_string(20)
            }
        }
        "CLID-5" => {
            if with_timestamp {
                format!("{}{}", timestamp_part(), random_string(10))
            } else {
                random_string(16)
            }
        }
        _ => String::from("Unknown format"),
    }
}

fn main() {
    println!("=== CLID Generator ===");
    println!("CLID-1:      {}", generate_clid("CLID-1", false));
    println!("CLID-1-TS:   {}", generate_clid("CLID-1", true));
    println!("CLID-2:      {}", generate_clid("CLID-2", false));
    println!("CLID-2-TS:   {}", generate_clid("CLID-2", true));
    println!("CLID-3:      {}", generate_clid("CLID-3", false));
    println!("CLID-3-TS:   {}", generate_clid("CLID-3", true));
    println!("CLID-4:      {}", generate_clid("CLID-4", false));
    println!("CLID-4-TS:   {}", generate_clid("CLID-4", true));
    println!("CLID-5:      {}", generate_clid("CLID-5", false));
    println!("CLID-5-TS:   {}", generate_clid("CLID-5", true));
}
