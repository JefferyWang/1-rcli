use anyhow::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::Claims;

pub fn process_jwt_sign(key: &str, claims: &Claims) -> Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(key.as_ref()),
    )?;
    Ok(token)
}

pub fn process_jwt_verify(key: &str, token: &str) -> Result<bool> {
    let mut validation = Validation::default();
    validation.validate_aud = false;
    let token = decode::<Claims>(token, &DecodingKey::from_secret(key.as_ref()), &validation);
    Ok(token.is_ok())
}

#[cfg(test)]
mod tests {
    use crate::parse_exp;

    #[test]
    fn test_jwt_sign_verify() -> anyhow::Result<()> {
        let key = "your-256-bit-secret";
        let claims = super::Claims {
            sub: "test".to_string(),
            aud: "test".to_string(),
            exp: parse_exp("1d")?,
        };
        let token = super::process_jwt_sign(key, &claims)?;
        assert!(super::process_jwt_verify(key, &token)?);
        Ok(())
    }
}
