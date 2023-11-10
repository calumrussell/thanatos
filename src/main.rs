use alloy_sol_types::{sol, SolCall};
use alloy_primitives::hex;

sol!(
   MissingParam,
   r#"[{
        "constant": false,
        "inputs": [
            {
                "name": "path",
                "type": "address[]"
            },
            {
                "name": "to",
                "type": "address"
            },
            {
                "name": "deadline",
                "type": "uint256"
            }
        ],
        "name": "swapTokensForExactTokens",
        "outputs": [{
            "type": "uint256[]",
            "name": "amounts"
        }],
        "type": "function",
        "stateMutability": "view"
    }]"#
);

sol!(
   RightContract,
   r#"[{
        "constant": false,
        "inputs": [
            {
                "name": "amountOut",
                "type": "uint256"
            },
            {
                "name": "amountInMax",
                "type": "uint256"
            },
            {
                "name": "path",
                "type": "address[]"
            },
            {
                "name": "to",
                "type": "address"
            },
            {
                "name": "deadline",
                "type": "uint256"
            }
        ],
        "name": "swapTokensForExactTokens",
        "outputs": [{
            "type": "uint256[]",
            "name": "amounts"
        }],
        "type": "function",
        "stateMutability": "view"
    }]"#
);

fn main() {
    let hex = hex!("8803dbee000000000000000000000000000000000000000000000001158e460913d000000000000000000000000000000000000000000000000000003a4837fc5242c4ec00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000009a7ed54b8c2c5816c1800476f5111a1e886575020000000000000000000000000000000000000000000000000000000060cfee3f0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000003c9d6c1c73b31c837832c72e04d3152f051fc1a9");
    let val = RightContract::swapTokensForExactTokensCall::abi_decode(&hex, false).unwrap();
    println!("{:?}", val.amountInMax);
    println!("{:?}", val.amountOut);
    println!("{:?}", val.deadline);
    println!("{:?}", val.to);

    //Should throw an error value because we are using the wrong spec
    let val = MissingParam::swapTokensForExactTokensCall::abi_decode(&hex, false).unwrap();
}
