const fs = require('fs')

const util = require('util')

const logy = (msg) => {
  console.log(util.inspect(msg, {showHidden: false, depth: null}))
}

const prepareData = (data) => {
  let preparedData = data.split('\n').map(expense => parseInt(expense));
  return preparedData;
}

const part1 = (data) => {
  console.log("Part 1:")
  let result = data.map(expense => (
    {
      expense: expense, 
      foundMatch: data.filter(expense2 => expense+expense2 === 2020)
    })
  ).filter(expense => expense.foundMatch.length > 0).map(f => f.expense*f.foundMatch[0]);

  console.log(result)
}

const part2 = (data) => {

  const hasMatch = (expense) => {
    return {expense: expense.expense, foundMatch:expense.expense2.filter(f => f.foundMatch.length > 0)}
  }

  console.log("Part 2:")
  let result = data.map(expense => (
  {
      expense: expense, 
      expense2: data.map(
        expense2 => ({
          expense2:expense2, 
          foundMatch: data.filter(expense3 => expense+expense2+expense3 === 2020)
        }))
  })).map(
    expense => hasMatch(expense).foundMatch.length > 0 ? hasMatch(expense) : null).filter(expense => expense).map(e => e.expense* e.foundMatch[0].expense2 * e.foundMatch[0].foundMatch[0])
    logy(result);
}

const main = (data) => {
  let preparedData = prepareData(data);
  console.log(preparedData);
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