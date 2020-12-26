const fs = require('fs')


const prepareData = (data) => {
  return data.split('\n\n').map(group => group.split('\n').map(person => person.split('')))
}

const part1 = (data) => {

  let result = data.map(group => {
    let groupResult = {};
    group.map(person => person.map(vote => vote in groupResult ? groupResult[vote] += 1 : groupResult[vote] = 1))
    return {
      numPersons: group.length, 
      result: groupResult
    };
  })

  console.log("Part 1:")
  console.log(result.map(group => Object.keys(group.result).length).reduce((a,b) => a+b, 0))

  console.log("Part 2:")
  let result2 = result.map(group => Object.keys(group.result).filter(key => group.result[key] === group.numPersons)).map(group => group.length).reduce((a,b) => a+b,0)
  console.log(result2)
}


const main = (data) => {
  
  let preparedData = prepareData(data);
  part1(preparedData)
}

fs.readFile('input.txt', 'utf8' , (err, data) => {
  if (err) {
    console.error(err)
    return
  }
  main(data);
})
