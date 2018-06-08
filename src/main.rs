extern crate dxlib_rust;
use dxlib_rust::dxlib;

fn main() {
    game();
}

fn game() {
    unsafe {
        // DxLibはデフォルトでフルスクリーンなのでウィンドウモードに変更
        dxlib::dx_ChangeWindowMode(dxlib::TRUE);

        // ウィンドウサイズを320x240に変更
        // （32bit, 60fpsを指定していますがウィンドウモードでは関係ないはず）
        dxlib::dx_SetGraphMode(320, 240, 32, 60);

        // DxLibの初期化
        dxlib::dx_DxLib_Init();

        dxlib::dx_ClearDrawScreen();
        // キー入力かマウス入力を待つ
        dxlib::dx_WaitKey();

        // キー入力かマウス入力を待つ
        dxlib::dx_WaitKey();

        // DxLibの後処理
        dxlib::dx_DxLib_End();
    }
}
