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
result=`cp ./../../../target/aarch64-apple-ios/release/liblevel_3.a ./target_/aarch64-apple-ios/liblevel_3.a`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi
result=`cp ./../../../target/aarch64-apple-ios-sim/release/liblevel_3.a ./target_/aarch64-apple-ios-sim/liblevel_3.a`
if [ $? != 0 ];
then
    echo "An error was occured: \n $result ."
    exit 1
fi
exit 0