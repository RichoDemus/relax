# Relax, a non intrusive slack bot

## Raspberry setup
1. Install and run [ServoBlaster](https://github.com/richardghirst/PiBits/tree/master/ServoBlaster)
2. Connect a servo to 5V, ground and GPIO3
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
