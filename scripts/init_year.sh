#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <directory> <n>"
    echo "ex.: $0 ./src/solutions/ 2023"
    exit 1
fi

directory="$1"
year="$2"
year_dir="${directory}/y${year}"
days=24

read -p "Create files in ${year_dir}? (y/n): " choice
if [ "$choice" == "n" ]; then
    echo "Exiting script. No files created."
    exit 1
fi

if [ ! -d "${year_dir}" ]; then
    mkdir -p "${year_dir}"
fi

# Create mod.rs
mod_file="${year_dir}/mod.rs"
> "$mod_file"

# Generate mod lines
for day in $(seq 1 "$days"); do
    echo "pub mod d${day};" >> "$mod_file"

    cat << EOF > "$directory/y${year}/d${day}.rs"
pub fn solve() {
    println!("Solution for ${year}-12-${day} is not implemented yet.");
}
EOF
done
