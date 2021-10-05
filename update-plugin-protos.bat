cargo build -p puroro-plugin && ^
rmdir .\puroro-protobuf-compiled\src /S /Q && ^
mkdir .\puroro-protobuf-compiled\src
call %PURORO_PROTOC_PATH% ^
    protobuf/src/google/protobuf/compiler/plugin.proto ^
    --plugin=protoc-gen-rust=./target/debug/puroro-plugin.exe ^
    --rust_out=./puroro-protobuf-compiled/src/ ^
    --proto_path=./protobuf/src/