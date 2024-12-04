defmodule AdventOfCode.DayThreeTest do
  use ExUnit.Case, async: true
  doctest AdventOfCode.DayThree

  alias AdventOfCode.DayThree

  test "examples" do
    assert DayThree.part_one(1) == 0
    assert DayThree.part_one(12) == 3
    assert DayThree.part_one(23) == 2
    assert DayThree.part_one(1024) == 31

    assert DayThree.part_two(265149) == 266330
  end
end
