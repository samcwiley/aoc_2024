import scala.io.Source

case class Coord(x: Int, y: Int, w: Int, h: Int) {
  val inBounds: Boolean = x >= 0 && x < w && y >= 0 && y < h

  def applyVector(v: (Int, Int)): Coord = {
    val (dx, dy) = v
    Coord(x + dx, y + dy, w, h)
  }

  def diff(other: Coord): (Int, Int) = (x - other.x, y - other.y)

  override def toString: String = s"<$x, $y>"
}

object Main {
  def prepareInput(puzzleInput: String): (Map[Char, List[Coord]], Int, Int) = {
    val lines = puzzleInput.split("\n").toList
    val w = if (lines.nonEmpty) lines.head.length else 0
    val h = lines.length

    val antennas = lines.zipWithIndex.flatMap { case (row, y) =>
      row.zipWithIndex.collect {
        case (ch, x) if ch != '.' => (ch, Coord(x, y, w, h))
      }
    }.groupBy(_._1).view.map { case (k, v) => k -> v.map(_._2) }.toMap

    (antennas, w, h)
  }

  def partOne(puzzleInput: (Map[Char, List[Coord]], Int, Int)): Int = {
    val (antennas, _, _) = puzzleInput

    val antinodes = scala.collection.mutable.Set[Coord]()

    for ((_, locs) <- antennas) {
      for (a <- locs; b <- locs if a != b) {
        val vector = a.diff(b)

        val first = b.applyVector((-vector._1, -vector._2))
        val second = a.applyVector(vector)

        Seq(first, second).foreach { v =>
          if (v.inBounds) antinodes.add(v)
        }
      }
    }

    antinodes.size
  }

  def partTwo(puzzleInput: (Map[Char, List[Coord]], Int, Int)): Int = {
    val (antennas, _, _) = puzzleInput
    val antinodes = scala.collection.mutable.Set[Coord]()

    for ((_, locs) <- antennas) {
      for (a <- locs; b <- locs if a != b) {
        val vector = a.diff(b)
        var first = a
        var second = b

        while (first.inBounds || second.inBounds) {
          first = first.applyVector((-vector._1, -vector._2))
          second = second.applyVector(vector)

          Seq(first, second).foreach { v =>
            if (v.inBounds) antinodes.add(v)
          }
        }
      }
    }

    antinodes.size
  }

  def main(args: Array[String]): Unit = {
    val filename = "../../src/.inputs/input08.txt"
    val input = Source.fromFile(filename).mkString.trim
    Source.fromFile(filename).close()

    val parsedInput = prepareInput(input)
    val resultOne = partOne(parsedInput)
    println(s"Part One Result: $resultOne")

    val resultTwo = partTwo(parsedInput)
    println(s"Part Two Result: $resultTwo")
  }
}
