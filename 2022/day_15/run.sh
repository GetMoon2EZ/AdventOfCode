#!/bin/bash

set -e

LRED='\033[1;31m'   # Light Red
LGREEN='\033[1;32m' # Light Green
LCYAN='\033[1;36m'  # Light Cyan
NC='\033[0m' # No Color

dir_name=${PWD##*/}
# dir_name=${result:-/}

dir_name=$(echo "${dir_name}" | sed 's/_/ /g')

printf "${LGREEN}------------------ BUILDING ${dir_name^^} ------------------${NC}\n"
cargo build --release
printf "${LGREEN}--------------- DONE BUILDING ${dir_name^^} ----------------${NC}\n\n"

printf "${LGREEN}------------- RUNNING ${dir_name^^} TEST CASES -------------${NC}\n"

printf "\n${LRED}----------- TEST CASE: control_input.txt -----------${NC}\n"
printf "${LCYAN}# CHALLENGE 1${NC}\n"
./target/release/advent_of_code 1 control_input.txt 10
printf "\n${LCYAN}# CHALLENGE 2${NC}\n"
./target/release/advent_of_code 2 control_input.txt 20

printf "\n${LRED}--------------- TEST CASE: input.txt ---------------${NC}\n"
printf "${LCYAN}# CHALLENGE 1${NC}\n"
./target/release/advent_of_code 1 input.txt 2000000
printf "\n${LCYAN}# CHALLENGE 2${NC}\n"
./target/release/advent_of_code 2 input.txt 4000000

printf "\n${LGREEN}----------- DONE RUNNING ${dir_name^^} TEST CASES -----------${NC}\n"
