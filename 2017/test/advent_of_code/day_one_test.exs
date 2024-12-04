defmodule AdventOfCode.DayOneTest do
  use ExUnit.Case, async: true

  alias AdventOfCode.DayOne

  test "part_one/1 sums up digits" do
    assert DayOne.part_one(1122) == 3
    assert DayOne.part_one(1111) == 4
    assert DayOne.part_one(1234) == 0
    assert DayOne.part_one(91212129) == 9
  end

  test "calculate/2 sums up digits based on offset" do
    assert DayOne.part_two(1212) == 6
    assert DayOne.part_two(1221) == 0
    assert DayOne.part_two(123425) == 4
    assert DayOne.part_two(123123) == 12
    assert DayOne.part_two(12131415) == 4
  end
end
