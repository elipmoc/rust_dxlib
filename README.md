# rust_dxlib
DXライブラリをRustで使えるようにする。

使い方



```Rust
extern crate dxlib_rust;
use dxlib_rust::dxlib;

fn main(){
  dxlib::dx_DxLib_Init();
}
```

あとはコンパイルしてできた生成物と同じ階層にDxLib_x64.dllを配置すると動く。

DxLib_x64.dllはDxライブラリ公式サイトのダウンロードページからC#版のDXライブラリをダウンロードすると手に入る


## License
MIT
