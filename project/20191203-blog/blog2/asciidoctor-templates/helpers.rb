module SlimHelpers
  def capture_to_local(var, &block)
    set_var = block.binding.eval("lambda {|x| #{var} = x }")
    # In Rails we have to use capture!
    # If we are using Slim without a framework (Plain Tilt),
    # you can just yield to get the captured block.
    set_var.call(defined?(::Rails) ? capture(&block) : yield)
    nil
  end
end

include SlimHelpers
