hrtor.register_command(
  hrtor.command.new {
    action = function()
      hrtor.api.echo('FooFoo!!')
      hrtor.api.quit()
    end,
    trigger = {
      'q'
    },
  }
)

hrtor.register_command(
  hrtor.command.new {
    action = function()
      hrtor.api.echo('mama!!')
    end,
    trigger = {
      'papa'
    },
  }
)
