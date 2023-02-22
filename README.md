# wevm

WebAssembly Engine for Waves Enterprise 

## Scala mock

```sh
# Compile
scalac VM.scala

# javah needs access to scala-library.jar
LIBS_HOME=/usr/share/scala-2.11/lib
CP=$LIBS_HOME/scala-library.jar

javah -cp $CP:. VM
```

```sh
# Run
scala -cp . -Djava.library.path=../target/debug/ VM
```
