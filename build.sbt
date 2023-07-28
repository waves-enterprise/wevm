lazy val root = (project in file("."))
  .aggregate(core, native)

// see this tutorial https://engineering.avast.io/scala-and-rust-interoperability-via-jni/
lazy val native = (project in file("native"))
  .settings(
    nativeCompile / sourceDirectory := baseDirectory.value,
  )
  .enablePlugins(JniNative)

lazy val core = project
  .in(file("core"))
  .settings(
    sbtJniCoreScope := Compile, // because we use `NativeLoader`, not the `@nativeLoader` macro
    classLoaderLayeringStrategy := ClassLoaderLayeringStrategy.Flat
  )
  .dependsOn(native % Runtime)
