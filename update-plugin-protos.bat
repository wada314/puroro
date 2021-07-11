cargo build -p puroro-plugin && ^
rmdir .\puroro-plugin\src\protos /S /Q && ^
mkdir .\puroro-plugin\src\protos
call %PURORO_PROTOC_PATH% ^
    protobuf/src/google/protobuf/compiler/plugin.proto ^
    --plugin=protoc-gen-rust=./target/debug/puroro-plugin.exe ^
    --rust_out=./puroro-plugin/src/protos/ ^
    --proto_path=./protobuf/src/