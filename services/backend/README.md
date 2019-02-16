# Backend

## Running locally:
```
iex -S mix
{:ok, _} = Plug.Adapters.Cowboy.http(Backend, [])
http localhost:4000

## Running in docker:
```
docker build -t s12g-backend .& docker run -it --rm -p 4000:4000 s12g-backend
```

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `backend` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:backend, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/backend](https://hexdocs.pm/backend).

