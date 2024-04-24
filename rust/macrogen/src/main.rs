use std::{concat, env, stringify};
macro_rules! gencode {
    ($($c:ident),*) => {

             $(
                 abigen!($c, env!(OUT_DIR)) ;
             )*

    }
}

macro_rules! abigen {
    ($contract_name: ident, $abi_path: literal) => {
        const $contract_name: &str = $abi_path;
    };
}

macro_rules! contract_list {
    ($macro_name: ident) => {
        $macro_name!(NordSConctrac1);
        $macro_name!(NordSCContract2);
        $macro_name!(NordSCContract3);
    };
}

const path: &str = stringify!(std::env!(OUTDIR));

contract_list!(gencode);
fn main() {
    //let s = vec_strs![1, "a", true, 3.14159f32];
    //let v = vec_strs!(["faraz", "sana", "rahmah", "hana"]);

    //gencode!("faraz", "sana", "rahmah", "hana");
}
