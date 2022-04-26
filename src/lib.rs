pub mod crosslink {
    tonic::include_proto!("crosslink");
}

#[cfg(test)]
mod tests {
    use super::crosslink;
    use prost::Message;

    #[test]
    fn construct_proto() {
        let decimal = crosslink::DecimalValue {
            value: 1,
            decimal: 2,
        };
        let mut encoded = Vec::new();
        encoded.reserve(decimal.encoded_len());
        decimal.encode(&mut encoded).unwrap();
        let decoded = crosslink::DecimalValue::decode(std::io::Cursor::new(encoded)).unwrap();

        assert_eq!(decoded.value, 1);
        assert_eq!(decoded.decimal, 2);
    }
}
