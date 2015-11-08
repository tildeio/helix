require 'spec_helper'
require 'fileutils'

module TurboRuby::FileSystem
  describe Tree do
    let(:turboruby) { Tree.new(File.expand_path('../..', __FILE__)) }
    let(:lib) { Tree.new(File.expand_path('../../lib', __FILE__)) }
    let(:spec) { Tree.new(File.expand_path('../../spec', __FILE__)) }

    it 'can query files' do
      expect(turboruby.file?('Gemfile')).to be_truthy
      expect(turboruby.file?('lib')).to be_falsy
      expect(turboruby.file?('zomg.rb')).to be_falsy

      expect(turboruby.file?('lib/turbo_ruby/file_system.rb')).to be_truthy
      expect(turboruby.file?('lib/turbo_ruby')).to be_falsy
      expect(turboruby.file?('lib/turbo_ruby/zomg.rb')).to be_falsy
    end

    it 'can query directories' do
      expect(turboruby.directory?('Gemfile')).to be_falsy
      expect(turboruby.directory?('lib')).to be_truthy
      expect(turboruby.directory?('zomg.rb')).to be_falsy

      expect(turboruby.directory?('lib/turbo_ruby/file_system.rb')).to be_falsy
      expect(turboruby.directory?('lib/turbo_ruby')).to be_truthy
      expect(turboruby.directory?('lib/turbo_ruby/zomg.rb')).to be_falsy
    end

    it 'can read files' do
      expect(turboruby.read('turboruby.gemspec')).to include('TurboRuby::VERSION')
      expect(turboruby.read('lib/turbo_ruby/file_system.rb')).to eq(lib.read('turbo_ruby/file_system.rb'))
      expect(turboruby.read('spec/fixtures/fast_blank.trb')).to eq(spec.read('fixtures/fast_blank.trb'))
    end

    it 'raises when reading non-existent files' do
      expect { turboruby.read('zomg') }.to raise_error(Errno::ENOENT, /No such file or directory/)
    end

    it 'raises when reading directories' do
      expect { turboruby.read('lib') }.to raise_error(Errno::EISDIR, /Is a directory/)
    end

    it 'can write files' do
      Dir.mktmpdir do |dir|
        tree = Tree.new(dir)

        tree.write('lol', 'zomg')
        expect(File.read(File.expand_path('lol', dir))).to eq('zomg')

        tree.write('very/deeply/nested/file', 'wow')
        expect(File.read(File.expand_path('very/deeply/nested/file', dir))).to eq('wow')

        tree.write('very/file') { |f| f.print 'such content' }
        expect(File.read(File.expand_path('very/file', dir))).to eq('such content')
      end
    end

    it 'overwrites existing files' do
      Dir.mktmpdir do |dir|
        File.write(File.expand_path('lol', dir), 'zomg')

        tree = Tree.new(dir)

        tree.write('lol', 'wow')
        expect(File.read(File.expand_path('lol', dir))).to eq('wow')
      end
    end

    it 'raises when writing to directories' do
      Dir.mktmpdir do |dir|
        FileUtils.mkdir(File.expand_path('lol', dir))

        tree = Tree.new(dir)

        expect { tree.write('lol', 'wow') }.to raise_error(Errno::EISDIR, /Is a directory/)
      end
    end

    it 'can mkdir_p' do
      Dir.mktmpdir do |dir|
        FileUtils.mkdir_p(File.expand_path('a/b/c/d/e', dir))

        tree = Tree.new(dir)

        tree.mkdir_p('very')
        expect(File.directory?(File.expand_path('very', dir))).to be_truthy

        tree.mkdir_p('very/deeply/nested')
        expect(File.directory?(File.expand_path('very/deeply/nested', dir))).to be_truthy

        tree.mkdir_p('a/b/c')
        tree.mkdir_p('a/b/c/d/e')
      end
    end

    it 'can glob' do
      expect(turboruby.glob('*')).to include('lib', 'Gemfile', 'turboruby.gemspec')
      expect(turboruby.glob('*')).not_to include('fixtures', 'turboruby.rb', 'lib/turboruby.rb')

      expect(turboruby.glob('*.gemspec')).to eq(['turboruby.gemspec'])
      expect(turboruby.glob('lib/*.rb')).to eq(['lib/turboruby.rb'])

      expect(turboruby.glob('lib/**/*.rb')).to include('lib/turbo_ruby/parser.rb', 'lib/turbo_ruby/file_system.rb')
      expect(lib.glob('**/*.rb')).to include('turbo_ruby/parser.rb', 'turbo_ruby/file_system.rb')

      expect(turboruby.glob('spec/**/*_spec.rb')).to include('spec/parser_spec.rb', 'spec/file_system_spec.rb')
      expect(spec.glob('*_spec.rb')).to include('parser_spec.rb', 'file_system_spec.rb')

      expect(spec.glob('*_spec.rb')).not_to include('spec_helper.rb')
      expect(spec.glob('fixtures/*.rb')).to be_empty

      found = []
      turboruby.glob('Gemfile*') { |filename| found << filename }

      expect(found).to eq(['Gemfile', 'Gemfile.lock'])
    end

    it 'is equal to another `Tree` with the same root' do
      expect(spec).to eq(Tree.new(File.expand_path('..', __FILE__)))
      expect(spec).to_not eq(lib)
    end

    it 'is equal to its hash representation' do
      Dir.mktmpdir do |dir|
        tree = Tree.new(dir)

        expect(tree).to eq({})

        FileUtils.mkdir_p(File.expand_path('a/b/c/d/e', dir))

        FileUtils.touch(File.expand_path('boom', dir))
        File.write(File.expand_path('a/zomg', dir), 'lol')
        File.write(File.expand_path('a/b/c/wow', dir), 'such file')

        expect(tree).to_not eq({})
        expect(tree).to eq({
          a: {
            b: {
              c: {
                d: {
                  e: {}
                },
                wow: 'such file'
              }
            },
            zomg: 'lol'
          },
          boom: ''
        })
      end

      expect(lib).to_not eq(spec)
    end
  end

  describe InMemoryTree do
    let(:turboruby) { InMemoryTree.from_root(File.expand_path('../..', __FILE__)) }
    let(:lib) { InMemoryTree.from_root(File.expand_path('../../lib', __FILE__)) }
    let(:spec) { InMemoryTree.from_root(File.expand_path('../../spec', __FILE__)) }

    it 'can query files' do
      expect(turboruby.file?('Gemfile')).to be_truthy
      expect(turboruby.file?('lib')).to be_falsy
      expect(turboruby.file?('zomg.rb')).to be_falsy

      expect(turboruby.file?('lib/turbo_ruby/file_system.rb')).to be_truthy
      expect(turboruby.file?('lib/turbo_ruby')).to be_falsy
      expect(turboruby.file?('lib/turbo_ruby/zomg.rb')).to be_falsy
    end

    it 'can query directories' do
      expect(turboruby.directory?('Gemfile')).to be_falsy
      expect(turboruby.directory?('lib')).to be_truthy
      expect(turboruby.directory?('zomg.rb')).to be_falsy

      expect(turboruby.directory?('lib/turbo_ruby/file_system.rb')).to be_falsy
      expect(turboruby.directory?('lib/turbo_ruby')).to be_truthy
      expect(turboruby.directory?('lib/turbo_ruby/zomg.rb')).to be_falsy
    end

    it 'can read files' do
      expect(turboruby.read('turboruby.gemspec')).to include('TurboRuby::VERSION')
      expect(turboruby.read('lib/turbo_ruby/file_system.rb')).to eq(lib.read('turbo_ruby/file_system.rb'))
      expect(turboruby.read('spec/fixtures/fast_blank.trb')).to eq(spec.read('fixtures/fast_blank.trb'))
    end

    it 'raises when reading non-existent files' do
      expect { turboruby.read('zomg') }.to raise_error(Errno::ENOENT, /No such file or directory/)
    end

    it 'raises when reading directories' do
      expect { turboruby.read('lib') }.to raise_error(Errno::EISDIR, /Is a directory/)
    end

    it 'can write files' do
      Dir.mktmpdir do |dir|
        tree = InMemoryTree.from_root(dir)

        tree.write('lol', 'zomg')
        expect(tree.read('lol')).to eq('zomg')
        expect(File.file?(File.expand_path('lol', dir))).to be_falsy

        tree.write('very/deeply/nested/file', 'wow')
        expect(tree.read('very/deeply/nested/file')).to eq('wow')
        expect(File.file?(File.expand_path('very/deeply/nested/file', dir))).to be_falsy

        tree.write('very/file') { |f| f.print 'such content' }
        expect(tree.read('very/file')).to eq('such content')
        expect(File.file?(File.expand_path('very/file', dir))).to be_falsy
      end
    end

    it 'overwrites existing files' do
      Dir.mktmpdir do |dir|
        File.write(File.expand_path('lol', dir), 'zomg')

        tree = InMemoryTree.from_root(dir)

        tree.write('lol', 'wow')
        expect(tree.read('lol')).to eq('wow')
        expect(File.read(File.expand_path('lol', dir))).to eq('zomg')
      end
    end

    it 'raises when writing to directories' do
      Dir.mktmpdir do |dir|
        FileUtils.mkdir(File.expand_path('lol', dir))

        tree = InMemoryTree.from_root(dir)

        expect { tree.write('lol', 'wow') }.to raise_error(Errno::EISDIR, /Is a directory/)
      end
    end

    it 'can mkdir_p' do
      Dir.mktmpdir do |dir|
        FileUtils.mkdir_p(File.expand_path('a/b/c/d/e', dir))

        tree = InMemoryTree.from_root(dir)

        tree.mkdir_p('very')
        expect(tree.directory?('very')).to be_truthy
        expect(File.directory?(File.expand_path('very', dir))).to be_falsy

        tree.mkdir_p('very/deeply/nested')
        expect(tree.directory?('very/deeply/nested')).to be_truthy
        expect(File.directory?(File.expand_path('very/deeply/nested', dir))).to be_falsy

        tree.mkdir_p('a/b/c')
        tree.mkdir_p('a/b/c/d/e')
      end
    end

    it 'can glob' do
      expect(turboruby.glob('*')).to include('lib', 'Gemfile', 'turboruby.gemspec')
      expect(turboruby.glob('*')).not_to include('fixtures', 'turboruby.rb', 'lib/turboruby.rb')

      expect(turboruby.glob('*.gemspec')).to eq(['turboruby.gemspec'])
      expect(turboruby.glob('lib/*.rb')).to eq(['lib/turboruby.rb'])

      expect(turboruby.glob('lib/**/*.rb')).to include('lib/turbo_ruby/parser.rb', 'lib/turbo_ruby/file_system.rb')
      expect(lib.glob('**/*.rb')).to include('turbo_ruby/parser.rb', 'turbo_ruby/file_system.rb')

      expect(turboruby.glob('spec/**/*_spec.rb')).to include('spec/parser_spec.rb', 'spec/file_system_spec.rb')
      expect(spec.glob('*_spec.rb')).to include('parser_spec.rb', 'file_system_spec.rb')

      expect(spec.glob('*_spec.rb')).not_to include('spec_helper.rb')
      expect(spec.glob('fixtures/*.rb')).to be_empty

      found = []
      turboruby.glob('Gemfile*') { |filename| found << filename }

      expect(found).to eq(['Gemfile', 'Gemfile.lock'])
    end

    it 'is equal to another `Tree` with the same content' do
      fs_turboruby = Tree.new(File.expand_path('../..', __FILE__))
      fs_lib = Tree.new(File.expand_path('../../lib', __FILE__))
      fs_spec = Tree.new(File.expand_path('../../spec', __FILE__))

      expect(turboruby).to eq(fs_turboruby)
      expect(fs_turboruby).to eq(turboruby)

      expect(lib).to eq(fs_lib)
      expect(fs_lib).to eq(lib)

      expect(spec).to eq(fs_spec)
      expect(fs_spec).to eq(spec)
    end

    it 'is equal to another `InMemoryTree` with the same content' do
      tree1 = InMemoryTree.new({
        omg: "lol",
        very: {
          deeply: {
            nested: "wow"
          }
        }
      })

      tree2 = InMemoryTree.new({
        very: {
          deeply: {
            nested: "wow"
          }
        },
        omg: "lol"
      })

      expect(tree1).to eq(tree2)
      expect(tree2).to eq(tree1)

      expect(tree1).to_not eq(InMemoryTree.new)
      expect(InMemoryTree.new).to_not eq(tree1)
    end

    it 'is equal to its hash representation' do
      tree = InMemoryTree.new({
        omg: "lol",
        very: {
          deeply: {
            nested: "wow"
          }
        }
      })

      expect(tree).to eq({
        omg: "lol",
        very: {
          deeply: {
            nested: "wow"
          }
        }
      })

      expect(tree).to_not eq({})
    end
  end
end