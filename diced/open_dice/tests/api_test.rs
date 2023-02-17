/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use diced_open_dice::{
    derive_cdi_certificate_id, hash, kdf, HASH_SIZE, ID_SIZE, PRIVATE_KEY_SEED_SIZE,
};

#[test]
fn hash_succeeds() {
    const EXPECTED_HASH: [u8; HASH_SIZE] = [
        0x30, 0x9e, 0xcc, 0x48, 0x9c, 0x12, 0xd6, 0xeb, 0x4c, 0xc4, 0x0f, 0x50, 0xc9, 0x02, 0xf2,
        0xb4, 0xd0, 0xed, 0x77, 0xee, 0x51, 0x1a, 0x7c, 0x7a, 0x9b, 0xcd, 0x3c, 0xa8, 0x6d, 0x4c,
        0xd8, 0x6f, 0x98, 0x9d, 0xd3, 0x5b, 0xc5, 0xff, 0x49, 0x96, 0x70, 0xda, 0x34, 0x25, 0x5b,
        0x45, 0xb0, 0xcf, 0xd8, 0x30, 0xe8, 0x1f, 0x60, 0x5d, 0xcf, 0x7d, 0xc5, 0x54, 0x2e, 0x93,
        0xae, 0x9c, 0xd7, 0x6f,
    ];
    assert_eq!(EXPECTED_HASH, hash(b"hello world").expect("hash failed"));
}

#[test]
fn kdf_succeeds() {
    let mut derived_key = [0u8; PRIVATE_KEY_SEED_SIZE];
    kdf(b"myInitialKeyMaterial", b"mySalt", b"myInfo", &mut derived_key).unwrap();
    const EXPECTED_DERIVED_KEY: [u8; PRIVATE_KEY_SEED_SIZE] = [
        0x91, 0x9b, 0x8d, 0x29, 0xc4, 0x1b, 0x93, 0xd7, 0xeb, 0x09, 0xfa, 0xd7, 0xc9, 0x87, 0xb0,
        0xd1, 0xcc, 0x26, 0xef, 0x07, 0x83, 0x42, 0xcf, 0xa3, 0x45, 0x0a, 0x57, 0xe9, 0x19, 0x86,
        0xef, 0x48,
    ];
    assert_eq!(EXPECTED_DERIVED_KEY, derived_key);
}

#[test]
fn derive_cdi_certificate_id_succeeds() {
    const EXPECTED_ID: [u8; ID_SIZE] = [
        0x7a, 0x36, 0x45, 0x2c, 0x02, 0xf6, 0x2b, 0xec, 0xf9, 0x80, 0x06, 0x75, 0x87, 0xa5, 0xc1,
        0x44, 0x0c, 0xd3, 0xc0, 0x6d,
    ];
    assert_eq!(EXPECTED_ID, derive_cdi_certificate_id(b"MyPubKey").unwrap());
}