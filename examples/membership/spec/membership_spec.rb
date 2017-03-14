require 'spec_helper'

describe Array do
  describe '#is_superset' do
    it 'should return true if the needle is empty' do
      expect([1,2,3,4,5]).to be_superset_of([])
    end

    it 'should return false if the source is empty' do
      expect([]).to_not be_superset_of([1,2,3,4,5])
    end

    it 'should return true if both the source and needle are empty' do
      expect([]).to be_superset_of([])
    end

    it 'should handle strings' do
      pending if ENV['IMPLEMENTATION'] == 'RUST'
      expect(['a', 'b', 'c']).to be_superset_of(['a', 'b'])
    end

    it 'should return true if the needle is fully contained in the source' do
      expect([1,2,3,4,5]).to be_superset_of([1])
      expect([1,2,3,4,5]).to be_superset_of([2])
      expect([1,2,3,4,5]).to be_superset_of([5])

      expect([1,2,3,4,5]).to be_superset_of([1,2])
      expect([1,2,3,4,5]).to be_superset_of([1,4])
      expect([1,2,3,4,5]).to be_superset_of([2,5])
      expect([1,2,3,4,5]).to be_superset_of([4,5])

      expect([1,2,3,4,5]).to be_superset_of([1,2,3])
      expect([1,2,3,4,5]).to be_superset_of([2,3,5])
      expect([1,2,3,4,5]).to be_superset_of([1,4,5])

      expect([1,2,3,4,5]).to be_superset_of([1,2,3,4])
      expect([1,2,3,4,5]).to be_superset_of([1,2,4,5])
      expect([1,2,3,4,5]).to be_superset_of([2,3,4,5])

      expect([1,2,3,4,5]).to be_superset_of([1,2,3,4,5])
    end

    it 'should return false if the needle is not fully contained in the source' do
      expect([1,2,3,4,5]).to_not be_superset_of([0])
      expect([1,2,3,4,5]).to_not be_superset_of([6])

      expect([1,2,3,4,5]).to_not be_superset_of([0,1])
      expect([1,2,3,4,5]).to_not be_superset_of([2,6])
      expect([1,2,3,4,5]).to_not be_superset_of([6,7])

      expect([1,2,3,4,5]).to_not be_superset_of([1,2,6])
      expect([1,2,3,4,5]).to_not be_superset_of([0,2,3])
      expect([1,2,3,4,5]).to_not be_superset_of([7,8,9])

      expect([1,2,3,4,5]).to_not be_superset_of([0,1,2,3])
      expect([1,2,3,4,5]).to_not be_superset_of([3,4,5,6])
      expect([1,2,3,4,5]).to_not be_superset_of([6,7,8,9])

      expect([1,2,3,4,5]).to_not be_superset_of([0,1,2,3,4])
      expect([1,2,3,4,5]).to_not be_superset_of([2,3,4,5,6])
      expect([1,2,3,4,5]).to_not be_superset_of([6,7,8,9,10])
    end
  end
end
