import scala.io.Source

@main def day4: Unit = 
  val source = Source.fromFile("input.txt").mkString

  val pairs = source.split("\n").map(_.trim.filter(_ >= ' ')).map(_.split(",").flatMap(_.split("-").map(_.toInt)))

  val ranges = pairs.map({ case Array(x, y, z, w) => (x to y, z to w) })
  val p1 = ranges.count((r1, r2) => r1.forall(r2.contains) || r2.forall(r1.contains))
  val p2 = ranges.count((r1, r2) => r1.exists(r2.contains))
  
  println(s"Part1: ${p1}")
  println(s"Part2: ${p2}")