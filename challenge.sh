#!/bin/zsh
for i in `seq 2015 2025`;
do
   ./target/computus $i
done
