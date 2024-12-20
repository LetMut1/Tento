#!/bin/sh

result=`cargo valgrind test --manifest-path=./source/Cargo.toml`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi

result=`cargo build --manifest-path=./source/Cargo.toml --release --lib --target=aarch64-apple-ios`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi

result=`cargo build --manifest-path=./source/Cargo.toml --release --lib --target=aarch64-apple-ios-sim`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi

result=`cargo build --manifest-path=./source/Cargo.toml --release --lib --target=aarch64-linux-android`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi

result=`cargo build --manifest-path=./source/Cargo.toml --release --lib --target=x86_64-linux-android`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi

result=`cp ./../../../source/target/aarch64-apple-ios/release/libclient_server_data_serializer.a ./target_/aarch64-apple-ios/libclient_server_data_serializer.a`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi

result=`cp ./../../../source/target/aarch64-apple-ios-sim/release/libclient_server_data_serializer.a ./target_/aarch64-apple-ios-sim/libclient_server_data_serializer.a`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi

result=`cp ./../../../source/target/aarch64-linux-android/release/libclient_server_data_serializer.a ./target_/aarch64-linux-android/libclient_server_data_serializer.a`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi

result=`cp ./../../../source/target/x86_64-linux-android/release/libclient_server_data_serializer.a ./target_/x86_64-linux-android/libclient_server_data_serializer.a`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi

exit 0