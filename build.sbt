libraryDependencies += "org.scalatest" %% "scalatest" % "3.1.0" % "test"
libraryDependencies += "com.wavesenterprise" % "we-core" % "1.12.3" % "test"

lazy val wevm = (project in file("."))
  .settings(
    sbtJniCoreScope := Compile, // because we use `NativeLoader`, not the `@nativeLoader` macro
    classLoaderLayeringStrategy := ClassLoaderLayeringStrategy.Flat
  )
  .aggregate(native)
  .dependsOn(native % Runtime)

// see this tutorial https://engineering.avast.io/scala-and-rust-interoperability-via-jni/
lazy val native = (project in file("native"))
  .settings(
    nativeCompile / sourceDirectory := baseDirectory.value,
  )
  .enablePlugins(JniNative)
