FROM elixir:1.8.1

RUN apt-get update && apt-get install -y inotify-tools

WORKDIR "/opt/app"

RUN mix local.hex --force && mix local.rebar --force

COPY config/* config/
COPY mix.exs mix.lock ./
RUN mix do deps.get, deps.compile

COPY . ./

CMD elixir --sname app -S mix run --no-halt
