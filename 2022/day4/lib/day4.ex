defmodule Day4 do
  def solve do
    input = parse(File.read!("input.txt"))

    part1 = count(input, &contains?/1)
    IO.puts "Part 1: #{part1}"

    part2 = count(input, &overlap?/1)
    IO.puts "Part 2: #{part2}"
  end

  def count(entries, f) do
    entries
    |> Enum.map(fn entry -> if f.(entry), do: 1, else: 0 end)
    |> Enum.sum()
  end

  defp contains?({map1, map2}) do
    MapSet.subset?(map1, map2) || MapSet.subset?(map2, map1)
  end

  defp overlap?({map1, map2}) do
    Enum.any?(MapSet.intersection(map1, map2))
  end

  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.reject(&is_nil/1)
    |> Enum.reject(& &1 == "")
    |> Enum.map(&parse_line/1)
  end

  defp parse_line(line) do
    [_|rest] = Regex.run(~r/^(\d*)-(\d*),(\d*)-(\d*)$/, line)
    [a, b] =
      rest
      |> Enum.map(&String.to_integer/1)
      |> Enum.chunk_every(2)
      |> Enum.map(fn [a, b] -> MapSet.new(a..b) end)

    {a, b}
  end

  def to_string(entries) do
    string =
      entries
      |> Enum.map(fn {{s1, e1}, {s2, e2}} -> "#{s1}-#{e1},#{s2}-#{e2}" end)
      |> Enum.join("\n")

    string <> "\n"
  end
end
