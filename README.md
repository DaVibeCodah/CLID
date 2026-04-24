# CLID - Clean ID Formats

A collection of clean, human-readable identifier formats without unnecessary complexity.

## Format Specifications

### CLID-1
<4 char>-<4 char>-<4 char>-<4 char>-<4 char>-<4 char>

- Total length: 24 characters (excluding hyphens)
- Hyphens included for readability
- Character set: A-Z, a-z, 0-9 (case sensitive)
- Example: AbC1-dEfG-89Hi-JkL0-mNoP-QrS2

### CLID-1-TS
<timestamp>-<4 char>-<4 char>-<4 char>-<4 char>-<4 char>

- Timestamp (Base62 encoded) + 20 random characters
- Total length: timestamp length varies + 20 chars (excluding hyphens)
- Example: aB3xY-7kLm-NpQ9-RsTu-VwX0-YzA1

### CLID-2
<4 char><4 char><4 char><4 char><4 char><4 char>

- Total length: 24 characters (no separators)
- Character set: A-Z, a-z, 0-9 (case sensitive)
- Example: AbC1dEfG89HiJkL0mNoPQrS2

### CLID-2-TS
<timestamp><4 char><4 char><4 char><4 char><4 char>

- Timestamp (Base62 encoded) + 20 random characters
- Total length: timestamp length varies + 20 chars (no separators)
- Example: aB3xY7kLmNpQ9RsTuVwX0YzA1

### CLID-3
21 characters using Base62 encoding (A-Z, a-z, 0-9)

- Example: aB3xY7kLmNpQ9RsTuVwX0

### CLID-3-TS
<timestamp> + 15 random characters (Base62)

- Total length: timestamp length varies + 15 chars
- Example: xY9kLmNpQ9RsTuVwX0

### CLID-4
20 characters using Base62 encoding

- Example: aB3xY7kLmNpQ9RsTuVw

### CLID-4-TS
<timestamp> + 14 random characters (Base62)

- Total length: timestamp length varies + 14 chars
- Example: xY9kLmNpQ9RsTuVw

### CLID-5
16 characters using Base62 encoding

- Example: aB3xY7kLmNpQ9RsT

### CLID-5-TS
<timestamp> + 10 random characters (Base62)

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

## Usage Notes

- All formats are case sensitive
- No external dependencies or third-party recommendations
- Simple, self-contained specification
- Compatible with any programming language that supports Base62 encoding

## License

This specification is free to use and implement.
