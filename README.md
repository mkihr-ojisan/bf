# Brainf*ckインタプリタを作って高速化を目指すプロジェクト

## 使い方
### ビルド
```console
$ cargo build --release
```

### 実行
```console
$ target/release/bf -O2 --native-compile <bf source file>
```

## どういった最適化をしているの？
### 連続した`+-><`をまとめる
例えば`+++++`は中間表現で`Add(5)`に変換される。
### 値のコピーや乗算のためのループを展開(?)する
最適化する条件は、ループ内にループとI/O命令がないこと。
さらに、ループの始めと終わりでポインタが同じ位置にあり、その位置の値が1変化すること。

#### 例
- `[-]` → `SetZero`
- `[->+<]` → `IfNotZero { PointerIncrement, AddValueAt(-1), PointerDecrement, SetZero }`
- `[->++<]` → `IfNotZero { PointerIncrement, AddValueMultipliedBy(2, -1), PointerDecrement, SetZero }`
- `[+>+<]` → `IfNotZero { Negate, PointerIncrement, AddValueAt(-1), PointerDecrement, SetZero }`

ほとんど速くならない。


## ネイティブコードの生成
`--native-compile`オプションをつけると、オレオレアセンブラを使ってx86_64の機械語を生成する。LLVMは甘え

add命令などのオペランドに直接メモリを指定しているのでたぶん遅い。
レジスタを使っていい感じにしたい。