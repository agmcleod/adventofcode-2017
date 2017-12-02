const fs = require('fs')
fs.readFile('1/input.txt', (err, text) => {
  const numbers = text.toString().split('')

  let sum = 0
  for (let i = 0; i < numbers.length; i++) {
    const num = parseInt(numbers[i], 10)
    if (parseInt(numbers[i + 1]) === num) {
      sum += num
    }
  }

  if (numbers[0] === numbers[numbers.length - 1]) {
    sum += parseInt(numbers[0], 10)
  }

  console.log(sum)

  let partTwoSum = 0
  let halfLen = numbers.length / 2
  for (let i = 0; i < numbers.length; i++) {
    const num = parseInt(numbers[i], 10)
    if (parseInt(numbers[(i + halfLen) % numbers.length]) === num) {
      partTwoSum += num
    }
  }

  console.log(partTwoSum)
})
