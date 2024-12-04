defmodule Day7 do
  # This one sucked. I wrote these functions and then did the rest in iex and pipes with enum stuff.
  def load do
    File.read!("input.txt")
  end

  def parse(input \\ load()) do
    input
    |> String.split("\n")
    |> Enum.reject(& &1 == "")
    |> Enum.map(&parse_line/1)
  end

  def directories_with_size(tree) do
    keys = Map.keys(tree)
    directories_with_size(keys, tree, 0, [])
  end

  def directories_with_size([], _tree, size, acc), do: {size, acc}
  def directories_with_size([key | rest], tree, size, acc) do
    case key do
      {:dir, dir} ->
        sub_dir = tree[key]
        {directory_size, new_acc} = directories_with_size(sub_dir)
        acc = [{dir, directory_size} | new_acc] ++ acc
        directories_with_size(rest, tree, directory_size + size, acc)


      {:file, _name} ->
        file_size = tree[key]
        directories_with_size(rest, tree, file_size+size, acc)
    end
  end

  def build_tree(parsed) do
    Process.put(:path, ["/"])
    Process.put(:tree, %{{:dir, "/"} => %{}})

    for entry <- parsed do
      case entry do
        {:cd, dir} -> cd(dir)
        :ls -> nil
        {:dir, dir} -> mkdir(dir)
        {:file, name, size} -> mkfile(name, size)
      end
    end

    Process.get(:tree)
  end

  def cd("/"), do: Process.put(:path, ["/"])
  def cd("..") do
    [_|rest] = Process.get(:path)
    Process.put(:path, rest)
  end
  def cd(dir) do
    path = Process.get(:path)
    Process.put(:path, [dir | path])
  end

  def mkfile(name, size) do
    path = Process.get(:path)
    path = Enum.reverse(path)
    tree = Process.get(:tree)

    tree = traverse(path, tree, fn parent ->
      nil = parent[name]
      Map.put(parent, {:file, name}, size)
    end)

    Process.put(:tree, tree)
  end

  def mkdir(dir) do
    path = Process.get(:path)
    path = Enum.reverse(path)
    tree = Process.get(:tree)

    tree = traverse(path, tree, fn parent ->
      nil = parent[dir]
      Map.put(parent, {:dir, dir}, %{})
    end)

    Process.put(:tree, tree)
  end

  defp traverse([dir], tree, f) do
    sub = tree[{:dir, dir}]
    Map.put(tree, {:dir, dir}, f.(sub))
  end

  defp traverse([dir | rest], tree, f) do
    case tree[{:dir, dir}] do
      nil -> throw :directory_does_not_exist
      sub_tree ->
        sub_tree = traverse(rest, sub_tree, f)
        Map.put(tree, {:dir, dir}, sub_tree)
    end
  end

  defp parse_line("$ cd " <> dir), do: {:cd, dir}
  defp parse_line("$ ls" <> _), do: :ls
  defp parse_line("dir " <> dir), do: {:dir, dir}
  defp parse_line(file) do
    case Regex.run(~r/^(\d+) (.*)$/, file) do
      [_, size, name] -> {:file, name, String.to_integer(size)}
      _ ->
        IO.inspect(file, label: "Unparseable")
        throw :error
    end
  end
end
