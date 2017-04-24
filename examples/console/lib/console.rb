require 'helix_runtime'
require 'console/native'

class RubyConsole
  def noop
  end

  def log(string)
    puts(string)
  end

  def inspect
    puts super
  end

  def hello
    log("hello")
  end

  def hello_str
    "hello"
  end

  def loglog(string1, string2)
    puts "#{string1} #{string2}"
  end

  def log_if(string, condition)
    log(string) if condition
  end

  def colorize(string)
    "\x1B[0;31;49m#{string}\x1B[0m"
  end

  def is_red(string)
    string.start_with?("\x1B[0;31;49m") && string.end_with?("\x1B[0m")
  end

  def freak_out
    raise "Aaaaahhhhh!!!!!"
  end
end

case ENV["IMPLEMENTATION"]
when "RUST"
  Console = RustConsole
when "RUBY"
  Console = RubyConsole
when "NONE"
else
  puts "\nPlease specify an IMPLEMENTATION: RUST, RUBY or NONE"
  exit!
end
