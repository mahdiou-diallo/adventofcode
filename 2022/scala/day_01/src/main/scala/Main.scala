@main def hello: Unit =
  // println(System.getProperty("user.dir"))
  val source = io.Source.fromFile("../../data/2022/01.txt")
  val lines =
    try source.mkString
    finally source.close()

  val calories = lines.split("(?:\r?\n){2}").map(getElfCalories(_)).toList
  val top3 = pickTopN(3, calories)

  println(s"max = ${top3.max}, sum top 3 = ${top3.sum}, top3 = $top3")
  val top3_v2 = topN(3, calories)
  println(s"max = ${top3_v2.max}, sum top 3 = ${top3_v2.sum}, top3 = $top3_v2")

def getElfCalories(text: String): Int =
  text.split("\n").map(_.toInt).sum

def pickTopN[T](k: Int, iterable: Iterable[T])(implicit
    ord: Ordering[T]
): Seq[T] =
  val q = collection.mutable.PriorityQueue[T](iterable.toSeq: _*)
  val end = Math.min(k, q.size)
  (1 to end).map(_ => q.dequeue())

def topN(k: Int, list: List[Int]): List[Int] =
  def partition(
      value: Int,
      l: List[Int],
      left: List[Int],
      right: List[Int]
  ): (List[Int], List[Int]) =
    l match
      case head :: next =>
        if head > value then partition(value, next, head :: left, right)
        else partition(value, next, left, head :: right)
      case Nil => (left, right)

  def partitionUntil(list: List[Int], n: Int): List[Int] =
    val value = list.head
    val (left, right) = partition(value, list, Nil, Nil)
    left.length match
      case len if len == n => left
      case len if len > n  => partitionUntil(left, n)
      case len: Int        => left ++: partitionUntil(right, n - len)

  partitionUntil(list, k)
