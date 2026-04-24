package main

import (
    "fmt"
    "math/rand"
    "time"
)

const base62 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"

func base62Encode(num int64) string {
    if num == 0 {
        return string(base62[0])
    }
    result := []byte{}
    for num > 0 {
        result = append([]byte{base62[num%62]}, result...)
        num /= 62
    }
    return string(result)
}

func randomString(length int) string {
    b := make([]byte, length)
    for i := range b {
        b[i] = base62[rand.Intn(62)]
    }
    return string(b)
}

func timestampPart() string {
    return base62Encode(time.Now().Unix())
}

func generateCLID(formatType string, withTimestamp bool) string {
    switch formatType {
    case "CLID-1":
        parts := make([]string, 6)
        for i := 0; i < 6; i++ {
            parts[i] = randomString(4)
        }
        if withTimestamp {
            parts[0] = timestampPart()
        }
        result := ""
        for i, p := range parts {
            if i > 0 {
                result += "-"
            }
            result += p
        }
        return result

    case "CLID-2":
        if withTimestamp {
            return timestampPart() + randomString(4) + randomString(4) + randomString(4) + randomString(4) + randomString(4)
        }
        return randomString(4) + randomString(4) + randomString(4) + randomString(4) + randomString(4) + randomString(4)

    case "CLID-3":
        if withTimestamp {
            return timestampPart() + randomString(15)
        }
        return randomString(21)

    case "CLID-4":
        if withTimestamp {
            return timestampPart() + randomString(14)
        }
        return randomString(20)

    case "CLID-5":
        if withTimestamp {
            return timestampPart() + randomString(10)
        }
        return randomString(16)

    default:
        return "Unknown format"
    }
}

func main() {
    rand.Seed(time.Now().UnixNano())
    
    fmt.Println("=== CLID Generator ===")
    fmt.Printf("CLID-1:      %s\n", generateCLID("CLID-1", false))
    fmt.Printf("CLID-1-TS:   %s\n", generateCLID("CLID-1", true))
    fmt.Printf("CLID-2:      %s\n", generateCLID("CLID-2", false))
    fmt.Printf("CLID-2-TS:   %s\n", generateCLID("CLID-2", true))
    fmt.Printf("CLID-3:      %s\n", generateCLID("CLID-3", false))
    fmt.Printf("CLID-3-TS:   %s\n", generateCLID("CLID-3", true))
    fmt.Printf("CLID-4:      %s\n", generateCLID("CLID-4", false))
    fmt.Printf("CLID-4-TS:   %s\n", generateCLID("CLID-4", true))
    fmt.Printf("CLID-5:      %s\n", generateCLID("CLID-5", false))
    fmt.Printf("CLID-5-TS:   %s\n", generateCLID("CLID-5", true))
}
