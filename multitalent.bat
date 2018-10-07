@echo off

for %%b in (1311211 1211211) do (
    cargo run -- -y -t %%b matabei.simc
)
