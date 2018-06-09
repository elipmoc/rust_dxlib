extern crate rust_dxlib;
use rust_dxlib::*;

fn main() {
    game();
}

fn game() {
    unsafe {
        //ウィンドウモード変更と初期化と裏画面設定
        dx_ChangeWindowMode(TRUE);
        dx_DxLib_Init();
        dx_SetDrawScreen(DX_SCREEN_BACK);

        let mut x = 0;
        // 画像格納用ハンドル
        let mut Handle;
        // 画像のロード
        Handle = dx_LoadGraph("sample_resource/hoge.png");

        // while( 裏画面を表画面に反映, メッセージ処理, 画面クリア )
        while (dx_ScreenFlip() == 0 && dx_ProcessMessage() == 0 && dx_ClearDrawScreen() == 0) {
            dx_DrawGraph(x, 100, Handle, TRUE); //画像の描画
            dx_DrawGraph(x / 2, 200, Handle, TRUE); //画像の描画
            dx_DrawGraph(x / 4, 300, Handle, TRUE); //画像の描画
            x = x + 2; // xを2増やす
        }

        dx_DxLib_End(); // DXライブラリ終了処理
    }
}
