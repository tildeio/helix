require 'spec_helper'

describe GameOfLife do
  describe '.parse' do
    context 'malformed inputs' do
      it 'should reject zero-sized boards' do
        expect(->() { GameOfLife.parse(0, 0, '') }).to raise_error(/cannot be 0/)
        expect(->() { GameOfLife.parse(0, 10, '') }).to raise_error(/cannot be 0/)
        expect(->() { GameOfLife.parse(10, 0, '') }).to raise_error(/cannot be 0/)
      end

      it 'should reject lines that are too long' do
        expect(->() { GameOfLife.parse(2, 2, '...') }).to raise_error(/line 1 is too long/)
      end

      it 'should reject too many lines' do
        expect(->() { GameOfLife.parse(2, 2, "\n\n\n") }).to raise_error(/too many lines/)
      end

      it 'should parse correctly' do
        pattern = <<~GAME
          ..*..**
          .......
          **..*..
        GAME

        game = GameOfLife.parse(7, 3, pattern)

        expect(game.to_s).to eq(pattern)
      end
    end
  end

  describe 'game rules' do
    context 'a live cell' do
      context 'with zero live neighbors' do
        it 'should die, as if by under population' do
          game = GameOfLife.parse 3, 3, <<~GAME
            ...
            .*.
            ...
          GAME

          game.advance!

          expect(game[1,1]).to eq(false)
        end
      end

      context 'with one live neighbor' do
        it 'should die, as if by under population' do
          game = GameOfLife.parse 3, 3, <<~GAME
            ..*
            .*.
            ...
          GAME

          game.advance!

          expect(game[1,1]).to eq(false)
        end
      end

      context 'with two live neighbors' do
        it 'should live on' do
          game = GameOfLife.parse 3, 3, <<~GAME
            ..*
            .**
            ...
          GAME

          game.advance!

          expect(game[1,1]).to eq(true)
        end
      end

      context 'with three live neighbors' do
        it 'should live on' do
          game = GameOfLife.parse 3, 3, <<~GAME
            ..*
            ***
            ...
          GAME

          game.advance!

          expect(game[1,1]).to eq(true)
        end
      end

      context 'with four live neighbors' do
        it 'should die, as if by overpopulation' do
          game = GameOfLife.parse 3, 3, <<~GAME
            ..*
            ***
            .*.
          GAME

          game.advance!

          expect(game[1,1]).to eq(false)
        end
      end
    end

    context 'a dead cell' do
      context 'with zero live neighbors' do
        it 'should stay dead' do
          game = GameOfLife.parse 3, 3, <<~GAME
            ...
            ...
            ...
          GAME

          game.advance!

          expect(game[1,1]).to eq(false)
        end
      end

      context 'with one live neighbor' do
        it 'should stay dead' do
          game = GameOfLife.parse 3, 3, <<~GAME
            ..*
            ...
            ...
          GAME

          game.advance!

          expect(game[1,1]).to eq(false)
        end
      end

      context 'with two live neighbors' do
        it 'should stay dead' do
          game = GameOfLife.parse 3, 3, <<~GAME
            ..*
            ..*
            ...
          GAME

          game.advance!

          expect(game[1,1]).to eq(false)
        end
      end

      context 'with three live neighbors' do
        it 'should become a live cell, as if by reproduction' do
          game = GameOfLife.parse 3, 3, <<~GAME
            ..*
            *.*
            ...
          GAME

          game.advance!

          expect(game[1,1]).to eq(true)
        end
      end

      context 'with four live neighbors' do
        it 'should stay dead' do
          game = GameOfLife.parse 3, 3, <<~GAME
            ..*
            *.*
            .*.
          GAME

          game.advance!

          expect(game[1,1]).to eq(false)
        end
      end
    end
  end

  describe 'known patterns' do
    shared_examples 'still life' do
      it 'should stay still' do
        game = GameOfLife.parse(width, height, pattern)

        expect(game.to_s).to eq(pattern)

        game.advance!

        expect(game.to_s).to eq(pattern)

        10.times { game.advance! }

        expect(game.to_s).to eq(pattern)
      end
    end

    shared_examples 'oscillator' do |period:|
      it 'should oscillate' do
        game = GameOfLife.parse(width, height, pattern)

        expect(game.to_s).to eq(pattern)

        game.advance!

        expect(game.to_s).to_not eq(pattern)

        (period - 1).times { game.advance! }

        expect(game.to_s).to eq(pattern)
      end
    end

    context 'Block' do
      let(:width) { 4 }
      let(:height) { 4 }
      let(:pattern) {
        <<~PATTERN
          ....
          .**.
          .**.
          ....
        PATTERN
      }

      it_should_behave_like 'still life'
    end

    context 'Beehive' do
      let(:width) { 6 }
      let(:height) { 5 }
      let(:pattern) {
        <<~PATTERN
          ......
          ..**..
          .*..*.
          ..**..
          ......
        PATTERN
      }

      it_should_behave_like 'still life'
    end

    context 'Loaf' do
      let(:width) { 6 }
      let(:height) { 6 }
      let(:pattern) {
        <<~PATTERN
          ......
          ..**..
          .*..*.
          ..*.*.
          ...*..
          ......
        PATTERN
      }

      it_should_behave_like 'still life'
    end

    context 'Boat' do
      let(:width) { 5 }
      let(:height) { 5 }
      let(:pattern) {
        <<~PATTERN
          .....
          .**..
          .*.*.
          ..*..
          .....
        PATTERN
      }

      it_should_behave_like 'still life'
    end

    context 'Tub' do
      let(:width) { 5 }
      let(:height) { 5 }
      let(:pattern) {
        <<~PATTERN
          .....
          ..*..
          .*.*.
          ..*..
          .....
        PATTERN
      }

      it_should_behave_like 'still life'
    end

    context 'Blinker' do
      let(:width) { 5 }
      let(:height) { 5 }
      let(:pattern) {
        <<~PATTERN
          .....
          ..*..
          ..*..
          ..*..
          .....
        PATTERN
      }

      it_should_behave_like 'oscillator', period: 2
    end

    context 'Toad' do
      let(:width) { 6 }
      let(:height) { 6 }
      let(:pattern) {
        <<~PATTERN
          ......
          ......
          ..***.
          .***..
          ......
          ......
        PATTERN
      }

      it_should_behave_like 'oscillator', period: 2
    end

    context 'Beacon' do
      let(:width) { 6 }
      let(:height) { 6 }
      let(:pattern) {
        <<~PATTERN
          ......
          .**...
          .**...
          ...**.
          ...**.
          ......
        PATTERN
      }

      it_should_behave_like 'oscillator', period: 2
    end

    context 'Pulsar' do
      let(:width) { 17 }
      let(:height) { 17 }
      let(:pattern) {
        <<~PATTERN
          .................
          .................
          ....***...***....
          .................
          ..*....*.*....*..
          ..*....*.*....*..
          ..*....*.*....*..
          ....***...***....
          .................
          ....***...***....
          ..*....*.*....*..
          ..*....*.*....*..
          ..*....*.*....*..
          .................
          ....***...***....
          .................
          .................
        PATTERN
      }

      it_should_behave_like 'oscillator', period: 3
    end

    context 'Pentadecathlon' do
      let(:width) { 11 }
      let(:height) { 18 }
      let(:pattern) {
        <<~PATTERN
          ...........
          ...........
          ...........
          ....***....
          ...*...*...
          ...*...*...
          ....***....
          ...........
          ...........
          ...........
          ...........
          ....***....
          ...*...*...
          ...*...*...
          ....***....
          ...........
          ...........
          ...........
        PATTERN
      }

      it_should_behave_like 'oscillator', period: 15
    end
  end
end

