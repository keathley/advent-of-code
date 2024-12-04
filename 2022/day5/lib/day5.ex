defmodule Day5 do
  def load do
    File.read!("input.txt")
  end

  def part2(input \\ load()) do
    {stacks, ops} =
      input
      |> parse()

    result = Enum.reduce(ops, stacks, fn op, stacks ->
      move_slice(stacks, op.count, op.from, op.to)
    end)

    answer =
      result
      |> Enum.map(fn {_, stack} -> peek(stack) end)
      |> Enum.join()

    IO.puts "Part 2: #{answer}"
  end

  def part1(input \\ load()) do
    {stacks, ops} =
      input
      |> parse()

    result = Enum.reduce(ops, stacks, fn op, stacks ->
      Enum.reduce((1..op.count), stacks, fn _, stacks ->
        move(stacks, op.from, op.to)
      end)
    end)

    answer =
      result
      |> Enum.map(fn {_, stack} -> peek(stack) end)
      |> Enum.join()

    IO.puts "Part 1: #{answer}"
  end

  def parse(input \\ load()) do
    lines = String.split(input, "\n")
    {stacks, proc} = Enum.split_while(lines, fn line -> line != "" end)
    [ids | rest] = Enum.reverse(stacks)
    stacks = Enum.reverse(rest)

    ids =
      ids
      |> String.split()
      |> Enum.map(&String.to_integer/1)
      |> Enum.map(fn id -> {id, stack()} end)
      |> Enum.into(%{})

    initial_operations =
      stacks
      |> Enum.map(fn line -> String.graphemes(line) end)
      |> Enum.map(fn line -> parse_stack(line) end)
      |> Enum.reverse()
      |> List.flatten()

    stacks = Enum.reduce(initial_operations, ids, fn {id, crate}, stacks -> update_in(stacks, [id], & push(&1, crate)) end)

    operations =
      proc
      |> Enum.reject(& &1 == "" || &1 == " ")
      |> Enum.map(fn line -> Regex.run(~r/move (\d+) from (\d+) to (\d+)/, line) end)
      |> Enum.map(fn results -> Enum.drop(results, 1) end)
      |> Enum.map(fn values -> Enum.map(values, &String.to_integer/1) end)
      |> Enum.map(fn [count, from, to] -> %{count: count, from: from, to: to} end)

    {stacks, operations}
  end

  def stack, do: []

  def move(stacks, from, to) do
    {item, new_from} = pop(stacks[from])
    new_to = push(stacks[to], item)
    %{stacks | from => new_from, to => new_to}
  end

  def move_slice(stacks, count, from, to) do
    {items, new_from} = Enum.split(stacks[from], count)
    new_to = items ++ stacks[to]
    %{stacks | from => new_from, to => new_to}
  end

  def push(stack, nil), do: stack
  def push(stack, item), do: [item | stack]

  def peek([]), do: nil
  def peek([item | _rest]), do: item

  def pop([]), do: {nil, []}
  def pop([item | rest]), do: {item, rest}

  defp parse_stack(list, i \\ 1, acc \\ [])
  defp parse_stack([], _, acc), do: acc
  defp parse_stack([_, crate, _ | rest], i, acc) do
    crate = if crate == " ", do: nil, else: crate
    rest = case rest do
      [" " | rest] -> rest
      _ -> rest
    end
    parse_stack(rest, i+1, [{i, crate} | acc])
  end
end
