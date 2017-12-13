const fs = require('fs')
const _ = require('lodash')

function findGroupSizeFor (scannedList, nodes, key) {
  const heap = [key]

  while (heap.length > 0) {
    const val = heap.pop()
    scannedList[val] = true
    nodes[val].forEach((neighbour) => {
      if (!scannedList[neighbour]) {
        heap.push(neighbour)
      }
    })
  }

  return Object.keys(scannedList).length
}

fs.readFile('12/input.txt', (err, data) => {
  const text = data.toString()
  const nodes = {}
  text.split(/\n/).forEach((line) => {
    if (!line) {
      return
    }
    const parts = line.split(' <-> ')
    nodes[parseInt(parts[0], 10)] = parts[1].split(', ').map((n) => parseInt(n, 10))
  })

  const scannedList = {}
  console.log('part1', findGroupSizeFor(scannedList, nodes, 0))

  const scannedListP2 = {}
  let groupCount = 0
  _.forEach(nodes, (_, node) => {
    if (!scannedListP2[node]) {
      findGroupSizeFor(scannedListP2, nodes, node)
      groupCount++
    }
  })
  console.log('part2', groupCount)
})