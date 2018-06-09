# rust_dxlib
DXライブラリをRustで使えるようにする。

使い方

Cargo.tomlに記述

```toml
[dependencies.rust_dxlib]
git = "https://github.com/elipmoc/rust_dxlib.git"
```

src/main.rs
```Rust
extern crate rust_dxlib;
use rust_dxlib::dxlib;

fn main(){
  dxlib::dx_DxLib_Init();
}
```

あとはコンパイルしてできた生成物と同じ階層にDxLib_x64.dllを配置すると動く。

DxLib_x64.dllはDxライブラリ公式サイトのダウンロードページからC#版のDXライブラリをダウンロードすると手に入る


## License
MIT
