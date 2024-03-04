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
        val first = line.iterator.filter(_.isDigit).map(_.toInt - 48).next()
        val last = line.reverseIterator.filter(_.isDigit).map(_.toInt - 48).next()
        val num = first * 10 + last
        num
      }

    val ans = answerWithIterator.sum

    val after = System.nanoTime()

    println(ans)

    val diff = after - before
    println(s"Took $diff nanoseconds")
  }
}
