defmodule AdventOfCode.DayTwo do
  def part_one(spreadsheet) do
    checksum(spreadsheet, &Enum.min_max/1, fn {min, max} -> max - min end)
  end

  def part_two(spreadsheet) do
    checksum(spreadsheet, &evenly_divisible/1, fn {a, b} -> b / a end)
  end

  def evenly_divisible(list) do
    import Enum
    import List, only: [delete_at: 2, first: 1]

    list
    |> with_index
    |> flat_map(fn {x, i} -> map(delete_at(list, i), fn y -> {x, y} end) end)
    |> filter(fn {x, y} -> rem(y, x) == 0 end)
    |> first
  end

  def checksum(spreadsheet, selector, aggregate) do
    spreadsheet
    |> String.split("\n", trim: true)
    |> Enum.map(& String.split(&1))
    |> Enum.map(fn is -> Enum.map(is, &String.to_integer/1) end)
    |> Enum.map(& selector.(&1))
    |> Enum.map(& aggregate.(&1))
    |> Enum.sum
  end
end
