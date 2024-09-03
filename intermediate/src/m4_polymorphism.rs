use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address String"),
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_address().unwrap();
    // do something
    converted_address
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_poly() {
        let address = Address::from_str("0x8eaD0E8FEc8319Fd62C8508c71a90296EfD4F042").unwrap();
        let new_address = get_ethereum_data(address);
        assert_eq!(address, new_address);
    }
}
