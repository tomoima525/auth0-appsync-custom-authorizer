use jsonwebtoken::{decode, decode_header, jwk, DecodingKey, Validation};
use serde_json::from_str;
use std::collections::HashMap;
use std::env;
use std::error::Error;

pub fn validate_token(jwt_data: &str, token: &str) -> Result<(), Box<dyn Error>> {
    let jwks: jwk::JwkSet = from_str(jwt_data).unwrap();
    let header = decode_header(token).unwrap();
    let kid = match header.kid {
        Some(k) => k,
        None => return Err("Token doesn't have kid".into()),
    };
    if let Some(key) = jwks.find(&kid) {
        match key.algorithm {
            jwk::AlgorithmParameters::RSA(ref rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();

                // verifying token
                let mut validation = Validation::new(key.common.algorithm.unwrap());
                validation.validate_exp = false;

                let audience: String = env::var("AUDIENCE").expect("No AUDIENCE env set");
                let token_issuer: String =
                    env::var("TOKEN_ISSUER").expect("No TOKEN_ISSUER env set");
                println!("Audience {:?}", audience);
                println!("Issuer {:?}", token_issuer);

                validation.set_audience(&[audience]);
                validation.set_issuer(&[token_issuer]);
                let decoded_token =
                    decode::<HashMap<String, serde_json::Value>>(token, &decoding_key, &validation)
                        .unwrap();
                println!("{:?}", decoded_token);
            }
            _ => return Err("Should be RSA".into()),
        }
    } else {
        return Err("No matching jwk found".into());
    }
    Ok(())
}
