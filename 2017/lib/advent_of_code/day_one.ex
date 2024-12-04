defmodule AdventOfCode.DayOne do
  def part_one(int) do
    digits =
      int
      |> Integer.digits

    calculate(digits, 1)
  end

  def part_two(int) do
    digits =
      int
      |> Integer.digits

    calculate(digits, half(digits))
  end

  def half(list) do
    round(Enum.count(list) / 2)
  end

  def calculate(digits, offset\\1) do
    digits
    |> Enum.zip(rotate(digits, offset))
    |> Enum.map(&value/1)
    |> Enum.sum
  end

  def rotate(list, 0), do: list
  def rotate([head | tail], n), do: rotate(tail ++ [head], n-1)

  def value({a, b}) when a == b, do: a
  def value(_), do: 0
end
