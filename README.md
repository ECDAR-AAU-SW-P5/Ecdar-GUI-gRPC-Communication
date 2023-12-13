# Ecdar-ProtoBuf-rs
A rust lib for communicating with Ecdar servers

Run ```git pull --recurse-submodules``` to update the submodule


## Custom compilation
To compile a lib with a different protobuff library set enviroment variable `ECDAR_PROTOBUFF_DIR`. Default is `./Ecdar-ProtoBuf/` the submodule included.
```
ECDAR_PROTOBUFF_DIR=/absolute/path/to/dir/
```
It is important that it is the absolute path because you cannot know the working directory when cargo build is called from a build script


To change the root file from `services.proto` do
```
ECDAR_PROTOBUFF_ROOT=file_name.proto
```


#### Example use case

```
ECDAR_PROTOBUFF_DIR=/dev/project/dir/Ecdar-ProtoBuf cargo run

```



