local quit_command = hrtor.command.new {
  action = function()
    hrtor.api.quit();
  end,
  trigger = { 'q' },
}
