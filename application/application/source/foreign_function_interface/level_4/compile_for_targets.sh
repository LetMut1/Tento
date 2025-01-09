#!/bin/sh
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
result=`cargo build --manifest-path=./source/Cargo.toml --release --lib --target=x86_64-apple-ios`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi
result=`cp ./../../../target/aarch64-apple-ios/release/liblevel_4.a ./target_/aarch64-apple-ios/liblevel_4.a`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi
result=`cp ./../../../target/aarch64-apple-ios-sim/release/liblevel_4.a ./target_/aarch64-apple-ios-sim/liblevel_4.a`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi
result=`cp ./../../../target/x86_64-apple-ios/release/liblevel_4.a ./target_/x86_64-apple-ios/liblevel_4.a`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi
exit 0