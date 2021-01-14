#! /bin/bash

pwd=`pwd`
dir=$pwd/target/pachou
cargo build --release

strip target/release/pachou.exe
rm pachou.zip

rm -rf $dir
mkdir -p $dir/config
cp $pwd/config/log4rs.yaml $dir/config
cp $pwd/config/output.json $dir/config
cp $pwd/target/release/pachou.exe $dir
cp $pwd/run.bat $dir

cd $dir

zip -q -r ../../pachou.zip * 
