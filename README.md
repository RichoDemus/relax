# title

## Run on raspberry
```
scp Cargo.* pi@pi.local:relax/ && scp -r src pi@pi.local:relax/ && ssh pi@pi.local "cd relax; /home/pi/.cargo/bin/cargo run --features gpio"
```

## run locally
```
cargo run -- --client-id <slack client id> --client-secret <slack client secret>
or
cargo run -- --user-id <slack user id> --token <slack api token>
```
