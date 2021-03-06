tonic::include_proto!("crosslink");

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn construct_proto() {
        let decimal = DecimalValue {
            value: 1,
            decimal: 2,
        };
        let mut encoded = Vec::new();
        encoded.reserve(decimal.encoded_len());
        decimal.encode(&mut encoded).unwrap();
        let decoded = DecimalValue::decode(std::io::Cursor::new(encoded)).unwrap();

        assert_eq!(decoded.value, 1);
        assert_eq!(decoded.decimal, 2);
    }
}
