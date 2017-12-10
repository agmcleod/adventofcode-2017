input = IO.read('8/input.txt')

registers = {}

def get_register_value(key, registers)
  if key == key.to_i.to_s
    return key.to_i
  end
  registers[key] || 0
end

def passes_condition(condition, registers)
  lh, op, rh = condition.split(' ')
  lh = get_register_value(lh, registers)
  rh = get_register_value(rh, registers)
  case op
  when '>='
    lh >= rh
  when '<='
    lh <= rh
  when '=='
    lh == rh
  when '>'
    lh > rh
  when '<'
    lh < rh
  when '!='
    lh != rh
  else
    raise "Could not parse operator for condition check: '#{op}'"
  end
end

def apply_instruction(value, op, amount)
  case op
  when 'inc'
    value + amount
  when 'dec'
    value - amount
  else
    raise "Could not apply instruction for operator: '#{op}'"
  end
end

highest_value = 0
input.split(/\n/).each do |line|
  pieces = line.split(' if ')
  if passes_condition pieces[1], registers
    register, op, amount = pieces[0].split(' ')
    registers[register] = apply_instruction get_register_value(register, registers), op, amount.to_i
    highest_value = [highest_value, registers[register]].max
  end
end

ans = 0
registers.each do |register, value|
  ans = [ans, value].max
end

puts ans
puts highest_value