extern crate libloading as lib;

/* DLL function binding macro!! */
macro_rules!  dx{
    ($fname:ident ($($argname:ident;$argt:ty),*) => $r:ty) => {
        pub fn $fname(&self, $($argname:$argt),* )->lib::Result<$r>{
            unsafe {
                let func: lib::Symbol<unsafe extern "stdcall" fn($($argt),*)->$r> = self.lib.get(stringify!($fname).as_bytes())?;
                Ok(func($($argname),*))
            }
        }
    };
}
macro_rules! dx_block {
    ($($fname:ident ($($argname:ident;$argt:ty),*) => $r:ty);*) => {
        $(dx!{ $fname($($argname;$argt),*)=>$r })*
    };
}

/*dxlib const variable*/
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;

/*dxlib function return type*/
pub type DxResult<T> = lib::Result<T>;

/*dxlib DLL object */
pub struct DxlibDLL {
    lib: lib::Library,
}

impl DxlibDLL {
    pub fn new() -> Self {
        DxlibDLL {
            lib: lib::Library::new("DxLib_x64.dll").unwrap(),
        }
    }
    dx_block!{
        dx_DxLib_Init()=>i32;
        dx_ChangeWindowMode(flag;i32)=>i32;
        dx_DxLib_End()=>i32;
        dx_SetMainWindowText(WindowText;*const u8)=>i32;
        dx_SetGraphMode(SizeX; i32,SizeY; i32,ColorBitNum; i32,RefreshRate; i32)=>i32;
        dx_WaitKey()=>i32
    }
}
