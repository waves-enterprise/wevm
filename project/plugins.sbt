Seq(
  "com.github.sbt"   % "sbt-jni"      % "1.7.0",
  "org.scalameta"    % "sbt-scalafmt" % "2.4.6",
  "com.eed3si9n"     % "sbt-assembly" % "1.2.0",
  "com.typesafe.sbt" % "sbt-git"      % "1.0.0"
).map(addSbtPlugin)
