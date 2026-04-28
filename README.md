# CLID - Clean ID Formats

A collection of clean, human-readable identifier formats without unnecessary complexity.

## Format Specifications

### CLID-1
<4 char>-<4 char>-<4 char>-<4 char>-<4 char>-<4 char>

- Total length: 24 characters (excluding hyphens)
- Hyphens included for readability
- Character set: A-Z, a-z, 0-9 (case sensitive)
- Example: AbC1-dEfG-89Hi-JkL0-mNoP-QrS2
- 0.0048% collision probability on 31,622,776,601,683,790,000 generated ID's.

### CLID-1-TS
\<timestamp\>-<4 char>-<4 char>-<4 char>-<4 char>-<4 char>

- Timestamp (Base62 encoded) + 20 random characters
- Total length: timestamp length varies + 20 chars (excluding hyphens)
- Example: aB3xY-7kLm-NpQ9-RsTu-VwX0-YzA1

### CLID-2
<4 char><4 char><4 char><4 char><4 char><4 char>

- Total length: 24 characters (no separators)
- Character set: A-Z, a-z, 0-9 (case sensitive)
- Example: AbC1dEfG89HiJkL0mNoPQrS2
- 0.0048% collision probability on 31,622,776,601,683,790,000 generated ID's.

### CLID-2-TS
\<timestamp\><4 char><4 char><4 char><4 char><4 char>

- Timestamp (Base62 encoded) + 20 random characters
- Total length: timestamp length varies + 20 chars (no separators)
- Example: aB3xY7kLmNpQ9RsTuVwX0YzA1

### CLID-3
21 characters using Base62 encoding (A-Z, a-z, 0-9)

- Example: aB3xY7kLmNpQ9RsTuVwX0

### CLID-3-TS
\<timestamp\> + 15 random characters (Base62)

- Total length: timestamp length varies + 15 chars
- Example: xY9kLmNpQ9RsTuVwX0

### CLID-4
20 characters using Base62 encoding

- Example: aB3xY7kLmNpQ9RsTuVw

### CLID-4-TS
\<timestamp\> + 14 random characters (Base62)

- Total length: timestamp length varies + 14 chars
- Example: xY9kLmNpQ9RsTuVw

### CLID-5
16 characters using Base62 encoding

- Example: aB3xY7kLmNpQ9RsT

### CLID-5-TS
\<timestamp\> + 10 random characters (Base62)

- Total length: timestamp length varies + 10 chars
- Example: xY9kLmNpQ9R

## Summary Table

| Format | Separators | Total Char Length | Timestamp Support |
|--------|-----------|-------------------|-------------------|
| CLID-1 | Yes (hyphen) | 24 (excl. hyphens) | Optional (TS variant) |
| CLID-2 | No | 24 | Optional (TS variant) |
| CLID-3 | No | 21 | Optional (TS variant) |
| CLID-4 | No | 20 | Optional (TS variant) |
| CLID-5 | No | 16 | Optional (TS variant) |

## Character Set

All CLID formats use Base62 character set:
- Uppercase: A-Z (26 chars)
- Lowercase: a-z (26 chars)
- Numbers: 0-9 (10 chars)

Total: 62 possible characters per position

## Timestamp Encoding

When Timestamp (TS) variants are used:
- Current Unix timestamp is encoded to Base62
- Encoded length is typically 6 characters
- Timestamp always appears at the beginning of the ID

## Collision Probability

All formats use Base62 (62 possible characters per position).

| Format | Keyspace | Safe up to (1% risk) | 50% collision at |
|--------|----------|----------------------|------------------|
| CLID-1/2 | ~10^43 | ~4.6 × 10^20 IDs | ~2.7 × 10^21 IDs |
| CLID-3 | ~10^37 | ~9.3 × 10^17 IDs | ~1.7 × 10^18 IDs |
| CLID-4 | ~10^35 | ~3.7 × 10^17 IDs | ~7.0 × 10^17 IDs |
| CLID-5 | ~10^28 | ~3.1 × 10^13 IDs | ~5.8 × 10^14 IDs |
| CLID-1-TS/2-TS | ~10^35 (random part) | ~3.7 × 10^17 per second | ~7.0 × 10^17/s |
| CLID-3-TS | ~10^26 (random part) | ~2.9 × 10^13 per second | ~5.5 × 10^13/s |
| CLID-4-TS | ~10^25 (random part) | ~1.6 × 10^12 per second | ~2.9 × 10^12/s |
| CLID-5-TS | ~10^17 (random part) | ~4.1 × 10^7 per second | ~7.6 × 10^8/s |

Collision probability is calculated using the birthday paradox approximation:
P ≈ n² / 2N, where n = IDs generated and N = total keyspace.

> **Note on TS variants:** The timestamp prefix is shared by all IDs generated within the same second,
> so collision resistance depends only on the random suffix. CLID-5-TS in particular is limited to
> ~41 million safe IDs per second — fine for most use cases, but worth knowing at high throughput.


## Usage Notes

- All formats are case sensitive
- No external dependencies or third-party recommendations
- Simple, self-contained specification
- Compatible with any programming language that supports Base62 encoding

## License

This specification is free to use and implement.
