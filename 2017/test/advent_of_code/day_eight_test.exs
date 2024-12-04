defmodule AdventOfCode.DayEightTest do
  use ExUnit.Case, async: true

  alias AdventOfCode.DayEight

  test "part one" do
    str = """
    b inc 5 if a > 1
    a inc 1 if b < 5
    c dec -10 if a >= 1
    c inc -20 if c == 10
    """

    assert DayEight.part_one(str) == 1
  end
end
