use crate::base::misc::brorand::Brorand;
use crate::base::data::constants::PASSWORD_LEN;
use crate::WalletType;
use crate::base::wallet::generate_str;
use crate::base::wallet::keypair::*;

static H_SECP256K1: &[u8] = &[33];
static H_ED25519: &[u8] = &[33];

pub struct Seed {}
impl Seed {
    pub fn build(wtype: &WalletType) -> String {
        //1. Generete PASSWORD_LEN random data
        let u: Vec<u8> = Brorand::brorand(PASSWORD_LEN);

        // let u: Vec<u8> = [30, 231, 104, 55, 135, 220, 64, 82, 229, 64, 178, 68, 30, 175, 96, 164].to_vec();
        // key_type: SECP256K1,
        // address: "jwBdNzVXZBzNBrzu3RLPUGqvJsXuTiEv6i",
        // secret: "ss3G85CSJVpUPTjAFPfntLxHHReiM",
        // keypair: Keypair {
        //     private_key: "00CB0838815F8E735262EEDC4C111D1940370F1C412F98A237BAD68C2FAC623662",
        //     public_key: "020769117B8AC2C88921143E834659C2D7492DF65610610A80F0EB5859D80A0A65"
        // }

        //2. dependen on type decide which curve to use secp256k1/de255119
        //3. encodeSeed function
        let mut version: Vec<u8>; //default secp256k1
        match wtype {
            &WalletType::ED25519 => {
                version = H_ED25519.to_vec();
            },

            &WalletType::SECP256K1 => {
                version = H_SECP256K1.to_vec();
            },
        }

        generate_str(&mut version, &u)
    }
}

impl Seed {
    pub fn check_secret(seed: &String) -> Option<bool> {
        let key_pair = KeypairBuilder::new(&seed, &WalletType::SECP256K1).build();
        if key_pair.is_ok() {
            return Some(true);
        }

        let key_pair = KeypairBuilder::new(&seed, &WalletType::ED25519).build();
        if key_pair.is_ok() {
            return Some(true);
        }

        None
    }
}
