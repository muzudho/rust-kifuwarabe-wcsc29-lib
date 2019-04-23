# rust-kifuwarabe-wcsc29-lib
コンピューター将棋ソフトきふわらべ 第29回世界コンピューター将棋選手権に向けたバージョン☆（＾～＾）

## 環境設定。

棋譜を読み込むために、以下の１０個のディレクトリーを作れ。これらのディレクトリーは設定ファイルに記入しろ。
RPM形式棋譜というのは きふわらべ に読める棋譜の形式(Reversible physical move)だぜ。

| 呼び名           | パスの例                             | 働き                                                             |
| ---------------- | ------------------------------------ | ---------------------------------------------------------------- |
| expansion.go     | C:/shogi-record/go/hunting           | ここに置いてあるものはテキストファイルになるまで解凍する。       |
| expansion.went   | C:/shogi-record/went/hunted          | 解凍したらここへ退避。                                           |
| expansion.output | C:/shogi-record/go/cooking           | 解凍した中身はここへ出力。                                       |
| formation.go     | ※expansion.outputと同じ              | ここに置いてあるテキストファイルは UTF8 エンコーディングにする。 |
| formation.went   | C:/shogi-record/went/cooked          | あとはパターンを感じろ。                                         |
| formation.output | C:/shogi-record/go/eating            |                                                                  |
| eating.go        | ※formation.outputと同じ              | ここに置いてあるものは RPM形式棋譜に変換する。                   |
| eating.went      | C:/shogi-record/went/ate             | 感じろ。                                                         |
| eating.output    | C:/muzudho/shogi-record/rpm          | ここに成果物を置く。実践では使わない。                           |
| learning         | C:/muzudho/shogi-record/learning/rpm | 実践で対局した棋譜を置く。実践では使わない。                     |
| rpm_record       | C:/muzudho/shogi-record/rpm-json     | 実践で使う。使いたいRPM棋譜をここへ移動しろだぜ。                |

棋譜の変換（翻訳）は Rust言語 の examples に入っている。
これはライブラリなので実行できないので、 kifuwarabe-wcsc29.exe の方を呼び出せだぜ。
解凍や エンコーディング、全自動翻訳は `CsaOpener` といった感じの名前の適当に作った C#言語 のプログラムから Rust言語を叩いてやる。

## How to eat .kif record?

```Shell
### Example.
cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
cls

### Compile.
set RUST_BACKTRACE=1
cargo clippy --example eat_a_kif
 
### Run.
### '--' is separator. You can pass arguments to exe.
### Please change encoding to UTF-8.
cargo run --example eat_a_kif -- --path C:/muzudho/kifuwarabe-wcsc29-learn/output-wcsc-record/copied-daiwa/daiwa.kif
```

## How to convert .kif record?

```Shell
### Example.
cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
cls

### Compile.
set RUST_BACKTRACE=1
cargo clippy --example conv_a_kif

### Run.
### '--' is separator. You can pass arguments to exe.
### Please change encoding to UTF-8.
cargo run --example conv_a_kif -- --input "C:/shogi-record/go/eating/wcsc/永世名人/01eis-kak.kif" --output "C:/muzudho/shogi-record/rpm/wcsc/永世名人/01eis-kak.rpmove"
cargo run --example conv_a_rec -- --input "C:\shogi-record\go\eating\wcsc\１回戦\kifu.csa" --output "C:/muzudho/shogi-record/rpm/wcsc/１回戦\kifu.rpmove"
```

## How to eat .csa record?

```Shell
### Example.
cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
set RUST_BACKTRACE=1
cargo clippy --example eat_a_csa
 
### Run.
### '--' is separator. You can pass arguments to exe.
### Please change encoding to UTF-8.
cargo run --example eat_a_csa -- --path C:/muzudho/kifuwarabe-wcsc29-learn/output-wcsc-record/extracted-wcsc28_kifu/wcsc28_kifu/WCSC_F1_APR_MCB.csa
cargo run --example eat_a_csa -- --path C:/muzudho/kifuwarabe-wcsc29-learn/csa-record/WCSC_F2_QHA_TNK.csa
```

Output.
`C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29/target/release/libkifuwarabe-wcsc29.rlib`

## How to convert usi-record to rpm-record?

```Shell
### Example.
cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
set RUST_BACKTRACE=1
cargo clippy --example usi_to_rpm
 
### Run.
### '--' is separator. You can pass arguments to exe.
### Please change encoding to UTF-8.
cargo run --example usi_to_rpm -- --path C:/muzudho/kifuwarabe-wcsc29-learn/usi-record/test.usi
```

## How to use engine?

[rust-kifuwarabe-wcsc29.](https://github.com/muzudho/kifuwarabe-wcsc29)

## Test

```Shell
### Example.
cd C:\muzudho\projects_rust\rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
set RUST_BACKTRACE=1
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
