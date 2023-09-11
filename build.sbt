import scala.sys.process._

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

lazy val buildWAT = taskKey[Unit]("build WAT contract")
buildWAT := {
  val wat2wasm = baseDirectory.value / "native" / "wat2wasm"

  val input = wat2wasm / "wat"
  val output = baseDirectory.value / "src" / "test" / "resources"

  val success: Int = ( s"cargo run --manifest-path ${wat2wasm}/Cargo.toml -- --input ${input} --output ${output}" #&& "echo WAT build successfully" ! )
}
