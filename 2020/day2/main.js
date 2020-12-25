const fs = require('fs')

const prepareData = (data) => {

  const prepareRow = (row) => {
    let ruleChar = row.split(': ')[0].split(' ')[1];
    let interval = row.split(': ')[0].split(' ')[0].split('-').map(n => parseInt(n));
    let password = row.split(': ')[1];
    return {ruleChar, interval, password}
  }
  
  return data.split('\n').map(row => prepareRow(row));
}

const inInterval = (interval, occurrences) => occurrences >= interval[0] && occurrences <= interval[1]

const part1 = (data) => {
  const countOccurrences = (c, password) => password.split(c).length - 1;
  let result = data.map(
    row => ({...row, occurrences: countOccurrences(row.ruleChar, row.password)})
  ).filter(
    row => inInterval(row.interval, row.occurrences)
  ).length;

  console.log(result);
}

const part2 = (data) => {
  const isPresent = (c, interval, password) => interval.map(i => password[i-1] === c)
  const isValid = (present) => present[0] ^ present[1];

  let result = data.map(
    row => ({...row, present: isPresent(row.ruleChar, row.interval, row.password)})
  ).filter(row => isValid(row.present)).length;
  console.log(result);
}

const main = (data) => {
  
  let preparedData = prepareData(data);

  part1(preparedData);
  part2(preparedData);
}

fs.readFile('input.txt', 'utf8' , (err, data) => {
  if (err) {
    console.error(err)
    return
  }
  main(data);
})