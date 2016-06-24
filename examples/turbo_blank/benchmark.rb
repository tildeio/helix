require 'bundler/setup'
require 'benchmark'

ENV['IMPLEMENTATION'] = 'NONE'

require 'turbo_blank'

WARMUP_ITER = 10_000
BENCHMARK_ITER = 2_000_000

puts "Benchmarking at #{BENCHMARK_ITER} iterations..."

def scenario(title, str, expected)
  title = " #{title} ".inspect

  program = <<-RUBY_CODEZ
    str = #{str.inspect}

    puts
    puts #{title}.center(80, '-')
    puts

    # smoke test

    unless str.rails_4_2_blank? == #{expected}
      raise "Expected " + str.inspect + ".rails_4_2_blank? to be #{expected}"
    end

    unless str.rails_5_blank? == #{expected}
      raise "Expected " + str.inspect + ".rails_5_blank? to be #{expected}"
    end

    unless str.is_blank == #{expected}
      raise "Expected " + str.inspect + ".is_blank to be #{expected}"
    end

    # warmup
    i = 0

    while i < #{WARMUP_ITER}
      str.rails_4_2_blank?
      str.rails_5_blank?
      str.is_blank
      i += 1
    end

    # for realz
    GC.start

    rails_4_2_result = Benchmark.measure {
      i = 0

      while i < #{BENCHMARK_ITER}
        str.rails_4_2_blank?
        i += 1
      end
    }

    GC.start

    rails_5_result = Benchmark.measure {
      i = 0

      while i < #{BENCHMARK_ITER}
        str.rails_5_blank?
        i += 1
      end
    }

    GC.start

    rust_result = Benchmark.measure {
      i = 0

      while i < #{BENCHMARK_ITER}
        str.is_blank
        i += 1
      end
    }

    # Should I use real time or...?
    puts "Rails 4.2    " + rails_4_2_result.real.round(5).to_s.ljust(7,"0") + " sec"
    puts "Rails 5      " + rails_5_result.real.round(5).to_s.ljust(7,"0") + " sec (" + (#{BENCHMARK_ITER}/rails_5_result.real).round.to_s.rjust(8) + " ops/sec) - " + (rails_4_2_result.real / rails_5_result.real).round(2).to_s.rjust(5) + "x faster"
    puts "Rust         " + rust_result.real.round(5).to_s.ljust(7,"0") + " sec (" + (#{BENCHMARK_ITER}/rust_result.real).round.to_s.rjust(8) + " ops/sec) - " + (rails_4_2_result.real / rust_result.real).round(2).to_s.rjust(5) + "x faster"
  RUBY_CODEZ

  eval program
end

scenario("Empty string", "", true)
scenario("Single space", " ", true)
scenario("10 spaces", "         ", true)
scenario("Mixed", "\r\n\r\n  ", true)

scenario("Not blank", "this is a test", false)
scenario("Not blank (padded)", "   this is a test", false)
scenario("Not blank (long)",
"this is a longer test
this is a longer test
this is a longer test
this is a longer test
this is a longer test",
false)
scenario("Not blank (long, padded)",
"      this is a longer test
      this is a longer test
      this is a longer test
      this is a longer test
      this is a longer test",
false)
