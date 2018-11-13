To cross-compile from x86_64 to armv7hf, use:
```
 $ sudo apt install gcc-7-multilib-arm-linux-gnueabihf
```
and then add in _~/.cargo/config_:

`[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc-7"
`
to install and able the armv7 compiler

Finally add the linker for armv7 architecture with:
```
 $ rustup target add armv7-unknown-linux-gnueabihf
```


To build the project for the armv7 target use:
``` 
 $ cargo build --target=armv7-unknown-linux-gnueabihf
```
