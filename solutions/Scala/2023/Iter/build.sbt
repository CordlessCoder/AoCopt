ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "3.4.0"

resolvers += "Sonatype OSS Snapshots" at
  "https://oss.sonatype.org/content/repositories/releases"

libraryDependencies ++= Seq(
  ("com.storm-enroute" %% "scalameter" % "0.21").cross(CrossVersion.for3Use2_13)
)

testFrameworks += new TestFramework("org.scalameter.ScalaMeterFramework")
Test / parallelExecution := false

fork := true

outputStrategy := Some(StdoutOutput)

connectInput := true

lazy val root = (project in file("."))
  .enablePlugins(net.virtualvoid.optimizer.SbtOptimizerPlugin)
  .settings(
    name := "AocOne",
    version := "0",
    idePackagePrefix := Some("org.risa.aoc"),
  )
