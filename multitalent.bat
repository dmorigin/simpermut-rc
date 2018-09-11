@echo off

for %%b in (1311211 1211211 3311312 3211312 3331312 3231312 3211213) do (
    cargo run -- -y -t %%b matabei.simc
)
