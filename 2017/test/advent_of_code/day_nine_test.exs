defmodule AdventOfCode.DayNineTest do
  use ExUnit.Case, async: true

  alias AdventOfCode.DayNine

  test "part one" do
    assert DayNine.part_one("{}") == 1
    assert DayNine.part_one("{{{}}}") == 3
    assert DayNine.part_one("{{},{}}") == 3
    assert DayNine.part_one("{{{},{},{{}}}}") == 6
    assert DayNine.part_one("{<{},{},{{}}>}") == 1
    assert DayNine.part_one("{<a>,<a>,<a>,<a>}") == 1
    assert DayNine.part_one("{{<a>},{<a>},{<a>},{<a>}}") == 5
    assert DayNine.part_one("{{<!>},{<!>},{<!>},{<a>}}") == 2
  end
end
