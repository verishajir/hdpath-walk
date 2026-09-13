use crate::digest::{digest, hex32};

#[derive(Debug, Clone)]
pub struct Account {
    pub index: u32,
    pub label: String,
    pub address: String,
}

#[derive(Debug, Clone)]
pub struct Vault {
    pub id: String,
    pub name: String,
    pub coin: String,
    pub accounts: Vec<Account>,
}

pub fn seed_from(mnemonic: &str, pass: &str) -> [u8; 32] {
    let mut buf = Vec::from(mnemonic.as_bytes());
    buf.extend_from_slice(b"|");
    buf.extend_from_slice(pass.as_bytes());
    digest(&buf)
}

pub fn derive(seed: &[u8; 32], index: u32, path: &str) -> ([u8; 32], [u8; 32]) {
    let mut buf = seed.to_vec();
    buf.extend_from_slice(path.as_bytes());
    buf.extend_from_slice(&index.to_be_bytes());
    let privk = digest(&buf);
    let mut pub_src = privk.to_vec();
    pub_src.extend_from_slice(b"pub");
    (privk, digest(&pub_src))
}

pub fn address(prefix: &str, pubk: &[u8; 32]) -> String {
    let mut buf = pubk.to_vec();
    buf.extend_from_slice(prefix.as_bytes());
    format!("{}{}", prefix, &hex32(&digest(&buf))[..32])
}

pub fn create_vault(name: &str, pass: &str) -> Vault {
    let seed = seed_from(name, pass);
    let (_, pubk) = derive(&seed, 0, "m/44'/0'/0'");
    let id = hex32(&digest(name.as_bytes()))[..16].to_string();
    Vault {
        id,
        name: name.to_string(),
        coin: "MULTI".into(),
        accounts: vec![Account {
            index: 0,
            label: "Primary".into(),
            address: address("hd", &pubk),
        }],
    }
}

pub fn add_account(vault: &mut Vault, label: &str) {
    let seed = seed_from(&vault.name, "demo");
    let index = vault.accounts.len() as u32;
    let (_, pubk) = derive(&seed, index, "m/44'/0'/0'");
    vault.accounts.push(Account {
        index,
        label: label.into(),
        address: address("hd", &pubk),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_has_account() {
        let v = create_vault("test", "pw");
        assert_eq!(v.name, "test");
        assert_eq!(v.accounts.len(), 1);
        assert!(v.accounts[0].address.starts_with("hd"));
    }

    #[test]
    fn unique_addresses() {
        let mut v = create_vault("test", "pw");
        add_account(&mut v, "A");
        add_account(&mut v, "B");
        let set: std::collections::HashSet<_> =
            v.accounts.iter().map(|a| a.address.clone()).collect();
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn seed_pass_changes() {
        assert_ne!(seed_from("m", "a"), seed_from("m", "b"));
    }
}
