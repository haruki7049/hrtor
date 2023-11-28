-- sample config file for hrtor:v0.2.0

hrtor.commands = {
  print = function()
    hrtor.api.display_buffer();
  end,
  exit = function()
    hrtor.api.quit();
  end,
}
