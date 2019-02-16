defmodule Backend do
  def init(default_options) do
    IO.puts "initializing plugin"
    default_options
  end

  def call(conn, options) do
    IO.puts "calling plug"

    conn
    |> Plug.Conn.send_resp(200, "hello world")
  end

  @moduledoc """
  Documentation for Backend.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Backend.hello()
      :world

  """
  def hello do
    :world
  end
end
