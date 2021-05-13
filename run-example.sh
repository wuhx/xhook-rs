TARGET=target/aarch64-linux-android/debug/examples/main

rm $TARGET
cargo +nightly build --target aarch64-linux-android  --example main

adb_run() {
 EXE=$1
 NAME=$(basename $1)
 TMP=/data/local/tmp/ADB_RUN
 echo run $EXE on $TMP
 adb shell rm -fr  $TMP/$NAME 2> /dev/null
 adb shell mkdir -p $TMP 1>/dev/null 2>/dev/null
 adb push $EXE $TMP
 adb shell "chmod +x $TMP/$NAME"
 adb shell "$TMP/$NAME"
}

adb_run $TARGET
