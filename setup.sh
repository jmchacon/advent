#!/bin/sh
cp ../advent/run.sh .
git add run.sh
cp ../advent/template-gitignore .gitignore
cp ../advent/template-top-level-Cargo.toml ./Cargo.toml
git add Cargo.toml
mkdir .cargo
cp ../advent/config.toml ./.cargo/config.toml
git add ./.cargo/config.toml
mkdir .github
mkdir .github/workflows
cp -rp ../advent/github/* .github/workflows
for i in `seq 1 25`; do
	mkdir day$i
	mkdir day$i/src
	sed -e "s:dayXX:day$i:g" < ../advent/template-Cargo.toml > day$i/Cargo.toml
	sed -e "s:dayXX:day$i:g" < ../advent/template.rs > day$i/src/main.rs
        git add day$i/Cargo.toml day$i/src/main.rs
done
