/*
Copyright 2018 elipmoc

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated 
documentation files (the "Software"), to deal in the Software without restriction, including without limitation 
the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and 
to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of 
the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO 
THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS 
OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR 
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

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
