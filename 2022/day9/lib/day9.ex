defmodule Day9 do
  def load(file \\ "input.txt") do
    File.read!(file)
  end

  def parse(input \\ load()) do
    input
    |> String.split("\n")
    |> Enum.reject(& &1 == "")
    |> Enum.map(fn line -> Regex.run(~r/(U|L|R|D) (\d+)/, line) end)
    |> Enum.map(& Enum.drop(&1, 1))
    |> Enum.map(fn [dir, i] -> {:"#{dir}", String.to_integer(i)} end)
    |> Enum.flat_map(fn {dir, count} -> for _ <- 1..count, do: dir end)
  end

  def point(x, y) do
    %{x: x, y: y}
  end

  def close_enough?(p1, p2) do
    xdiff = abs(p1.x - p2.x)
    ydiff = abs(p1.y - p2.y)

    (xdiff == 0 && ydiff == 0) ||
    (xdiff == 0 && ydiff == 1) ||
    (xdiff == 1 && ydiff == 0) ||
    (xdiff == 1 && ydiff == 1)
  end

  def diagonal?(p1, p2) do
    xdiff = abs(p1.x - p2.x)
    ydiff = abs(p1.y - p2.y)

    (xdiff == 2 && ydiff == 1) ||
    (xdiff == 1 && ydiff == 2) ||
    (xdiff == 2 && ydiff == 2)
  end

  defp above?(p1, p2), do: p1.y > p2.y

  defp below?(p1, p2), do: p1.y < p2.y

  defp right?(p1, p2), do: p1.x > p2.x

  defp left?(p1, p2), do: p1.x < p2.x

  defp up(p), do: %{p | y: p.y+1}
  defp down(p), do: %{p | y: p.y-1}
  defp right(p), do: %{p | x: p.x+1}
  defp left(p), do: %{p | x: p.x-1}

  def move_diagonal(head, tail) do
    tail = if above?(head, tail) do
      up(tail)
    else
      down(tail)
    end

    if left?(head, tail) do
      left(tail)
    else
      right(tail)
    end
  end

  def simulate(head, tail) do
    cond do
      close_enough?(head, tail) -> tail
      diagonal?(head, tail) -> move_diagonal(head, tail)
      above?(head, tail) -> up(tail)
      below?(head, tail) -> down(tail)
      right?(head, tail) -> right(tail)
      left?(head, tail)  -> left(tail)
    end
  end

  def simulate(_head, [], new_tail) do
    Enum.reverse(new_tail)
  end
  def simulate(head, [next | rest], acc) do
    new_next = simulate(head, next)
    simulate(new_next, rest, [new_next | acc])
  end

  def full_sim(moves, points) do
    Process.put(:i, 1)
    Enum.reduce(moves, {points, %{}}, fn dir, {points, history} ->
      IO.write([IO.ANSI.home(), IO.ANSI.clear()])
      i = Process.get(:i)
      [head | tail] = points

      head = case dir do
        :U -> up(head)
        :D -> down(head)
        :R -> right(head)
        :L -> left(head)
      end

      tail = simulate(head, tail, [])
      last = List.last(tail)
      history = update_in(history, [last], & (&1 || 0) + 1)
      Process.put(:i, i + 1)
      points = [head | tail]

      display(points)
      Process.sleep(100)

      {points, history}
    end)
  end

  def part1(moves) do
    points = [point(0, 0), point(0, 0)]
    full_sim(moves, points)
  end

  def part2(moves) do
    points = for _ <- 0..9, do: point(11, 5)
    full_sim(moves, points)
  end

  def display(points, width \\ 26, height \\ 21) do
    points =
      points
      |> Enum.with_index(fn point, i -> {{point.x, point.y}, (if i == 0, do: "H", else: "#{i}")} end)
      |> Enum.reverse()
      |> Enum.into(%{})

    grid = Enum.reduce(height-1..0, "", fn y, grid ->
      row = Enum.reduce(0..width-1, "", fn x, row ->
        row <> (points[{x, y}] || ".")
      end)
      grid <> row <> "\n"
    end)

    IO.write(grid)
  end
end
