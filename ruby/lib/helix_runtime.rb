require "helix_runtime/version"
require "helix_runtime/native"
require "helix_runtime/platform"
require 'helix_runtime/project'

module HelixRuntime
  IS_WINDOWS = RUBY_PLATFORM =~ /mingw/

  class MissingDllError < RuntimeError; end

  @@dll_filename = "helix-runtime-#{VERSION.gsub('.', '-')}.#{Platform.libext}"
  @@dll_path_searched = false

  def self.dll_filename
    @@dll_filename
  end

  def self.dll_path
    return nil unless IS_WINDOWS
    return @@dll_path if @@dll_path_searched

    dir = ENV['PATH'].split(';').find do |dir|
      File.exist?(File.expand_path("#{dir}/#{dll_filename}", __FILE__))
    end

    @@dll_path_searched = true
    @@dll_path = dir ? File.join(dir, dll_filename) : nil
  end

  def self.ensure_dll!
    if IS_WINDOWS
      unless dll_path
        raise MissingDllError, "Unable to find #{dll_filename} in $PATH."
      end
    else
      # No-op
    end
  end

  def self.copy_dll
    if IS_WINDOWS
      so_path = File.expand_path("../native.#{Platform.dlext}", __FILE__)
      raise "Unable to find native bundle at #{so_path}" unless File.exists?(so_path)

      bindir = RbConfig::CONFIG['bindir']
      raise "Unable to determine Ruby bindir" unless bindir

      FileUtils.cp so_path, File.join(bindir, dll_filename)
    else
      # No-op
    end
  end
end
