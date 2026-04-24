#!/usr/bin/env node

const BASE62 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

function base62Encode(num) {
    if (num === 0) return BASE62[0];
    let result = [];
    while (num > 0) {
        result.push(BASE62[num % 62]);
        num = Math.floor(num / 62);
    }
    return result.reverse().join('');
}

function randomString(length) {
    let result = '';
    for (let i = 0; i < length; i++) {
        result += BASE62[Math.floor(Math.random() * BASE62.length)];
    }
    return result;
}

function timestampPart() {
    return base62Encode(Math.floor(Date.now() / 1000));
}

function generateCLID(formatType, withTimestamp = false) {
    switch(formatType) {
        case "CLID-1":
            let parts = Array(6).fill().map(() => randomString(4));
            if (withTimestamp) parts[0] = timestampPart();
            return parts.join('-');
        
        case "CLID-2":
            if (withTimestamp) {
                return timestampPart() + Array(5).fill().map(() => randomString(4)).join('');
            }
            return Array(6).fill().map(() => randomString(4)).join('');
        
        case "CLID-3":
            if (withTimestamp) return timestampPart() + randomString(15);
            return randomString(21);
        
        case "CLID-4":
            if (withTimestamp) return timestampPart() + randomString(14);
            return randomString(20);
        
        case "CLID-5":
            if (withTimestamp) return timestampPart() + randomString(10);
            return randomString(16);
        
        default:
            throw new Error(`Unknown format: ${formatType}`);
    }
}

console.log("=== CLID Generator ===");
console.log(`CLID-1:      ${generateCLID("CLID-1")}`);
console.log(`CLID-1-TS:   ${generateCLID("CLID-1", true)}`);
console.log(`CLID-2:      ${generateCLID("CLID-2")}`);
console.log(`CLID-2-TS:   ${generateCLID("CLID-2", true)}`);
console.log(`CLID-3:      ${generateCLID("CLID-3")}`);
console.log(`CLID-3-TS:   ${generateCLID("CLID-3", true)}`);
console.log(`CLID-4:      ${generateCLID("CLID-4")}`);
console.log(`CLID-4-TS:   ${generateCLID("CLID-4", true)}`);
console.log(`CLID-5:      ${generateCLID("CLID-5")}`);
console.log(`CLID-5-TS:   ${generateCLID("CLID-5", true)}`);
