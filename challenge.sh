#!/bin/zsh

# I am sooooo annoyed that this same script 
# amounts to `2015..2025 | ./target/computus`
# in Powershell. 

for i in {2015..2025}
do
   ./target/computus $i
done
