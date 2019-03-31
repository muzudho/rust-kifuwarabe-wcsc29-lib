# rust-kifuwarabe-wcsc29-lib
コンピューター将棋ソフトきふわらべ 第29回世界コンピューター将棋選手権に向けたバージョン☆（＾～＾）

## How to convert csa-record to rpm-record?

```Shell
### Example.
cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
cargo clippy --example csa_to_rpm
 
### Run.
### '--' is separator. You can pass arguments to exe.
### Please change encoding to UTF-8.
cargo run --example csa_to_rpm -- --path download-kifu/WCSC28_F6_PAL_HFW.csa
cargo run --example csa_to_rpm -- --path download-kifu/WCSC_F2_QHA_TNK.csa
```

Output.
`C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29/target/release/libkifuwarabe-wcsc29.rlib`

## How to convert usi-record to rpm-record?

```Shell
### Example.
cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
cargo clippy --example usi_to_rpm
 
### Run.
### '--' is separator. You can pass arguments to exe.
### Please change encoding to UTF-8.
cargo run --example usi_to_rpm -- --path download-kifu/test.usi
```

## How to use engine?

[rust-kifuwarabe-wcsc29.](https://github.com/muzudho/kifuwarabe-wcsc29)

## Test

```Shell
### Example.
cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
cargo clippy --example main
 
### Run.
cargo run --example main
```

## Learn

```
### Example.
cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
cargo clippy --example learn
 
### Run.
cargo run --example learn
```
