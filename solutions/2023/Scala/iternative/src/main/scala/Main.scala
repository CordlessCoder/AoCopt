import scala.io.StdIn

object Main {
  def main(args: Array[String]): Unit = {
    val str = Iterator
      .continually(StdIn.readLine)
      .takeWhile(_ != null)
      .mkString("\n")

    val before = System.nanoTime()

    val answerWithIterator =
      for line <- str.linesIterator yield {
        val digits = line.filter(_.isDigit).map(_.toInt - 48).toList
        val num = digits.head * 10 + digits.last
        num
      }

    val ans = answerWithIterator.sum

    val after = System.nanoTime()

    println(ans)

    val diff = after - before
    println(s"Took $diff nanoseconds")
  }
}
