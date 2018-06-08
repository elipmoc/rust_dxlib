#! /bin/sh

# -----------------------------------------------------------------------------
# infomation
# 渡されたファイルを元にdefファイルを作成します
#   グローバル変数    :全部大文字
#   ローカル変数      :LCC (hoge,foo,hogeRoot,fooDir)
#   終端/             :Root
#   終端              :Dir
#   関数              :先頭f (fPoo)

# -----------------------------------------------------------------------------
# setting

CurDir=$(cd $(dirname ${BASH_SOURCE:-$0}); pwd)
TempDir=$(mktemp -d)

# -----------------------------------------------------------------------------
# [function] 画面消去
function fClearScreen() {
    # cygwin
    cmd /c cls

    # linux
    # clear
}

# -----------------------------------------------------------------------------
# exec
#set -eux
OutputFile=${1%.*}.def
echo EXPORTS > $OutputFile

isExport=0
while read line ; do
    # 比較しやすいようにスペースを取り除いたものを用意する
    t=`echo $line | sed -e "s/\s//g"`

    if [ $isExport -eq 1 ] ; then
        if [ "$line" = "Summary" ]; then
            isExport=0
        else
            val=`echo $line | sed -e "s/.*\s\(.*\)$/\1/g"`
            if [ -n "$val" ]; then
                echo " $val" >> $OutputFile
            fi
        fi
    fi
    if [ "$t" = "ordinalhintRVAname" ] ; then
        isExport=1
    fi
done < $1

#set +eux
# -----------------------------------------------------------------------------
# trap
trap "rm -fd $TempDir" 0

# -----------------------------------------------------------------------------
# end

echo "Finished"

# EOF