defmodule Main do
  def start(_type, _args) do
    IO.puts "I have no idea what I'm doing"
    {:ok, _} = Plug.Adapters.Cowboy.http(Backend, [])
  end
end
