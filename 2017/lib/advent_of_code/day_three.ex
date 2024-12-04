defmodule AdventOfCode.DayThree do
  def part_one(label) do
    label
    |> to_point
    |> manhattan_distance({0, 0})
  end

  @doc """
  Extract values from a stream until we find a value that is greater then our
  starting value.
  """
  def part_two(n) do
    sum_square() # returns a stream of maps that represent the next generated square
    |> Stream.map(& Enum.max_by(&1, fn({_, k}) -> k end)) # find the max value in the map
    |> Stream.map(fn {_, v} -> v end) # convert keys and values to just values
    |> Enum.find(& &1 > n)
  end

  def to_point(label) do
    graph()
    |> Enum.take(label)
    |> List.last
  end

  def manhattan_distance({ax, ay}, {bx, by}) do
    abs(ax - bx) + abs(ay - by)
  end

  def directions, do: [
    fn({a, b}) -> {a+1, b}     end, # right
    fn({a, b}) -> {a,   b + 1} end, # up
    fn({a, b}) -> {a-1, b}     end, # left
    fn({a, b}) -> {a,   b - 1} end, # down
  ]

  def diagonals, do: [
    fn({a, b}) -> {a+1, b+1} end, # top-right
    fn({a, b}) -> {a-1, b+1} end, # top-left
    fn({a, b}) -> {a-1, b-1} end, # bottom-left
    fn({a, b}) -> {a+1, b-1} end, # bottom-right
  ]

  @doc """
  A list of functions that help us find adjacent values. Its in a list so that
  we can easily map over it from a single point like so:
  ```
  adjacents()
  |> Enum.map(fn f -> f.({0, 0}) end)
  ```
  """
  def adjacents, do: directions() ++ diagonals()

  @doc """
  Generate a square in a spiral by summing adjacent squares.
  Each scan call returns a new Map which contains the known square values
  at that point.
  """
  def sum_square do
    graph()
    |> Stream.drop(1)
    |> Stream.scan(%{{0,0} => 1}, &set_value/2)
  end

  def set_value(pos, m) do
    Map.put(m, pos, get_value(pos, m))
  end

  def get_value(pos, m) do
    adjacents()
    |> Enum.map(fn f -> f.(pos) end)
    |> Enum.map(fn p -> Map.get(m, p, 0) end)
    |> Enum.sum
  end

  @doc """
  Builds an infinite list representing coordinates in a spiral graph
  """
  def graph do
    g =
      movements()
      |> Stream.scan({0, 0}, fn(f, pos) -> f.(pos) end)

    Stream.concat([{0, 0}], g)
  end

  @doc """
  Infinite stream of movement functions that "move" in a spiral pattern.
  A pattern would look like:
  `[right, up, left, left, down, down, right, right, right]`
  """
  def movements do
    steps()
    |> Stream.zip(Stream.cycle(directions()))
    |> Stream.map(fn {i, f} -> replicate(f, i) end)
    |> Stream.concat
  end

  @doc """
  Generates an increasing infinite list where each item is duplicated

  ## Examples
    iex> AdventOfCode.DayThree.steps |> Enum.take(4)
    [1,1,2,2]
    iex> AdventOfCode.DayThree.steps |> Enum.take(10)
    [1,1,2,2,3,3,4,4,5,5]
  """
  def steps do
    infinite_list()
    |> Stream.zip(infinite_list())
    |> Stream.flat_map(fn {a, b} -> [a, b] end)
  end

  def infinite_list, do: Stream.iterate(1, &(&1 + 1))

  @doc """
  Replicates an element a number of times.
  """
  def replicate(elem, count) do
    Stream.repeatedly(fn -> elem end) |> Enum.take(count)
  end
end
