defmodule AdventOfCode.DayFiveTest do
  use ExUnit.Case, async: true

  alias AdventOfCode.DayFive

  test "part one" do
    instructions = """
    0
    3
    0
    1
    -3
    """

    assert DayFive.part_one(instructions) == 5
  end

  test "part two" do
    instructions = """
    0
    3
    0
    1
    -3
    """

    assert DayFive.part_two(instructions) == 10
  end
end
