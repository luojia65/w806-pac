svd2rust -i w806.svd --target riscv --const_generic --keep_list --nightly
Remove-Item -Path "src" -Recurse
form -i lib.rs -o src/ 
Remove-Item lib.rs
cargo fmt
