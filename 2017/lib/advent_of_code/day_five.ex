defmodule AdventOfCode.DayFive do
  defmodule Program do
    defmacro __using__(_args) do
      quote do
        @behaviour unquote(__MODULE__)

        def run(instructions) do
          instructions
          |> parse()
          |> execute(0, 0)
        end

        def parse(string) do
          string
          |> String.split("\n", trim: true)
          |> Enum.map(&to_opcode/1)
          |> Enum.reduce({%{}, 0}, &build_program/2)
          |> elem(0)
        end

        def to_opcode(char) do
          char
          |> String.to_integer
        end

        def build_program(op, {program, pointer}) do
          {Map.put(program, pointer, op), pointer+1}
        end

        def done?(instructions, pointer) do
          is_nil(instructions[pointer])
        end

        def execute(instructions, pointer, steps) do
          if done?(instructions, pointer) do
            steps
          else
            new_pointer = instructions[pointer] + pointer
            new_instructions = update_in(instructions, [pointer], &new_offset/1)
            execute(new_instructions, new_pointer, steps+1)
          end
        end
      end
    end

    @callback new_offset(integer()) :: integer()
  end

  defmodule PartOne do
    use Program

    def new_offset(offset), do: offset + 1
  end

  defmodule PartTwo do
    use Program

    def new_offset(offset) when offset >= 3, do: offset-1
    def new_offset(offset), do: offset+1
  end

  def part_one(instructions), do: PartOne.run(instructions)

  def part_two(instructions), do: PartTwo.run(instructions)
end
