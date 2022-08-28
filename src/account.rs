use aptos_keygen::KeyGen;
use aptos_types::{account_address::AccountAddress, transaction::authenticator::AuthenticationKey};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serde_json::to_string_pretty;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub address: String,
    public_key: String,
    private_key: String,
}

impl Account {
    pub fn generate() -> Self {
        let (private_key, public_key) = KeyGen::from_os_rng().generate_ed25519_keypair();
        let auth_key = AuthenticationKey::ed25519(&public_key).to_vec();
        Self {
            address: AccountAddress::from_bytes(&auth_key).unwrap().to_string(),
            public_key: hex::encode(&public_key.to_bytes()),
            private_key: hex::encode(&private_key.to_bytes()),
        }
    }

    pub fn write_key(&self, path: Option<String>) {
        let output = to_string_pretty(self).unwrap();
        let mut file_path = PathBuf::new();
        let path = path.unwrap_or(String::from("./"));
        file_path.push(path.clone());
        let mut file_name = self.address.clone();
        file_name.push_str(".json");
        file_path.push(file_name);
        let mut generated_file = File::create(file_path.clone())
            .expect(&format!("Unable to create file. {:?}", file_path.to_str()));
        generated_file
            .write_all(output.as_bytes())
            .expect("Unable to write data");
        println!("Wrote {} to {:?}", self.address, path);
    }
}
