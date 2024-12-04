defmodule AdventOfCode.DayEight do
  def part_one(str) do
    {registers, _} =
      str
      |> parse_list
      |> execute(%{}, 0)

    largest_value(registers)
  end

  def part_two(str) do
    {_, largest_value} =
      str
      |> parse_list
      |> execute(%{}, 0)
  end

  def execute([], registers, largest_value), do: {registers, largest_value}
  def execute([op | tail], registers, lv) do
    new_registers = update_registers(registers, op)
    new_lv = largest_value(new_registers, lv)
    execute(tail, new_registers, new_lv)
  end

  def update_registers(registers, {op, register, value, condition}) do
    if valid_condition?(registers, condition) do
      update_register(registers, op, register, value)
    else
      registers
    end
  end

  def valid_condition?(registers, {condition, register, value}) do
    old_value = Map.get(registers, register, 0)
    apply(Kernel, condition, [old_value, value])
  end

  def update_register(registers, op, register, value) do
    old_value =
      registers
      |> Map.get(register, 0)

    Map.put(registers, register, updater(op, old_value, value))
  end

  def updater(:inc, old_value, value), do: old_value + value
  def updater(:dec, old_value, value), do: old_value - value

  def largest_value(registers, lv) do
    new_lv = largest_value(registers)
    if new_lv > lv, do: new_lv, else: lv
  end

  def largest_value(registers) do
    registers
    |> Enum.map(fn {_, v} -> v end)
    |> Enum.concat([0])
    |> Enum.max
  end

  def parse_list(str) do
    str
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_line/1)
  end

  def parse_line(line) do
    line
    |> to_charlist
    |> tokenize
    |> parse
    |> normalize
  end

  def parse(tokens) do
    {:ok, program} = :instruction_parser.parse(tokens)
    program
  end

  def normalize({op, register, value, condition}) do
    {parse_op(op), to_string(register), value, parse_condition(condition)}
  end

  def parse_op('inc'), do: :inc
  def parse_op('dec'), do: :dec

  def parse_condition({check, register, value}) do
    {to_atom(check), to_string(register), value}
  end

  def to_atom(charlist), do: charlist |> to_string |> String.to_atom

  def tokenize(charlist) do
    {:ok, tokens, _} = :instruction_lexer.string(charlist)
    tokens
  end
end
