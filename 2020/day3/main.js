const fs = require('fs')

const prepareData = (data) => {
  return data.split('\n').map(row => row.split(''))
}

const part1 = (data) => {
  let numTrees = data.map((row, index) => row[index*3 % row.length]).filter(result => result === '#').length;
  return numTrees
}


const part2 = (data) => {

  const getNumTrees = (slopeData, slopeRight) => {
    return slopeData.map((row, index) => row[index*slopeRight % row.length]).filter(result => result === '#').length;
  }

  let slopes = [[1,1], [3,1], [5,1], [7,1], [1,2]]
  let results = slopes.map(
    slope => getNumTrees(
              data.filter((row, index) => index % slope[1] === 0 ),
              slope[0]
             )
  ).reduce((init,val) => init*val, 1)
  console.log(results)
}

const main = (data) => {
  
  let preparedData = prepareData(data);

  let numTrees = part1(preparedData);
  console.log(numTrees)
  part2(preparedData);
}

fs.readFile('input.txt', 'utf8' , (err, data) => {
  if (err) {
    console.error(err)
    return
  }
  main(data);
})