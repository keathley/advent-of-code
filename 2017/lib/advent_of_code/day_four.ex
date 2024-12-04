defmodule AdventOfCode.DayFour do
  def part_one(string) do
    string
    |> parse
    |> valid_count(& &1)
  end

  def part_two(string) do
    string
    |> parse
    |> valid_count(&lexically_order/1)
  end

  def parse(string) do
    string
    |> String.split("\n", trim: true)
    |> Enum.map(&String.split/1)
  end

  def valid_count(lines, f) do
    lines
    |> Enum.map(& valid?(&1, f))
    |> Enum.filter(& &1)
    |> Enum.count
  end

  def lexically_order(word) do
    word
    |> String.graphemes
    |> Enum.sort
    |> Enum.join
  end

  def valid?(words, f) do
    words
    |> Enum.uniq_by(f)
    |> Enum.count == Enum.count(words)
  end
end
