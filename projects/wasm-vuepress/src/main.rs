use crypto_marysue::{decode, encode};

fn main() {
    let x = "苟利国家生死以, 岂因祸福避趋之?";
    println!("{:#?}", encode(&encode(&encode(&encode(&encode(&encode(&encode(&encode(x)))))))))
}
