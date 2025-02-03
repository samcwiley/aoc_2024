import scala.io.Source

object Main {
  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("../../src/.inputs/input04.txt").mkString
    val data = parseGridChars(input)
    
    val part1Result = part1(data)
    println(s"Part 1: $part1Result")
    
    val part2Result = part2(data)
    println(s"Part 2: $part2Result")
  }
  
  def parseGridChars(input: String): Array[Array[Char]] = {
    input.split("\r?\n").map(_.toCharArray)
  }
  
  def part1(grid: Array[Array[Char]]): String = {
    var result = 0
    
    for (i <- grid.indices; j <- grid(i).indices) {
      result += checkDirections(grid, i, j, 4).count(_ == Seq('X', 'M', 'A', 'S'))
    }
    
    result.toString
  }
  
  def part2(grid: Array[Array[Char]]): String = {
    var result = 0
    
    for (i <- 1 until grid.length - 1; j <- 1 until grid(i).length - 1) {
      if (grid(i)(j) == 'A') {
        val diff1 = (grid(i - 1)(j - 1) - grid(i + 1)(j + 1)).abs
        val diff2 = (grid(i - 1)(j + 1) - grid(i + 1)(j - 1)).abs
        if (diff1 == ('S' - 'M').abs && diff2 == ('S' - 'M').abs) {
          result += 1
        }
      }
    }
    
    result.toString
  }
  
  def checkDirections(grid: Array[Array[Char]], x: Int, y: Int, n: Int): Seq[Seq[Char]] = {
    val deltas = Seq(
      (0, -1),
      (0, 1),
      (-1, 0),
      (1, 0),
      (-1, -1),
      (1, -1),
      (-1, 1),
      (1, 1)
    )
    
    deltas.map { case (dx, dy) =>
      (0 until n).flatMap { i =>
        val nx = x + i * dx
        val ny = y + i * dy
        if (nx >= 0 && ny >= 0 && nx < grid.length && ny < grid(nx).length) {
          Some(grid(nx)(ny))
        } else None
      }
    }
  }
}
