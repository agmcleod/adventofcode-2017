data = IO.read('input.txt')

checksum = 0
divisors = 0
data.split(/\n/).each do |line|
  sorted = line.split(/\t/).map{|v| v.to_i }.sort
  checksum += sorted[sorted.size - 1] - sorted[0]

  large_first = sorted.reverse
  large_first.each do |h|
    sorted.each do |l|
      if h != l && h % l == 0
        divisors += h / l
      end
    end
  end
end

puts checksum
puts divisors