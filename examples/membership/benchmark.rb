require 'bundler/setup'
require 'benchmark'

ENV['IMPLEMENTATION'] = 'NONE'

require 'membership'

WARMUP_ITER = 10_000
BENCHMARK_ITER = 2_000_000

puts "Benchmarking at #{BENCHMARK_ITER} iterations..."

def scenario(title, haystack, needle, expected)
  title = " #{title} ".inspect
  haystack = "haystack_#{haystack}"
  needle = "needle_#{needle}"

  program = <<-RUBY_CODEZ
    haystack_empty   = []
    haystack_small   = [1,2,3]
    haystack_medium  = [1,2,3,4,5,6]
    haystack_large   = (1..15).to_a
    haystack_mega    = (1..100).to_a

    needle_empty     = []
    needle_tiny_1    = [1]
    needle_tiny_2    = [3]
    needle_tiny_3    = [6]
    needle_tiny_4    = [15]
    needle_tiny_5    = [50]
    needle_tiny_6    = [100]
    needle_tiny_7    = [101]
    needle_small_1   = [1,2,3]
    needle_small_2   = [2,3,4]
    needle_small_3   = [4,5,6]
    needle_small_4   = [5,6,7]
    needle_small_5   = [13,14,15]
    needle_small_6   = [14,15,16]
    needle_small_7   = [49,50,51]
    needle_small_8   = [98,99,100]
    needle_small_9   = [99,100,101]
    needle_medium_1  = [1,2,3,4,5,6]
    needle_medium_2  = [2,3,4,5,6,7]
    needle_medium_3  = [6,7,8,9,10,11]
    needle_medium_4  = [2,4,6,8,10,12]
    needle_medium_5  = [10,11,12,13,14,15]
    needle_medium_6  = [11,12,13,14,15,16]
    needle_medium_7  = [48,49,50,51,52,53]
    needle_medium_8  = [25,35,45,55,65,75]
    needle_medium_9  = [95,96,97,98,99,100]
    needle_medium_10 = [96,97,98,99,100,101]
    needle_large_1   = (1..15).to_a
    needle_large_2   = (2..16).to_a
    needle_large_3   = (43..57).to_a
    needle_large_4   = [15,20,25,30,35,40,45,50,55,60,65,70,75,80,85]
    needle_large_5   = (86..100).to_a
    needle_large_6   = (87..101).to_a
    needle_mega_1    = (1..100).to_a
    needle_mega_2    = (2..101).to_a

    puts
    puts #{title}.center(80, '-')
    puts

    # smoke test

    unless #{haystack}.naive_superset_of?(#{needle}) == #{expected}
      raise "Expected #{haystack}.naive_superset_of?(#{needle}) to be #{expected}"
    end

    unless #{haystack}.fast_superset_of?(#{needle}) == #{expected}
      raise "Expected #{haystack}.fast_superset_of?(#{needle}) to be #{expected}"
    end

    unless #{haystack}.is_superset_of(#{needle}) == #{expected}
      raise "Expected #{haystack}.is_superset_of(#{needle}) to be #{expected}"
    end

    # warmup
    i = 0

    while i < #{WARMUP_ITER}
      #{haystack}.naive_superset_of?(#{needle})
      #{haystack}.fast_superset_of?(#{needle})
      #{haystack}.is_superset_of(#{needle})
      i += 1
    end

    # for realz
    GC.start

    naive_result = Benchmark.measure {
      i = 0

      while i < #{BENCHMARK_ITER}
        #{haystack}.naive_superset_of?(#{needle})
        i += 1
      end
    }

    GC.start

    fast_result = Benchmark.measure {
      i = 0

      while i < #{BENCHMARK_ITER}
        #{haystack}.fast_superset_of?(#{needle})
        i += 1
      end
    }

    GC.start

    rust_result = Benchmark.measure {
      i = 0

      while i < #{BENCHMARK_ITER}
        #{haystack}.is_superset_of(#{needle})
        i += 1
      end
    }

    # Should I use real time or...?
    puts "Ruby (Naive)    " + naive_result.real.round(5).to_s.ljust(7,"0") + " sec"
    puts "Ruby (Fast)     " + fast_result.real.round(5).to_s.ljust(7,"0") + " sec (" + (#{BENCHMARK_ITER}/fast_result.real).round.to_s.rjust(8) + " ops/sec) - " + (naive_result.real / fast_result.real).round(2).to_s.rjust(5) + "x faster"
    puts "Rust            " + rust_result.real.round(5).to_s.ljust(7,"0") + " sec (" + (#{BENCHMARK_ITER}/rust_result.real).round.to_s.rjust(8) + " ops/sec) - " + (naive_result.real / rust_result.real).round(2).to_s.rjust(5) + "x faster"
  RUBY_CODEZ

  eval program
end

scenario("Empty haystack (exact match)", "empty", "empty", true)
scenario("Empty haystack (not found)", "empty", "tiny_1", false)

scenario("Small haystack, tiny needle (found at front)", "small", "tiny_1", true)
scenario("Small haystack, tiny needle (found at back)", "small", "tiny_2", true)
scenario("Small haystack, tiny needle (not found)", "small", "tiny_3", false)

scenario("Small haystack, small needle (exact match)", "small", "small_1", true)
scenario("Small haystack, small needle (not found)", "small", "small_2", false)

scenario("Medium haystack, tiny needle (found at front)", "medium", "tiny_1", true)
scenario("Medium haystack, tiny needle (found at back)", "medium", "tiny_3", true)
scenario("Medium haystack, tiny needle (not found)", "medium", "tiny_4", false)

scenario("Medium haystack, small needle (found at front)", "medium", "small_1", true)
scenario("Medium haystack, small needle (found at back)", "medium", "small_3", true)
scenario("Medium haystack, small needle (not found)", "medium", "small_4", false)

scenario("Medium haystack, medium needle (exact match)", "medium", "medium_1", true)
scenario("Medium haystack, medium needle (not found)", "medium", "medium_2", false)

scenario("Large haystack, tiny needle (found at front)", "large", "tiny_1", true)
scenario("Large haystack, tiny needle (found at back)", "large", "tiny_4", true)
scenario("Large haystack, tiny needle (not found)", "large", "tiny_5", false)

scenario("Large haystack, small needle (found at front)", "large", "small_1", true)
scenario("Large haystack, small needle (found at back)", "large", "small_5", true)
scenario("Large haystack, small needle (not found)", "large", "small_6", false)

scenario("Large haystack, medium needle (found at front)", "large", "medium_1", true)
scenario("Large haystack, medium needle (found in middle)", "large", "medium_3", true)
scenario("Large haystack, medium needle (spread out)", "large", "medium_4", true)
scenario("Large haystack, medium needle (found at back)", "large", "medium_5", true)
scenario("Large haystack, medium needle (not found)", "large", "medium_6", false)

scenario("Large haystack, large needle (exact match)", "large", "large_1", true)
scenario("Large haystack, large needle (not found)", "large", "large_2", false)

scenario("Mega haystack, tiny needle (found at front)", "mega", "tiny_1", true)
scenario("Mega haystack, tiny needle (found in middle)", "mega", "tiny_5", true)
scenario("Mega haystack, tiny needle (found at back)", "mega", "tiny_6", true)
scenario("Mega haystack, tiny needle (not found)", "mega", "tiny_7", false)

scenario("Mega haystack, small needle (found at front)", "mega", "small_1", true)
scenario("Mega haystack, small needle (found in middle)", "mega", "small_7", true)
scenario("Mega haystack, small needle (found at back)", "mega", "small_8", true)
scenario("Mega haystack, small needle (not found)", "mega", "small_9", false)

scenario("Mega haystack, medium needle (found at front)", "mega", "medium_1", true)
scenario("Mega haystack, medium needle (found in middle)", "mega", "medium_7", true)
scenario("Mega haystack, medium needle (spread out)", "mega", "medium_8", true)
scenario("Mega haystack, medium needle (found at back)", "mega", "medium_9", true)
scenario("Mega haystack, medium needle (not found)", "mega", "medium_10", false)

scenario("Mega haystack, large needle (found at front)", "mega", "large_1", true)
scenario("Mega haystack, large needle (found in middle)", "mega", "large_3", true)
scenario("Mega haystack, large needle (spread out)", "mega", "large_4", true)
scenario("Mega haystack, large needle (found at back)", "mega", "large_5", true)
scenario("Mega haystack, large needle (not found)", "mega", "large_6", false)

scenario("Mega haystack, mega needle (exact match)", "mega", "mega_1", true)
scenario("Mega haystack, mega needle (not found)", "mega", "mega_2", false)
