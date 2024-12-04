defmodule AdventOfCode.DaySix do
  def part_one(input) do
    {_, _, count} = reallocate(input)
    count
  end

  def part_two(input) do
    {banks, previous, _} = reallocate(input)
    i = Enum.find_index(previous, & banks_match(&1, banks))
    i + 1
  end

  def reallocate(input) do
    input
    |> String.split(" ", trim: true)
    |> Enum.map(&String.to_integer/1)
    |> Enum.with_index
    |> Enum.map(fn {v, i} -> {i, v} end)
    |> Enum.into(%{})
    |> loop([], 0)
  end

  def loop(banks, previous, count) do
    if seen?(banks, previous) do
      {banks, previous, count}
    else
      do_loop(banks, previous, count)
    end
  end

  def do_loop(banks, previous, count) do
    banks
    |> empty_largest
    |> redistribute
    |> loop([banks | previous], count+1)
  end

  def seen?(banks, previous) do
    Enum.any?(previous, & banks_match(&1, banks))
  end

  def banks_match(a, b), do: a == b

  def empty_largest(banks) do
    {i, v} = find_largest(banks)

    {%{banks | i => 0}, v, i+1}
  end

  def redistribute({banks, 0, _}), do: banks
  def redistribute({banks, v, i}) do
    i = cycle_index(banks, i)
    redistribute({%{banks | i => banks[i]+1}, v-1, i+1})
  end

  def cycle_index(banks, i) do
    cond do
      Map.has_key?(banks, i) -> i
      true                   -> 0
    end
  end

  def find_largest(banks) do
    banks
    |> Enum.max_by(fn {_, v} -> v end)
  end

  def pp(banks) do
    Enum.map(banks, fn {_, v} -> v end)
  end
end
