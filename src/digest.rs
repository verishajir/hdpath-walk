//! Tiny deterministic 32-byte digest. Not a real hash function.

pub fn digest(data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut state: u64 = 0x9e3779b97f4a7c15;
    for (i, b) in data.iter().enumerate() {
        state = state
            .wrapping_mul(0x100000001b3)
            .wrapping_add(*b as u64)
            .wrapping_add(i as u64);
        out[i % 32] ^= (state >> ((i % 7) * 8)) as u8;
        out[(i + 13) % 32] = out[(i + 13) % 32].wrapping_add(*b);
    }
    out
}

pub fn hex32(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable() {
        assert_eq!(digest(b"abc"), digest(b"abc"));
    }

    #[test]
    fn changes() {
        assert_ne!(digest(b"a"), digest(b"b"));
    }
}
