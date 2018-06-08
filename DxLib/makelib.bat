@echo off

rem ------------------------------------
rem infomation
rem batの説明を記載

rem ------------------------------------
rem setting
rem 各種設定
set CurDir=%~dp0
set VS_BIN=%VS140COMNTOOLS%..\..\VC\bin\
set DUMPBIN_EXE="%VS_BIN%dumpbin.exe"
set LIB_EXE="%VS_BIN%lib.exe"
set MAKEDEF_SH=%CurDir%makedef.sh

set InputDir=%1
set OutputDir=%2

call :CLEAR_SCREEN

echo %InputDir%
echo %OutputDir%

rem ------------------------------------
rem 実装部
:EXEC
for /f "usebackq" %%f in (`dir /b /s %InputDir%\*.dll`) do (
   echo MakeLib [%%f,%%~nf.lib]

   rem dumpファイル作成
   %DUMPBIN_EXE% /EXPORTS %%f > %%~nf.txt
   rem dumpファイルからdefファイルを作成
   bash %MAKEDEF_SH% %%~nf.txt
   rem defファイルからlibファイルを生成
   %LIB_EXE% /DEF:%%~nf.def /MACHINE:X64 /out:%OutputDir%\%%~nf.lib

   rem 不必要になったゴミの削除
   del %%~nf.txt
   del %%~nf.def
   del %OutputDir%\*.exp
)

rem ------------------------------------
rem 終了処理
:END


:: /B 呼び出し元へ戻る
exit /B

rem ------------------------------------
rem ログ消去
:CLEAR_SCREEN

cls
exit /B

rem ------------------------------------
rem EOF