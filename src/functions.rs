use std::ffi::CString;
use std::os::raw::c_char;

use const_variables::*;
use types::*;

//極大闇魔黒呪文
macro_rules!  dx_func{
    ($fname:ident($($argname:ident:$argt:ty),*) -> $r:ty;wrap ($($w_argname:ident:$w_argt:ty),*)->$w_r:ty {$($st:stmt);*}$($tt:tt)*) => {
        mod $fname{
            use std::os::raw::c_char;
            use const_variables::*;
            use types::*;
            #[link(name = "DxLib_x64")]
            #[no_mangle]
            extern "stdcall" {
                pub fn $fname($($argname:$argt),* )->$r;
            }
        }
        pub fn $fname($($w_argname:$w_argt),* )->$w_r {
            use self::$fname::$fname as this;
            $($st)*
        }
        dx_func!{$($tt)*}
    };
    ($fname:ident($($argname:ident:$argt:ty),*) -> $r:ty; $($tt:tt)*) => {
        #[link(name = "DxLib_x64")]
        #[no_mangle]
        extern "stdcall" {
            pub fn $fname($($argname:$argt),* )->$r;
        }
        dx_func!($($tt)*);        
    };
    ()=>{};
}

/*dxlib function extern declaration*/

dx_func!{
    //使用必須関数
    dx_DxLib_Init()->i32;
    dx_DxLib_End() -> i32;
    dx_ProcessMessage() -> i32;
    //図形描画関数
    dx_DrawLine(x1:i32 ,y1:i32 ,x2:i32 ,y2:i32 , Color:u32 ,Thickness:i32)->i32 ;
    dx_DrawLineAA(x1:f32, y1:f32, x2:f32, y2:f32, Color:u32, Thickness:f32)->i32;
    dx_DrawBox(x1:i32, y1:i32, x2:i32, y2:i32, Color:u32, FillFlag:i32)->i32;
    dx_DrawBoxAA(x1:f32,y1:f32,x2:f32, y2:f32, Color:u32, FillFlag:i32, LineThickness:f32)->i32;
    dx_DrawCircle(x:i32,y:i32,r:i32,Color:u32,FillFlag:i32, LineThickness:i32)->i32;
    dx_DrawCircleAA(x:f32, y:f32, r:f32,posnum:i32,Color:u32, FillFlag:i32,LineThickness:f32)->i32;
    dx_DrawOval( x:i32, y:i32, rx:i32, ry:i32, Color:u32, FillFlag:i32, LineThickness:i32)->i32;
    dx_DrawOvalAA( x:f32, y:f32, rx:f32, ry:f32, posnum:i32, Color:u32, FillFlag:i32, LineThickness:f32)->i32;
    dx_DrawTriangle( x1:i32, y1:i32, x2:i32,y2:i32, x3:i32, y3:i32, Color:u32, FillFlag:i32)->i32;
    dx_DrawTriangleAA(x1:f32, y1:f32, x2:f32, y2:f32, x3:f32,y3:f32, Color:u32, FillFlag:i32, LineThickness:f32)->i32;
    dx_DrawPixel( x:i32, y:i32, Color:u32)->i32;
    dx_GetPixel( x:i32, y:i32)->u32;


    dx_LoadGraph(FileName: *const c_char, NotUse3DFlag: i32) -> i32;
    wrap(FileName:&str)->i32 {
        unsafe { this(CString::new(FileName).unwrap().as_ptr(), FALSE) }
    }
    dx_DrawGraph(x: i32, y: i32, GrHandle: i32, TransFlag: i32) -> i32;
    //その他画面操作系関数
    dx_SetGraphMode(SizeX: i32, SizeY: i32, ColorBitNum: i32, RefreshRate: i32) -> i32;
    dx_ClearDrawScreen(ClearRect: *mut RECT)->i32;
    wrap()->i32 {
         let mut tmp = RECT {
            left: -1,
            right: -1,
            top: -1,
            bottom: -1,
        };
        unsafe { this(&mut tmp) }
    }
    dx_GetColor(Red:i32,Green:i32 ,Blue:i32)->u32;
    dx_SetDrawScreen(DrawScreen: i32) -> i32;
    dx_ScreenFlip() -> i32;


    //ウエイト関係の関数
    dx_WaitKey() -> i32;
    //ウインドウモード関係
    dx_ChangeWindowMode(flag: i32) -> i32;
    dx_SetMainWindowText(WindowText: *const u8) -> i32;
}
