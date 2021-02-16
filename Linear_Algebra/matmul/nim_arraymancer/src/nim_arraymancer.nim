# This is just an example to get you started. A typical binary package
# uses this file as the main entry point of the application.

import arraymancer, random, sequtils, os, strutils

let cmd = commandLineParams()
let row = parseInt(cmd[0])
let col = parseInt(cmd[1])

proc mm: float64 =
  let m = newSeqWith(row*col, rand(1.0)).toTensor.reshape(row, col)
  let n = newSeqWith(row*col, rand(1.0)).toTensor.reshape(row, col)
  let res = m * n
  return res[int(row/2), int(col/2)]


when isMainModule:
  echo mm()
