defmodule AdventOfCode.DaySeven do
  defmodule Tree do
    def insert(node) do
    end

    def root(ls) do
      ds =
        ls
        |> Enum.map(fn {_, _, ds} -> ds end)
        |> Enum.concat

      ls
      |> Enum.map(fn {name, _, _} -> name end)
      |> Enum.find(fn i -> !Enum.any?(ds, & &1 == i) end)
    end

    def build(ls, name, acc) do
      {name, weight, ds} = Enum.find(ls, fn {e_name, _, _} -> name == e_name end)
      subt = Enum.reduce(ds, [], & build(ls, &1, &2))
      subt_weight = Enum.map(subt, fn {_, w, _} -> w end) |> Enum.sum
      [{name, weight + subt_weight, subt} | acc]
      # Map.put(acc, {name, weight}, Enum.reduce(ds, %{}, & build(ls, &1, &2)))
    end

    def find_imbalance([{name, weight, subt}]) do
      if uniq_weights(subt) |> Enum.count != 1 do
        [t1, t2] =
          subt
          |> uniq_weights
        IO.puts "Subtree is imbalanced"
        {n1, w1, _} = t1
        {n2, w2, _} = t2
        IO.inspect([n1, w1], label: "Tree 1")
        IO.inspect([n2, w2], label: "Tree 2")
        # IO.puts "Subtree is imbalanced by #{w2-w1}"
      else
        find_imbalance(subt)
      end
    end

    def uniq_weights(tree) do
      tree
      |> Enum.uniq_by(fn {_, weight, _} -> weight end)
    end

    def tree_weight([]), do: 0
    def tree_weight([{_, weight, tree}]), do: weight + tree_weight(tree)
    def tree_weight([{_, weight, tree} | tail]) do
      weight + tree_weight(tree) + tree_weight(tail)
    end

    def find_imbalance(tree) do

    end

    def program_weight({{_, weight}, subp}) do
      IO.inspect(weight)
      IO.inspect(subp)
      weight + Enum.reduce(subp, 0, fn({{_, w}, _}, acc) -> acc + w end)
    end
  end

  defmodule DAG do
    defstruct [:graph, :weights]

    def build(vertices) do
      %__MODULE__{
        graph: build_dag(vertices),
        weights: build_weights(vertices),
      }
    end

    def build_dag(vertices) do
      dag = :digraph.new

      Enum.each(vertices, fn {name, _, ds} ->
        :digraph.add_vertex(dag, name)
        Enum.each(ds, fn dn ->
          :digraph.add_vertex(dag, dn)
          :digraph.add_edge(dag, name, dn)
        end)
      end)

      dag
    end

    def build_weights(list) do
      Enum.reduce(list, %{}, fn {name, weight, _}, acc -> Map.put(acc, name, weight) end)
    end

    def root(%{graph: dag}) do
      dag
      |> toposort
      |> List.first
    end

    def weight(%{weights: weights}, name) do
      IO.inspect(name, label: "Name")
      IO.inspect(weights[name], label: "Weight")
      weights[name]
    end

    defp toposort(dag), do: :digraph_utils.topsort(dag)

    def neighbors(%{graph: dag}, v), do: :digraph.out_neighbours(dag, v)
  end

  def part_one(list) do
    list
    |> parse_list
    |> DAG.build
    |> DAG.root
  end

  def part_two(list) do
    dag =
      list
      |> parse_list
      |> DAG.build

    root = DAG.root(dag)
    weights(dag, root)
  end

  def weights(dag, root) do
    neighbours = DAG.neighbors(dag, root)
    Enum.map(neighbours, & DAG.weight(dag, &1))
  end
  def weights(dag, node, []) do
    DAG.weight(dag, node)
  end
  def weights(dag, node, neighbors) do
    if weights_match?(dag, node) do
    end
  end

  def weights_match?(dag, node) do
    dag
    |> Enum.map(& DAG.weight(dag, &1))
    |> Enum.uniq
    |> Enum.count == 1
  end
  #   weights(dag, root, neighbours, 0)
  # end
  # def weights(_dag, _v, [], weight), do: weight
  # def weights(dag, v, [head | tail], weight) do
  #   weight + weights(dag, head, neighbors(dag, head)) + weights(dag, v, tail, weight)
  # end

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
    {:ok, program} = :list_parser.parse(tokens)
    program
  end

  def normalize({name, weight, descendants}) do
    {to_string(name), weight, Enum.map(descendants, &to_string/1)}
  end

  def tokenize(charlist) do
    {:ok, tokens, _} = :list_lexer.string(charlist)
    tokens
  end
end
