const fs = require('fs')


const prepareData = (data) => {
  return data.split('\n').map(row => row.split(''))
}

const halve= (interval, remainingCode) => {
  if (remainingCode.length === 0) {
    return interval
  }

  let code = remainingCode.pop();

  return code === "F" || code === "L" ? 
    halve([interval[0], interval[1]-((interval[1]-interval[0]+1)/2)], remainingCode) 
    :
    halve([interval[0]+((interval[1]-interval[0]+1)/2), interval[1]], remainingCode)
  
}


const part1And2 = (data) => {
  let result = data.map(
    seat => ({ 
      row: halve([0,127], seat.slice(0,7).reverse())[0],
      column: halve([0,7], seat.slice(7,10).reverse())[0],
    })
  ).map(seat => seat.row*8+seat.column).sort((a,b) => a-b)

  console.log(`Answer part 1: ${result[result.length-1]}`)
  let v = result.sort((a,b) => a-b).filter((id,index) => (result[index+1]-id) === 2)
  console.log(`Answer part 2: ${v[0]+1}`)
}


const main = (data) => {
  
  let preparedData = prepareData(data);
  part1And2(preparedData)
}

fs.readFile('input.txt', 'utf8' , (err, data) => {
  if (err) {
    console.error(err)
    return
  }
  main(data);
})
