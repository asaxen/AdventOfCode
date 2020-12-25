const fs = require('fs')

const parsePassport = (passport) => {
  let re = /\n/gi;
  let passportAttributes = passport.replace(re, ' ').split(' ')
  
  let passportObject = {};
  passportAttributes.map(attribute => passportObject[attribute.split(':')[0]] = attribute.split(':')[1])
  return passportObject
}

const prepareData = (data) => {
  return data.split('\n\n').map(passport => parsePassport(passport))
}

const part1 = (data) => {
  validPassportKeys = ['eyr', 'iyr', 'ecl', 'pid', 'hgt', 'hcl', 'byr']
  let validPassports = data.filter(passport => validPassportKeys.filter(key => key in passport).length === validPassportKeys.length)

  console.log(data)
  console.log(`Valid passports: ${validPassports.length} Total number of passports: ${data.length}`)
}


const part2 = (data) => {
  validPassportKeys = {
    byr: (value) => parseInt(value) >= 1920 && parseInt(value) <= 2002 , 
    iyr: (value) => parseInt(value) >= 2010 && parseInt(value) <= 2020 , 
    eyr: (value) => parseInt(value) >= 2020 && parseInt(value) <= 2030 , 
    hgt: (value) => {
      let match = value.match(/^([0-9]{2,3})(in|cm)$/)
      let height = match ? parseInt(match[1]) : 0;
      return match ? match[2] === 'in' ? height >= 59 && height<= 76 : height >= 150 && height <= 193 : false
    },
    hcl: (value) => value.match(/^#[a-f0-9]{6}$/),
    ecl: (value) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].includes(value), 
    pid: (value) => value.match(/^[0-9]{9}$/)
  }
  console.log(Object.keys(validPassportKeys))

  let validPassports = data.filter(
    passport => Object.keys(validPassportKeys).filter(key => key in passport && validPassportKeys[key](passport[key])).length === Object.keys(validPassportKeys).length
  )



  console.log(validPassports)
  console.log(`Valid passports: ${validPassports.length} Total number of passports: ${data.length}`)
}

const main = (data) => {
  
  let preparedData = prepareData(data);
  part1(preparedData)
  part2(preparedData)
}

fs.readFile('input.txt', 'utf8' , (err, data) => {
  if (err) {
    console.error(err)
    return
  }
  main(data);
})