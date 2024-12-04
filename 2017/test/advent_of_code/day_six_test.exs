defmodule AdventOfCode.DaySixTest do
  use ExUnit.Case, async: true

  alias AdventOfCode.DaySix

  @initial_banks "0 2 7 0"

  test "part one" do
    assert DaySix.part_one(@initial_banks) == 5
  end

  test "part two" do
    assert DaySix.part_two(@initial_banks) == 4
  end
end
