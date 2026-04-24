#!/usr/bin/env python3
import random
import time
import secrets

BASE62 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"

def base62_encode(num):
    if num == 0:
        return BASE62[0]
    result = []
    while num > 0:
        result.append(BASE62[num % 62])
        num //= 62
    return ''.join(reversed(result))

def random_string(length):
    return ''.join(secrets.choice(BASE62) for _ in range(length))

def timestamp_part():
    return base62_encode(int(time.time()))

def generate_clid(format_type, with_timestamp=False):
    if format_type == "CLID-1":
        parts = [random_string(4) for _ in range(6)]
        if with_timestamp:
            parts[0] = timestamp_part()
        return '-'.join(parts)
    
    elif format_type == "CLID-2":
        if with_timestamp:
            return timestamp_part() + ''.join([random_string(4) for _ in range(5)])
        return ''.join([random_string(4) for _ in range(6)])
    
    elif format_type == "CLID-3":
        if with_timestamp:
            return timestamp_part() + random_string(15)
        return random_string(21)
    
    elif format_type == "CLID-4":
        if with_timestamp:
            return timestamp_part() + random_string(14)
        return random_string(20)
    
    elif format_type == "CLID-5":
        if with_timestamp:
            return timestamp_part() + random_string(10)
        return random_string(16)
    
    else:
        raise ValueError(f"Unknown format: {format_type}")

if __name__ == "__main__":
    print("=== CLID Generator ===")
    print(f"CLID-1:      {generate_clid('CLID-1')}")
    print(f"CLID-1-TS:   {generate_clid('CLID-1', with_timestamp=True)}")
    print(f"CLID-2:      {generate_clid('CLID-2')}")
    print(f"CLID-2-TS:   {generate_clid('CLID-2', with_timestamp=True)}")
    print(f"CLID-3:      {generate_clid('CLID-3')}")
    print(f"CLID-3-TS:   {generate_clid('CLID-3', with_timestamp=True)}")
    print(f"CLID-4:      {generate_clid('CLID-4')}")
    print(f"CLID-4-TS:   {generate_clid('CLID-4', with_timestamp=True)}")
    print(f"CLID-5:      {generate_clid('CLID-5')}")
    print(f"CLID-5-TS:   {generate_clid('CLID-5', with_timestamp=True)}")
