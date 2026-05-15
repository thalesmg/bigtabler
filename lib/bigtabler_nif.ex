defmodule :bigtabler_nif do
  use Rustler, otp_app: :bigtabler, crate: "bigtabler_nif"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)

  def mutate_rows_request(_req), do: :erlang.nif_error(:nif_not_loaded)

  def mutate_rows_response(_resp), do: :erlang.nif_error(:nif_not_loaded)
end
