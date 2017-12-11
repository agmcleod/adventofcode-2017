/**
 * Strategy for solving this one was to do it in phases. Remove the cancelled characters first, since they always negate the one after. Done via regex
 * The second phase was to parse through and find the indexes of the string's garbage characters. Finding the left, then the right after, substring it out, and continue until none remain
 * The third phase was then counting depth level up & down as one encounters left & right brackets.
 *
 * Part 2 was simple enough to add on due to how i stripped out the garbage. Just sum the indexes, subtracting 1 to account for the bracket characters themselves.
 */

const fs = require('fs')

function removeGarbage (text) {
  let hasGargbage = true
  let charactersWithinGarbageCount = 0
  while (hasGargbage) {
    const nextGarbage = text.indexOf('<')
    let endGarbageIndex = -1
    for (let i = nextGarbage + 1; i < text.length; i++) {
      if (text[i] === '>') {
        endGarbageIndex = i
        break
      }
    }

    // {<ab>}

    if (endGarbageIndex > nextGarbage) {
      let newText = text.substring(0, nextGarbage)
      newText += text.substring(endGarbageIndex + 1)
      text = newText
      charactersWithinGarbageCount += (endGarbageIndex - nextGarbage - 1)
    }

    const leftAngleIndex = text.indexOf('<')
    hasGargbage = leftAngleIndex !== -1 && text.indexOf('>') > leftAngleIndex
  }

  console.log('garbage count', charactersWithinGarbageCount)

  return text
}

fs.readFile('9/input.txt', (err, text) => {
  text = text.toString().replace(/\!./g, '')
  text = removeGarbage(text)

  text = text.replace(/\,/g, '')
  const brackets = text.split('')
  let scoreCounter = 0
  let depthCounter = 0
  brackets.forEach((ch, i) => {
    if (ch === '{') {
      depthCounter++
    } else if (ch === '}') {
      scoreCounter += depthCounter
      depthCounter--
    }
  })

  console.log(scoreCounter)
})
