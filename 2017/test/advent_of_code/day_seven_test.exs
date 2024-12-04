defmodule AdventOfCode.DaySevenTest do
  use ExUnit.Case, async: true

  alias AdventOfCode.DaySeven

  test "part_one" do
    list = """
    pbga (66)
    xhth (57)
    ebii (61)
    havc (66)
    ktlj (57)
    fwft (72) -> ktlj, cntj, xhth
    qoyq (66)
    padx (45) -> pbga, havc, qoyq
    tknk (41) -> ugml, padx, fwft
    jptl (61)
    ugml (68) -> gyxo, ebii, jptl
    gyxo (61)
    cntj (57)
    """

    assert DaySeven.part_one(list) == "tknk"
  end

  test "part two" do
    
  end
end
